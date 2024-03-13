use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::{alpha1, char, digit1, multispace0, multispace1};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

use crate::types::{Frequency, Habit, HabitType};

fn identifier(input: &str) -> IResult<&str, String> {
    map(
        tuple((
            alpha1,
            terminated(many0(alt((alpha1, digit1, tag("-")))), char(':')),
        )),
        |(first, rest): (&str, Vec<&str>)| {
            let mut identifier = first.to_owned();
            for part in rest {
                identifier.push_str(part);
            }
            identifier
        },
    )(input)
}

fn habit_type(input: &str) -> IResult<&str, HabitType> {
    alt((
        map(tag("counter"), |_| HabitType::Counter),
        map(tag("done"), |_| HabitType::Done),
        map(tag("time"), |_| HabitType::Time),
        map(tag("abstain"), |_| HabitType::Abstain),
    ))(input)
}

fn frequency(input: &str) -> IResult<&str, Frequency> {
    alt((
        map(tag("daily"), |_| Frequency::Daily),
        map(tag("weekly"), |_| Frequency::Weekly),
        map(tag("monthly"), |_| Frequency::Monthly),
        map(tag("yearly"), |_| Frequency::Yearly),
    ))(input)
}

fn field(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        multispace1,
        tuple((
            alt((
                tag("goal"),
                tag("unit"),
                tag("type"),
                tag("frequency"),
                tag("start"),
                tag("end"),
            )),
            preceded(
                tuple((tag(":"), multispace0)),
                take_while_m_n(1, 20, |c: char| !c.is_whitespace()),
            ),
        )),
        opt(char('\n')),
    )(input)
}

fn habit_configuration(input: &str) -> IResult<&str, Habit> {
    let (input, name) = identifier(input)?;
    let (input, goal) = field(input)?;
    let (input, unit) = field(input)?;
    let (input, habit_type) = field(input)?;
    let (input, frequency) = field(input)?;
    let (input, start) = field(input)?;
    let (input, end) = field(input)?;

    Ok((
        input,
        Habit {
            name,
            goal: goal.1.parse().ok(),
            unit: Some(unit.1.to_string()),
            habit_type: habit_type.1.parse().unwrap(),
            frequency: frequency.1.parse().unwrap(),
            start: start.1.parse().ok(),
            end: end.1.parse().ok(),
        },
    ))
}

pub fn configuration(input: &str) -> IResult<&str, Vec<Habit>> {
    many0(delimited(multispace0, habit_configuration, multispace0))(input)
}

#[cfg(test)]
mod tests {
    use nom::error;
    use nom::Err;

    use super::*;

    #[test]
    fn test_parse_identifier() {
        assert_eq!(identifier("running:"), Ok(("", "running".to_string())));
        assert_eq!(identifier("running-1:"), Ok(("", "running-1".to_string())));
        assert_eq!(
            identifier("running-1: somethingh somthign"),
            Ok((" somethingh somthign", "running-1".to_string()))
        );
        assert_eq!(
            identifier("running_2"),
            Err(Err::Error(error::Error {
                input: "_2",
                code: error::ErrorKind::Char
            }))
        );
    }

    #[test]
    fn test_parse_habit_type() {
        assert_eq!(habit_type("counter"), Ok(("", HabitType::Counter)));
        assert_eq!(habit_type("done"), Ok(("", HabitType::Done)));
        assert_eq!(habit_type("time"), Ok(("", HabitType::Time)));
        assert_eq!(habit_type("abstain"), Ok(("", HabitType::Abstain)));
        assert_eq!(
            habit_type("asstain "),
            Err(Err::Error(error::Error {
                input: "asstain ",
                code: error::ErrorKind::Tag
            }))
        );
    }

    #[test]
    fn test_parse_frequency() {
        assert_eq!(frequency("daily"), Ok(("", Frequency::Daily)));
        assert_eq!(frequency("weekly"), Ok(("", Frequency::Weekly)));
        assert_eq!(frequency("monthly"), Ok(("", Frequency::Monthly)));
        assert_eq!(frequency("yearly"), Ok(("", Frequency::Yearly)));
        assert_eq!(
            frequency("yeerly "),
            Err(Err::Error(error::Error {
                input: "yeerly ",
                code: error::ErrorKind::Tag
            }))
        );
    }

    #[test]
    fn test_parse_field() {
        assert_eq!(field(" goal:  10\n"), Ok(("", ("goal", "10"))));
        assert_eq!(field("  unit: kg\n"), Ok(("", ("unit", "kg"))));
        assert_eq!(field("   type: counter\n"), Ok(("", ("type", "counter"))));
        assert_eq!(
            field(" frequency: daily\n"),
            Ok(("", ("frequency", "daily")))
        );
        assert_eq!(
            field("     start: 2020-01-01\n"),
            Ok(("", ("start", "2020-01-01")))
        );
        assert_eq!(
            field("  end: 2020-12-31\n"),
            Ok(("", ("end", "2020-12-31")))
        );
    }

    #[test]
    fn test_habit_configuration() {
        assert_eq!(
            habit_configuration(
                "running:\n  goal: 10\n  unit: km\n  type: counter\n  frequency: daily\n  start: 2020-01-01\n  end: 2020-12-31\n"
            ),
            Ok((
                "",
                Habit {
                    name: "running".to_string(),
                    goal: Some(10),
                    unit: Some("km".to_string()),
                    habit_type: HabitType::Counter,
                    frequency: Frequency::Daily,
                    start: Some("2020-01-01".parse::<crate::types::Date>().unwrap()),
                    end: Some("2020-12-31".parse::<crate::types::Date>().unwrap()),
                }
            ))
        );
    }

    #[test]
    fn test_configuration() {
        assert_eq!(
            configuration(
                "running:\n  goal: 10\n  unit: km\n  type: counter\n  frequency: daily\n  start: 2020-01-01\n  end: 2020-12-31\n"
            ),
            Ok((
                "",
                vec![Habit {
                    name: "running".to_string(),
                    goal: Some(10),
                    unit: Some("km".to_string()),
                    habit_type: HabitType::Counter,
                    frequency: Frequency::Daily,
                    start: Some("2020-01-01".parse::<crate::types::Date>().unwrap()),
                    end: Some("2020-12-31".parse::<crate::types::Date>().unwrap()),
                }]
            ))
        );
    }
}
