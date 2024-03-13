use core::fmt;
use std::{collections::HashMap, str::FromStr};

use crate::error::DateParseError;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

impl FromStr for Date {
    type Err = DateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return Err(DateParseError::InvalidFormat);
        }
        let year = parts[0].parse().map_err(|_| DateParseError::InvalidYear)?;
        let month = parts[1].parse().map_err(|_| DateParseError::InvalidMonth)?;
        let day = parts[2].parse().map_err(|_| DateParseError::InvalidDay)?;

        if month < 1 || month > 12 {
            return Err(DateParseError::InvalidMonth);
        }

        if day < 1 || day > 31 {
            return Err(DateParseError::InvalidDay);
        }
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
    pub start: Option<Date>,
    pub end: Option<Date>,
}

impl fmt::Display for Habit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Habit: {}\nType: {:?}\nFrequency: {:?}",
            self.name, self.habit_type, self.frequency
        )?;

        if let Some(goal) = self.goal {
            write!(f, "\nGoal: {}", goal)?;
        }
        if let Some(unit) = &self.unit {
            write!(f, "\nUnit: {}", unit)?;
        }
        if let Some(start) = &self.start {
            write!(f, "\nStart: {}", start)?;
        }
        if let Some(end) = &self.end {
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

#[derive(Debug, PartialEq)]
struct HabitLog {
    habit_id: u32,
    log_date: Date,
    progress: u32,
    note: Option<String>,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_date() {
        use super::Date;
        assert_eq!(
            "2021-01-01".parse::<Date>(),
            Ok(Date {
                year: 2021,
                month: 1,
                day: 1
            })
        );
        assert_eq!(
            "2021-12-31".parse::<Date>(),
            Ok(Date {
                year: 2021,
                month: 12,
                day: 31
            })
        );
        assert!("2021-12-32".parse::<Date>().is_err());
        assert!("2021-13-31".parse::<Date>().is_err());
        assert!("2021-12-31-".parse::<Date>().is_err());
    }

    #[test]
    fn test_parse_habit_type() {
        use super::HabitType;
        assert_eq!("counter".parse::<HabitType>(), Ok(HabitType::Counter));
        assert_eq!("done".parse::<HabitType>(), Ok(HabitType::Done));
        assert_eq!("time".parse::<HabitType>(), Ok(HabitType::Time));
        assert_eq!("abstain".parse::<HabitType>(), Ok(HabitType::Abstain));
        assert!("counter ".parse::<HabitType>().is_err());
    }

    #[test]
    fn test_parse_frequency() {
        use super::Frequency;
        assert_eq!("daily".parse::<Frequency>(), Ok(Frequency::Daily));
        assert_eq!("weekly".parse::<Frequency>(), Ok(Frequency::Weekly));
        assert_eq!("monthly".parse::<Frequency>(), Ok(Frequency::Monthly));
        assert_eq!("yearly".parse::<Frequency>(), Ok(Frequency::Yearly));
        assert!("daily ".parse::<Frequency>().is_err());
    }

    #[test]
    fn test_display_date() {
        use super::Date;
        assert_eq!(
            format!(
                "{}",
                Date {
                    year: 2021,
                    month: 1,
                    day: 1
                }
            ),
            "2021-1-1"
        );
        assert_eq!(
            format!(
                "{}",
                Date {
                    year: 2021,
                    month: 12,
                    day: 31
                }
            ),
            "2021-12-31"
        );
    }

    #[test]
    fn test_display_habit_type() {
        use super::HabitType;
        assert_eq!(format!("{}", HabitType::Counter), "Counter");
        assert_eq!(format!("{}", HabitType::Done), "Done");
        assert_eq!(format!("{}", HabitType::Time), "Time");
        assert_eq!(format!("{}", HabitType::Abstain), "Abstain");
    }

    #[test]
    fn test_display_frequency() {
        use super::Frequency;
        assert_eq!(format!("{}", Frequency::Daily), "Daily");
        assert_eq!(format!("{}", Frequency::Weekly), "Weekly");
        assert_eq!(format!("{}", Frequency::Monthly), "Monthly");
        assert_eq!(format!("{}", Frequency::Yearly), "Yearly");
    }
}
