#![allow(non_snake_case)]
use std::path::PathBuf;

use crate::utils::files::FileishExt;

pub const BASE_DIR: &str = ".discrepancy_resolver";
pub const DATA: &str = "data.json";
pub const FIX_DATA: &str = "fix.md";

pub fn DATA_PATH() -> PathBuf {
    let mut home = home::home_dir().unwrap();
    home.push(BASE_DIR);
    home.create_all_dirs().unwrap();
    home.push(DATA);

    home
}

pub fn FIX_PATH() -> PathBuf {
    let mut home = home::home_dir().unwrap();
    home.push(BASE_DIR);
    home.create_all_dirs().unwrap();
    home.push(FIX_DATA);

    home
}


pub const RG_EXE: &str = "rg";
pub const MINE_REGEX: &str = r#"\| \[([A-Z])-[0-9]+\]\(#[A-Z]-[0-9]+\) \| \[(.*?)\]\(#[A-Z]-[0-9]+\) \| ([0-9]+) \|"#;
pub const MINE_MASK: &str = r#"{"severity": "$1", "name": "$2", "instances": $3}"#;

// henry
pub const HENRY_SELECTOR: &str = "henry";
pub const HENRY_REGEX: &str = r#".*?\[\[([A-Z])&#x2011;[0-9]+\]\]\(#[a-z][0-9]+?.*?\) \| ~?~?(.*?)~?~? \| ([0-9]+) \|.*"#;
pub const HENRY_MASK: &str = r#"{"severity": "$1", "name": "$2", "instances": $3}"#;