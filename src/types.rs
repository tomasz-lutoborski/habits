use core::fmt;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

impl FromStr for Date {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return Err(());
        }
        let year = parts[0].parse().map_err(|_| ())?;
        let month = parts[1].parse().map_err(|_| ())?;
        let day = parts[2].parse().map_err(|_| ())?;
        Ok(Date { year, month, day })
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

#[derive(Debug, PartialEq)]
pub struct Habit {
    pub name: String,
    pub goal: Option<u32>,
    pub unit: Option<String>,
    pub habit_type: HabitType,
    pub frequency: Frequency,
    pub start: Option<Date>, // Using String for simplicity; consider using a date type
    pub end: Option<Date>,   // Using String for simplicity; consider using a date type
}

impl fmt::Display for Habit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Always print the name, type, and frequency as they are not optional
        write!(
            f,
            "Habit: {}\nType: {:?}\nFrequency: {:?}",
            self.name, self.habit_type, self.frequency
        )?;

        // Conditionally print optional fields if they have Some value
        if let Some(goal) = self.goal {
            write!(f, "\nGoal: {}", goal)?;
        }
        if let Some(unit) = &self.unit {
            write!(f, "\nUnit: {}", unit)?;
        }
        if let Some(start) = &self.start {
            // Assuming Date has a Display implementation; adjust accordingly
            write!(f, "\nStart: {}", start)?;
        }
        if let Some(end) = &self.end {
            // Assuming Date has a Display implementation; adjust accordingly
            write!(f, "\nEnd: {}", end)?;
        }

        write!(f, "\n")?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Configuration {
    pub habits: Vec<Habit>,
}

#[derive(Debug, PartialEq)]
pub enum HabitType {
    Counter,
    Done,
    Time,
    Abstain,
}

impl FromStr for HabitType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "counter" => Ok(HabitType::Counter),
            "done" => Ok(HabitType::Done),
            "time" => Ok(HabitType::Time),
            "abstain" => Ok(HabitType::Abstain),
            _ => Err(()),
        }
    }
}

impl fmt::Display for HabitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HabitType::Counter => write!(f, "Counter"),
            HabitType::Done => write!(f, "Done"),
            HabitType::Time => write!(f, "Time"),
            HabitType::Abstain => write!(f, "Abstain"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl FromStr for Frequency {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "daily" => Ok(Frequency::Daily),
            "weekly" => Ok(Frequency::Weekly),
            "monthly" => Ok(Frequency::Monthly),
            "yearly" => Ok(Frequency::Yearly),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Frequency::Daily => write!(f, "Daily"),
            Frequency::Weekly => write!(f, "Weekly"),
            Frequency::Monthly => write!(f, "Monthly"),
            Frequency::Yearly => write!(f, "Yearly"),
        }
    }
}
