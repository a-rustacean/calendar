use std::io::Write;
use colored::Colorize;

use chrono::{offset::Local, Datelike, NaiveDate, Weekday, Duration};

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

fn repeat(string: impl Into<String>, times: usize) -> String {
    let string = string.into();
    let mut output = String::new();
    for _ in 0..times {
        output = format!("{}{}", output, string);
    }
    output
}

fn pad(string: impl Into<String>, padding: usize) -> String {
    let string = string.into();
    let padding_left = ((padding - string.len()) as f32 / 2.0).ceil() as usize;
    format!(
        "{}{}{}",
        repeat(" ", padding_left),
        string,
        repeat(" ", padding - padding_left - string.len())
    )
}

fn month_to_string(month: u32) -> String {
    MONTHS[month as usize].to_string()
}

pub fn get_days_from_month(year: i32, month: u32) -> Option<i64> {
    Some(NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )?
    .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1)?)
    .num_days())
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let current_date = Local::now().date_naive();
    println!(
        "{}",
        pad(
            format!(
                "{} {}",
                month_to_string(current_date.month0()),
                current_date.year()
            ),
            20
        )
    );
    let month_start = current_date - Duration::days(current_date.day0() as i64);
    println!("Su Mo Tu We Th Fr Sa");
    let from_sunday = month_start.weekday().number_from_sunday() - 1;
    print!("{} ", repeat(" ", 2 * from_sunday as usize + (from_sunday.max(1) as usize - 1)));
    for day in 0..get_days_from_month(current_date.year(), current_date.month()).unwrap() {
        if day as u32 == current_date.day0() {
            print!("{} ", pad((day + 1).to_string(), 2).black().on_white());
        } else {
            print!("{} ", pad((day + 1).to_string(), 2)); 
        }
        std::io::stdout().flush().unwrap();
        if (month_start + Duration::days(day)).weekday() == Weekday::Sat {
            println!();
        }
    };
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_pad() {
        assert_eq!(pad("1", 2), " 1".to_string());
        assert_eq!(pad("10", 2), "10".to_string());
    }
}
