use std::{
    collections::HashMap,
    fmt::Write,
    io::Write as WriteIo,
    sync::{Arc, Mutex, MutexGuard},
};

use getters0::Getters;
use serde::{Deserialize, Serialize};

use crate::{
    constants::{DATA_PATH, FIX_PATH},
    prompt,
    utils::files::FileishExt,
};

#[derive(Debug, Serialize, Deserialize, Getters, Default)]
pub struct Database {
    #[get_mut]
    inner: HashMap<String, Vec<String>>,
    #[serde(skip)]
    fix_list: Vec<(String, FixKey)>,
    #[get_mut]
    checkpoint: usize,
}

impl Database {
    pub fn open() -> Self {
        match DATA_PATH().try_readable() {
            Ok(mut data_file) => match serde_json::from_reader::<_, Database>(&mut data_file) {
                Ok(mut db) => {
                    if *db.checkpoint() != 0 {
                        prompt!(
                            "Checkpoint was found in database, skip {} first issues in target report (y), or set checkpoint to zero(n)\n > ", *db.checkpoint();
                            {}
                            {*db.checkpoint_mut() = 0;}
                        )
                    }
                    db
                }
                Err(err) => unimplemented!("{}", err),
            },
            Err(_) => Database::default(),
        }
    }

    pub fn store(&self) {
        match DATA_PATH().try_truncatable() {
            Ok(mut data_file) => match serde_json::to_writer_pretty(&mut data_file, self) {
                Ok(db) => db,
                Err(err) => unimplemented!("{}", err),
            },
            Err(err) => unimplemented!("{}", err),
        }
        match FIX_PATH().try_appendable() {
            Ok(mut fix_file) => {
                let data = self.fix_list().iter().enumerate().fold(
                    String::new(),
                    |mut output, (i, (item, key))| {
                        let _ = writeln!(output, "{}. {} \n\t[fix key: {:?}]\n", i, item, key);
                        output
                    },
                );
                fix_file.write_all(data.as_bytes()).unwrap();
            }
            Err(_) => todo!(),
        }
    }

    pub fn has_issue(&self, issue: &str) -> bool {
        self.inner
            .values()
            .any(|v| v.iter().any(|value| value == issue))
    }

    pub fn get_mine_from_target(&self, issue: &str) -> String {
        self.inner()
            .iter()
            .find_map(|(k, v)| v.iter().any(|value| value == issue).then_some(k))
            .unwrap()
            .to_owned()
    }

    pub fn add_fix_list<S: Into<String>>(&mut self, issue: S, key: FixKey) {
        self.fix_list.push((issue.into(), key));
    }

    pub fn update_link(&mut self, mine: &str, target: &str) {
        if self.inner.contains_key(mine) {
            self.inner.get_mut(mine).unwrap().push(target.to_owned());
        } else {
            self.inner.insert(mine.to_owned(), vec![target.to_owned()]);
        }
    }

    pub fn set_checkpoint(&mut self, checkpoint: usize) {
        self.checkpoint = checkpoint;
    }

    pub fn increment(&mut self) {
        self.checkpoint += 1;
    }
}

pub trait DatabaseExt {
    fn callback<F>(&self, c: F) -> Result<(), std::sync::PoisonError<MutexGuard<Database>>>
    where
        F: FnOnce(&Database);

    fn callback_mut<F>(&self, c: F) -> Result<(), std::sync::PoisonError<MutexGuard<Database>>>
    where
        F: FnOnce(&mut Database);

    fn callback_arg<F, A, O>(
        &self,
        c: F,
        arg: A,
    ) -> Result<O, std::sync::PoisonError<MutexGuard<Database>>>
    where
        F: FnOnce(&Database, A) -> O;

    fn callback_arg2<F, A1, A2, O>(
        &self,
        c: F,
        arg: A1,
        arg2: A2,
    ) -> Result<O, std::sync::PoisonError<MutexGuard<Database>>>
    where
        F: FnOnce(&Database, A1, A2) -> O;

    fn callback_arg_mut<F, A, O>(
        &self,
        c: F,
        arg: A,
    ) -> Result<O, std::sync::PoisonError<MutexGuard<Database>>>
    where
        F: FnOnce(&mut Database, A) -> O;

    fn callback_arg2_mut<F, A1, A2, O>(
        &self,
        c: F,
        arg: A1,
        arg2: A2,
    ) -> Result<O, std::sync::PoisonError<MutexGuard<Database>>>
    where
        F: FnOnce(&mut Database, A1, A2) -> O;
}

impl DatabaseExt for Arc<Mutex<Database>> {
    fn callback<F>(&self, c: F) -> Result<(), std::sync::PoisonError<MutexGuard<Database>>>
    where
        F: FnOnce(&Database),
    {
        let db = self.lock()?;
        c(&db);
        Ok(())
    }

    fn callback_mut<F>(&self, c: F) -> Result<(), std::sync::PoisonError<MutexGuard<Database>>>
    where
        F: FnOnce(&mut Database),
    {
        let mut db = self.lock()?;
        c(&mut db);
        Ok(())
    }

    fn callback_arg<F, A, O>(
        &self,
        c: F,
        arg: A,
    ) -> Result<O, std::sync::PoisonError<MutexGuard<Database>>>
    where
        F: FnOnce(&Database, A) -> O,
    {
        let db = self.lock()?;
        Ok(c(&db, arg))
    }

    fn callback_arg2<F, A1, A2, O>(
        &self,
        c: F,
        arg: A1,
        arg2: A2,
    ) -> Result<O, std::sync::PoisonError<MutexGuard<Database>>>
    where
        F: FnOnce(&Database, A1, A2) -> O,
    {
        let db = self.lock()?;
        Ok(c(&db, arg, arg2))
    }

    fn callback_arg_mut<F, A, O>(
        &self,
        c: F,
        arg: A,
    ) -> Result<O, std::sync::PoisonError<MutexGuard<Database>>>
    where
        F: FnOnce(&mut Database, A) -> O,
    {
        let mut db = self.lock()?;
        Ok(c(&mut db, arg))
    }

    fn callback_arg2_mut<F, A1, A2, O>(
        &self,
        c: F,
        arg: A1,
        arg2: A2,
    ) -> Result<O, std::sync::PoisonError<MutexGuard<Database>>>
    where
        F: FnOnce(&mut Database, A1, A2) -> O,
    {
        let mut db = self.lock()?;
        Ok(c(&mut db, arg, arg2))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FixKey {
    AddIssue(String),
    MissingInstances(String),
    ChangeSeverity(String),
}
