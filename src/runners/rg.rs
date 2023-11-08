use std::process::Command;

use crate::constants::RG_EXE;

pub fn run_rg(file: &str, regex: &str, mask: &str) -> Vec<u8> {
    Command::new(RG_EXE)
        .arg(regex)
        .arg(file)
        .arg("-r")
        .arg(mask)
        .output()
        .unwrap()
        .stdout
}
