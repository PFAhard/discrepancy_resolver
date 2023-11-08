use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use clap::Parser;
use config::Config;
use database::Database;

use crate::database::{DatabaseExt, FixKey};

pub mod config;
pub mod constants;
pub mod database;
pub mod runners;
pub mod utils;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let database = Arc::new(Mutex::new(Database::open()));
    let db_for_ctrlc = database.clone();
    let config = Config::parse();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("Ctrl-C received! Exiting...");

        // Lock the database for exclusive access
        db_for_ctrlc.callback(Database::store).unwrap();

        let db = db_for_ctrlc.lock().unwrap();
        db.store();
        drop(db); // Explicitly drop the lock before exiting

        std::process::exit(0); // Exit the process
    })
    .expect("Error setting Ctrl-C handler");

    let (mine, mut target) = (config.get_mine_issues(), config.get_target_issues());
    target.reverse();
    let mut index = 0;

    while let Some(issue) = target.pop() {
        index += 1;
        if database
            .callback_arg(Database::has_issue, issue.name())
            .unwrap()
        {
            let mine_issue = database
                .callback_arg(Database::get_mine_from_target, issue.name())
                .unwrap()
                .to_owned();
            if mine.iter().any(|value| value.name() == &mine_issue) {
                let mine_issue = mine
                    .iter()
                    .find(|value| value.name() == &mine_issue)
                    .unwrap();
                prompt!(
                    "Mine reports has Target issue. Do you want to see differences?\n > ";
                    {
                        if issue.name() != mine_issue.name() {
                            println!("Index: {}\n\tName Different: Mine: `{}`, Target: `{}`", index, mine_issue.name(), issue.name());
                        } else {
                            println!("Index: {}\n\tIssue: `{}`", index, mine_issue.name());
                        }
                        if issue.severity() != mine_issue.severity() {
                            println!("\tSeverity Different: Mine: `{}`, Target: `{}`", mine_issue.severity(), issue.severity());
                        }
                        if issue.instances() != mine_issue.instances() {
                            println!("\tInstances Different: Mine: {}, Target: {}", mine_issue.instances(), issue.instances());
                        }
                        prompt!(
                            "Do you want to add this to fix list?\n > ";
                            {
                                let comment = uni_prompt!("Print comment for this fix:\n > ");
                                database.callback_arg2_mut(Database::add_fix_list, mine_issue.name(), FixKey::MissingInstances(comment)).unwrap();
                            }
                            {}
                        );
                    }
                    {}
                )
            } else {
                println!(
                    "Index: {}\n\tMine reports missing the issue: \n\t{:#?}",
                    index, mine_issue
                );
                let comment = uni_prompt!("Print comment for this fix:\n > ");
                database
                    .callback_arg2_mut(Database::add_fix_list, mine_issue, FixKey::MissingInstances(comment))
                    .unwrap();
            }
        } else {
            prompt!(
                "Target issue `{}` not found in database. Do you want to link one?\n > ",
                issue.name();
                {
                    let mine = uni_prompt!("Type in your name of this issue\n > ");
                    database.callback_arg2_mut(Database::update_link, &mine, issue.name()).unwrap();
                    prompt!(
                        "Do you want to add this to fix list?\n > ";
                        {
                            let comment = uni_prompt!("Print comment for this fix:\n > ");
                            database.callback_arg2_mut(Database::add_fix_list, mine, FixKey::MissingInstances(comment)).unwrap();
                        }
                        {}
                    );
                }
                {
                    database.callback_arg2_mut(Database::update_link, issue.name(), issue.name()).unwrap();
                    let comment = uni_prompt!("Print comment for this fix:\n > ");
                    database.callback_arg2_mut(Database::add_fix_list, issue.name(), FixKey::AddIssue(comment)).unwrap();
                }
            );
        }
        println!();
    }
}
