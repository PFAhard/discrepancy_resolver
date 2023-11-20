use clap::Parser;
use getters0::Getters;
use serde::Deserialize;

use crate::{
    constants::{
        HENRY_MASK, HENRY_REGEX, HENRY_SELECTOR, HOUND_MASK, HOUND_REGEX, HOUND_SELECTOR,
        MINE_MASK, MINE_REGEX,
    },
    error_handler,
    runners::rg::run_rg,
};

#[derive(Debug, Parser, Getters, Default)]
pub struct Config {
    #[use_deref]
    #[arg(long = "mrp")]
    mine_report: String,
    #[use_deref]
    #[arg(long = "trp")]
    target_report: String,
    #[use_deref]
    #[arg(short, long = "tsl")]
    target_selector: String,
}

impl Config {
    pub fn get_mine_issues(&self) -> Vec<Issue> {
        get_issues(self.mine_report(), MINE_REGEX, MINE_MASK)
    }

    pub fn get_target_issues(&self) -> Vec<Issue> {
        let (mrg, mmk) = if self.target_selector() == HENRY_SELECTOR {
            (HENRY_REGEX, HENRY_MASK)
        } else if self.target_selector() == HOUND_SELECTOR {
            (HOUND_REGEX, HOUND_MASK)
        } else {
            todo!()
        };
        get_issues(self.target_report(), mrg, mmk)
    }
}

pub fn get_issues(report: &str, rg: &str, mask: &str) -> Vec<Issue> {
    run_rg(report, rg, mask)
        .into_iter()
        .map(|c| c as char)
        .collect::<String>()
        .lines()
        .map(error_handler::from_str::<Issue>)
        .collect()
}

#[derive(Debug, Getters, Deserialize)]
pub struct Issue {
    severity: String,
    name: String,
    instances: usize,
    description: Option<String>,
}
