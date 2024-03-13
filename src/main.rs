#![allow(dead_code)]
#![allow(unused_imports)]
use std::fs::read_to_string;

use anyhow::Result;
use db::init_db;
use rusqlite::Connection;

use crate::cli::run_cli;
use crate::db::test_db;

mod cli;
mod db;
mod error;
mod parser;
mod types;
mod ui;

fn main() -> Result<()> {
    let conn = Connection::open("habits.db").unwrap();
    init_db(&conn).unwrap();

    let habits = parser::configuration(&read_to_string("sample-config.habits").unwrap())
        .unwrap()
        .1;

    for habit in habits {
        db::insert_habit(&conn, &habit).unwrap();
    }

    test_db().unwrap();

    Ok(())
}
