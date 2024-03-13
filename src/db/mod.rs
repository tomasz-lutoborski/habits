use rusqlite::{params, Connection, Result};

use crate::types::{Date, Frequency, Habit, HabitType};

pub fn init_db() -> Result<()> {
    let conn = Connection::open("habits.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS habits (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            goal INTEGER,
            unit TEXT,
            habit_type TEXT NOT NULL,
            frequency TEXT NOT NULL,
            start_date TEXT,
            end_date TEXT
        )",
        [],
    )?;

    Ok(())
}

pub fn insert_habit(conn: &Connection, habit: &Habit) -> Result<()> {
    conn.execute(
        "INSERT INTO habits (name, goal, unit, habit_type, frequency, start_date, end_date) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            habit.name,
            habit.goal,
            habit.unit,
            habit.habit_type.to_string(), // Assuming you implement ToString for HabitType
            habit.frequency.to_string(),  // Assuming you implement ToString for Frequency
            habit.start.map(|d| d.to_string()), // Assuming you have a Date type that implements ToString
            habit.end.map(|d| d.to_string()),
        ],
    )?;
    Ok(())
}

pub fn test_db() -> Result<()> {
    // Initialize the database and create the table if it doesn't exist
    init_db()?;

    // Open a connection to the database
    let conn = Connection::open("habits.db")?;

    // Example: Insert a habit after parsing
    // Assuming you have a parsed habit to insert
    let habit = Habit {
        name: "Reading".to_string(),
        goal: Some(20),
        unit: Some("pages".to_string()),
        habit_type: HabitType::Counter, // Example enum value
        frequency: Frequency::Daily,    // Example enum value
        start: Some(Date {
            year: 2021,
            month: 1,
            day: 1,
        }),
        end: Some(Date {
            year: 2021,
            month: 12,
            day: 31,
        }),
    };

    insert_habit(&conn, &habit)?;

    Ok(())
}
