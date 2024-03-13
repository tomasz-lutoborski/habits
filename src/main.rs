#![allow(dead_code)]
#![allow(unused_imports)]
use std::fs::read_to_string;

use crate::cli::run_cli;
use crate::db::test_db;

mod cli;
mod db;
mod parser;
mod types;
mod ui;

fn main() {
    run_cli();
    test_db().unwrap();
}
