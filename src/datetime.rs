use chrono::{Datelike, Local, Timelike};
use std::str::FromStr;

use crate::validator::to_english;

#[derive(Debug, PartialEq)]
pub enum DateTimeError {
    InvalidYear(String),
    InvalidMonth(String),
    InvalidDay(String),
    InvalidHour(String),
    InvalidMinute(String),
    InvalidSecond(String),
    InvalidDate(String),
    InvalidDateTime(String),
}

impl std::fmt::Display for DateTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateTimeError::InvalidYear(msg) => write!(f, "Invalid Year: {}", msg),
            DateTimeError::InvalidMonth(msg) => write!(f, "Invalid Month: {}", msg),
            DateTimeError::InvalidDay(msg) => write!(f, "Invalid Day: {}", msg),
            DateTimeError::InvalidHour(msg) => write!(f, "Invalid Hour: {}", msg),
            DateTimeError::InvalidMinute(msg) => write!(f, "Invalid Minute: {}", msg),
            DateTimeError::InvalidSecond(msg) => write!(f, "Invalid Second: {}", msg),
            DateTimeError::InvalidDate(msg) => write!(f, "Invalid Date: {}", msg),
            DateTimeError::InvalidDateTime(msg) => write!(f, "Invalid DateTime: {}", msg),
        }
    }
}

impl std::error::Error for DateTimeError {}

pub trait DateTimeValidator {
    // Validate year: raw string must be 1-4 digits and 1-9999
    fn validate_year(year: &str) -> Result<(), DateTimeError> {
        let year = to_english(year);
        if year.is_empty() {
            return Err(DateTimeError::InvalidYear(
                "year can not be empty!".to_string(),
            ));
        }
        if !year.chars().all(|c| c.is_ascii_digit()) {
            return Err(DateTimeError::InvalidYear(format!(
                "'{}' contains non-digit characters!",
                year
            )));
        }
        let year_num: i32 = year
            .parse()
            .map_err(|_| DateTimeError::InvalidYear(format!("'{}' is not a valid number", year)))?;
        if !(1..=9999).contains(&year_num) {
            return Err(DateTimeError::InvalidYear(format!(
                "year must be between 1-9999, got {}",
                year_num
            )));
        }
        Ok(())
    }

    // Validate month: raw string must be 2 digits (01-12)
    fn validate_month(month: &str) -> Result<(), DateTimeError> {
        let month = to_english(month);
        if month.is_empty() {
            return Err(DateTimeError::InvalidMonth(
                "month can not be empty!".to_string(),
            ));
        }
        if !month.chars().all(|c| c.is_ascii_digit()) {
            return Err(DateTimeError::InvalidMonth(format!(
                "'{}' contains non-digit characters!",
                month
            )));
        }
        if month.len() != 2 {
            return Err(DateTimeError::InvalidMonth(format!(
                "month must be 2 digits (e.g., '01'), got '{}'",
                month
            )));
        }
        let month_num: u32 = month.parse().map_err(|_| {
            DateTimeError::InvalidMonth(format!("'{}' is not a valid number", month))
        })?;
        if !(1..=12).contains(&month_num) {
            return Err(DateTimeError::InvalidMonth(format!(
                "month must be between 01-12, got {}",
                month
            )));
        }
        Ok(())
    }

    // Validate day: raw string must be 2 digits (01-31)
    fn validate_day(day: &str) -> Result<(), DateTimeError> {
        let day = to_english(day);

        if day.is_empty() {
            return Err(DateTimeError::InvalidDay(
                "day can not be empty!".to_string(),
            ));
        }
        if !day.chars().all(|c| c.is_ascii_digit()) {
            return Err(DateTimeError::InvalidDay(format!(
                "'{}' contains non-digit characters!",
                day
            )));
        }
        if day.len() != 2 {
            return Err(DateTimeError::InvalidDay(format!(
                "day must be 2 digits (e.g., '01'), got '{}'",
                day
            )));
        }
        let day_num: u32 = day
            .parse()
            .map_err(|_| DateTimeError::InvalidDay(format!("'{}' is not a valid number", day)))?;
        if !(1..=31).contains(&day_num) {
            return Err(DateTimeError::InvalidDay(format!(
                "day must be between 01-31, got {}",
                day
            )));
        }
        Ok(())
    }

    // Validate hour: raw string must be 2 digits (00-23)
    fn validate_hour(hour: &str) -> Result<(), DateTimeError> {
        let hour = to_english(hour);

        if hour.is_empty() {
            return Err(DateTimeError::InvalidHour(
                "hour can not be empty!".to_string(),
            ));
        }
        if !hour.chars().all(|c| c.is_ascii_digit()) {
            return Err(DateTimeError::InvalidHour(format!(
                "'{}' contains non-digit characters!",
                hour
            )));
        }
        // if hour.len() != 2 {
        //     return Err(DateTimeError::InvalidHour(format!(
        //         "hour must be 2 digits (e.g., '01'), got '{}'",
        //         hour
        //     )));
        // }
        let hour_num: u32 = hour
            .parse()
            .map_err(|_| DateTimeError::InvalidHour(format!("'{}' is not a valid number", hour)))?;
        if hour_num > 23 {
            return Err(DateTimeError::InvalidHour(format!(
                "hour must be between 00-23, got {}",
                hour
            )));
        }
        Ok(())
    }

    // Validate minute: raw string must be 2 digits (00-59)
    fn validate_minute(minute: &str) -> Result<(), DateTimeError> {
        let minute = to_english(minute);

        if minute.is_empty() {
            return Err(DateTimeError::InvalidMinute(
                "minute can not be empty!".to_string(),
            ));
        }
        if !minute.chars().all(|c| c.is_ascii_digit()) {
            return Err(DateTimeError::InvalidMinute(format!(
                "'{}' contains non-digit characters!",
                minute
            )));
        }
        if minute.len() != 2 {
            return Err(DateTimeError::InvalidMinute(format!(
                "minute must be 2 digits (e.g., '01'), got '{}'",
                minute
            )));
        }
        let minute_num: u32 = minute.parse().map_err(|_| {
            DateTimeError::InvalidMinute(format!("'{}' is not a valid number", minute))
        })?;
        if minute_num > 59 {
            return Err(DateTimeError::InvalidMinute(format!(
                "minute must be between 00-59, got {}",
                minute
            )));
        }
        Ok(())
    }

    // Validate second: raw string must be 2 digits (00-59)
    fn validate_second(second: &str) -> Result<(), DateTimeError> {
        let second = to_english(second);
        if second.is_empty() {
            return Err(DateTimeError::InvalidSecond(
                "second can not be empty!".to_string(),
            ));
        }
        if !second.chars().all(|c| c.is_ascii_digit()) {
            return Err(DateTimeError::InvalidSecond(format!(
                "'{}' contains non-digit characters!",
                second
            )));
        }
        if second.len() != 2 {
            return Err(DateTimeError::InvalidSecond(format!(
                "second must be 2 digits (e.g., '01'), got '{}'",
                second
            )));
        }
        let second_num: u32 = second.parse().map_err(|_| {
            DateTimeError::InvalidSecond(format!("'{}' is not a valid number", second))
        })?;
        if second_num > 59 {
            return Err(DateTimeError::InvalidSecond(format!(
                "second must be between 00-59, got {}",
                second
            )));
        }
        Ok(())
    }

    fn parse_validate_date(date: &str) -> Result<(String, String, String), DateTimeError> {
        let parts_of_date: Vec<&str> = date.split(['/', '-']).collect();
        if parts_of_date.len() != 3 {
            return Err(DateTimeError::InvalidDate(format!(
                "'{}' must be in format YYYY/MM/DD or YYYY-MM-DD",
                date
            )));
        }
        let [year, month, day]: [&str; 3] = parts_of_date
            .try_into()
            .map_err(|_| DateTimeError::InvalidDate("Conversion error".to_string()))?;

        Self::validate_year(year)?;
        Self::validate_month(month)?;
        Self::validate_day(day)?;

        // Store as English words (using to_english) – this is your design choice.
        Ok((to_english(year), to_english(month), to_english(day)))
    }

    fn parse_validate_time(date: &str) -> Result<(String, String, String), DateTimeError> {
        let parts_of_date: Vec<&str> = date.split(':').collect();
        if parts_of_date.len() != 3 {
            return Err(DateTimeError::InvalidDate(format!(
                "'{}' must be in format HH:MM:SS or H:MM:SS",
                date
            )));
        }
        let [hour, minute, second]: [&str; 3] = parts_of_date
            .try_into()
            .map_err(|_| DateTimeError::InvalidDate("Conversion error".to_string()))?;

        Self::validate_hour(hour)?;
        Self::validate_minute(minute)?;
        Self::validate_second(second)?;

        // Store as English words (using to_english) – this is your design choice.
        Ok((to_english(hour), to_english(minute), to_english(second)))
    }

    fn create(
        year: String,
        month: String,
        day: String,
        hour: Option<String>,
        minute: Option<String>,
        second: Option<String>,
    ) -> Self;

    fn parse_validate_datetime(datetime: &str) -> Result<Self, DateTimeError>
    where
        Self: Sized,
    {
        let parts_of_datetime: Vec<&str> = datetime.split_whitespace().collect();
        if parts_of_datetime.len() != 2 {
            return Err(DateTimeError::InvalidDate(format!(
                "'{}' must be in format YYYY/MM/DD HH:MM:SS or YYYY-MM-DD HH:MM:SS",
                datetime
            )));
        }

        let time: Vec<&str> = parts_of_datetime[1].split(':').collect();
        if time.len() != 3 {
            return Err(DateTimeError::InvalidDate(format!(
                "'{}' must be in format HH:MM:SS",
                parts_of_datetime[1]
            )));
        }
        let [hour, minute, second]: [&str; 3] = time
            .try_into()
            .map_err(|_| DateTimeError::InvalidDate("Conversion error".to_string()))?;

        Self::validate_hour(hour)?;
        Self::validate_minute(minute)?;
        Self::validate_second(second)?;

        let (year, month, day) = Self::parse_validate_date(parts_of_datetime[0])?;

        Ok(Self::create(
            year,
            month,
            day,
            Some(to_english(hour)),
            Some(to_english(minute)),
            Some(to_english(second)),
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct DateTime {
    year: String,
    month: String,
    day: String,
    hour: Option<String>,
    minute: Option<String>,
    second: Option<String>,
}

impl DateTimeValidator for DateTime {
    fn create(
        year: String,
        month: String,
        day: String,
        hour: Option<String>,
        minute: Option<String>,
        second: Option<String>,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
}

impl DateTime {
    pub fn new(year: i32, month: i8, day: i8) -> Self {
        Self {
            year: year.to_string(),
            month: month.to_string(),
            day: day.to_string(),
            hour: None,
            minute: None,
            second: None,
        }
    }
    pub fn set_date(date: &str) -> Result<Self, DateTimeError> {
        let (year, month, day) = Self::parse_validate_date(date)?;

        Ok(Self {
            year: year,
            month: month,
            day: day,
            hour: None,
            minute: None,
            second: None,
        })
    }

    pub fn set_datetime(datetime: &str) -> Result<Self, DateTimeError> {
        let datetime: DateTime = Self::parse_validate_datetime(datetime)?;
        Ok(datetime)
    }
    pub fn set_time(time: &str) -> Result<Self, DateTimeError> {
        let local_now = Local::now();
        let (hour, minute, second) = Self::parse_validate_time(time)?;
        Ok(Self {
            year: format!("{}", local_now.year()),
            month: format!("{}", local_now.month()),
            day: format!("{}", local_now.day()),
            hour: Some(to_english(&format!("{}", hour))),
            minute: Some(to_english(&format!("{}", minute))),
            second: Some(to_english(&format!("{}", second))),
        })
    }
    pub fn now() -> Self {
        let local_now = Local::now();

        Self {
            year: format!("{}", local_now.year()),
            month: format!("{}", local_now.month()),
            day: format!("{}", local_now.day()),
            hour: Some(format!("{}", local_now.hour())),
            minute: Some(format!("{}", local_now.minute())),
            second: Some(format!("{}", local_now.second())),
        }
    }
    pub fn year(&self) -> String {
        self.year.to_string()
    }
    pub fn month(&self) -> String {
        self.month.to_string()
    }
    // FIXED: previously returned month by mistake
    pub fn day(&self) -> String {
        self.day.to_string()
    }

    pub fn hour(&self) -> Option<String> {
        self.hour.as_ref().map(|s| to_english(s))
    }

    pub fn minute(&self) -> Option<String> {
        self.minute.as_ref().map(|f| to_english(f))
    }

    pub fn second(&self) -> Option<String> {
        self.second.as_ref().map(|f| to_english(f))
    }
}

#[derive(Debug, PartialEq)]
pub struct JDateTime {
    year: String,
    month: String,
    day: String,
    hour: Option<String>,
    minute: Option<String>,
    second: Option<String>,
}

impl DateTimeValidator for JDateTime {
    fn create(
        year: String,
        month: String,
        day: String,
        hour: Option<String>,
        minute: Option<String>,
        second: Option<String>,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
}

impl JDateTime {
    pub fn new(year: i32, month: i8, day: i8) -> Self {
        Self {
            year: year.to_string(),
            month: month.to_string(),
            day: day.to_string(),
            hour: None,
            minute: None,
            second: None,
        }
    }
    pub fn set_date(date: &str) -> Result<Self, DateTimeError> {
        let (year, month, day) = Self::parse_validate_date(date)?;
        Ok(Self {
            year: year.to_string(),
            month: month.to_string(),
            day: day.to_string(),
            hour: None,
            minute: None,
            second: None,
        })
    }
    pub fn set_datetime(datetime: &str) -> Result<Self, DateTimeError> {
        let datetime = Self::parse_validate_datetime(datetime)?;
        Ok(datetime)
    }

    pub fn set_time(hour: i8, minute: i8, second: i8) -> Result<Self, DateTimeError> {
        // Validate time ranges
        if !(0..=23).contains(&hour) {
            return Err(DateTimeError::InvalidHour(hour.to_string()));
        }
        if !(0..=59).contains(&minute) {
            return Err(DateTimeError::InvalidMinute(minute.to_string()));
        }
        if !(0..=59).contains(&second) {
            return Err(DateTimeError::InvalidSecond(second.to_string()));
        }

        let local_now = DateTime::now();
        let jalali_date = to_jalali(&local_now)?;
        Ok(Self {
            year: jalali_date.year,
            month: jalali_date.month,
            day: jalali_date.day,
            hour: Some(hour.to_string()),
            minute: Some(minute.to_string()),
            second: Some(second.to_string()),
        })
    }

    pub fn now() -> Result<Self, DateTimeError> {
        let local_now = DateTime::now();
        let jalali_date = to_jalali(&local_now)?;
        Ok(Self {
            year: jalali_date.year,
            month: jalali_date.month,
            day: jalali_date.day,
            hour: local_now.hour,
            minute: local_now.minute,
            second: local_now.second,
        })
    }

    pub fn year(&self) -> String {
        self.year.to_string()
    }
    pub fn month(&self) -> String {
        self.month.to_string()
    }
    // FIXED: previously returned month by mistake
    pub fn day(&self) -> String {
        self.day.to_string()
    }

    pub fn hour(&self) -> Option<String> {
        self.hour.as_ref().map(|s| to_english(s))
    }

    pub fn minute(&self) -> Option<String> {
        self.minute.as_ref().map(|f| to_english(f))
    }

    pub fn second(&self) -> Option<String> {
        self.second.as_ref().map(|f| to_english(f))
    }
}

// ------------------- Core calendar functions -------------------

fn parse_date_gregorian(dt: &DateTime) -> Result<(i64, u8, u8), DateTimeError> {
    let year = i64::from_str(&dt.year).map_err(|_| DateTimeError::InvalidYear(dt.year.clone()))?;
    let month =
        u8::from_str(&dt.month).map_err(|_| DateTimeError::InvalidMonth(dt.month.clone()))?;
    let day = u8::from_str(&dt.day).map_err(|_| DateTimeError::InvalidDay(dt.day.clone()))?;

    if !(1..=12).contains(&month) {
        return Err(DateTimeError::InvalidMonth(month.to_string()));
    }
    let max_days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => unreachable!(),
    };
    if day == 0 || day > max_days {
        return Err(DateTimeError::InvalidDay(day.to_string()));
    }
    Ok((year, month, day))
}

fn parse_date_jalali(jdt: &JDateTime) -> Result<(i64, u8, u8), DateTimeError> {
    let year =
        i64::from_str(&jdt.year).map_err(|_| DateTimeError::InvalidYear(jdt.year.clone()))?;
    let month =
        u8::from_str(&jdt.month).map_err(|_| DateTimeError::InvalidMonth(jdt.month.clone()))?;
    let day = u8::from_str(&jdt.day).map_err(|_| DateTimeError::InvalidDay(jdt.day.clone()))?;

    if !(1..=12).contains(&month) {
        return Err(DateTimeError::InvalidMonth(month.to_string()));
    }
    let max_days = if month <= 6 {
        31
    } else if month == 12 {
        if is_jalali_leap(year) { 29 } else { 30 }
    } else {
        30
    };
    if day == 0 || day > max_days {
        return Err(DateTimeError::InvalidDay(day.to_string()));
    }
    Ok((year, month, day))
}

// ========================
//  Persian (Jalali) calendar
// ========================

/// Check if a Jalali year is a leap year.
pub fn is_jalali_leap(year: i64) -> bool {
    let rem = (year + 33) % 33;
    matches!(rem, 1 | 5 | 9 | 13 | 17 | 22 | 26 | 30)
}

/// Number of leap years strictly before the given Jalali year.
fn leap_years_before(year: i64) -> i64 {
    if year <= 1 {
        return 0;
    }
    let y = year - 1; // complete years before `year`
    let cycles = y / 33; // full 33‑year cycles
    let rem = y % 33; // years left in the current cycle

    // Leap positions in a 33‑year cycle (1‑indexed)
    let leaps_in_partial = match rem {
        0 => 0,
        1..=4 => 1,
        5..=8 => 2,
        9..=12 => 3,
        13..=16 => 4,
        17..=21 => 5,
        22..=25 => 6,
        26..=29 => 7,
        30..=32 => 8,
        _ => 0, // unreachable
    };
    cycles * 8 + leaps_in_partial
}

/// Convert Jalali (Persian) date to JDN.
fn jalali_to_jdn(date: &JDateTime) -> Result<i64, DateTimeError> {
    // Assume parse_date_jalali returns (i32, i32, i32)
    let (y, m, d) = parse_date_jalali(date)?;
    let year = y as i64;
    let month = m as i64;
    let day = d as i64;

    let day_of_year = if month <= 6 {
        (month - 1) * 31 + day
    } else {
        186 + (month - 7) * 30 + day
    };
    const EPOCH: i64 = 1_948_320;
    let days_before_year = (year - 1) * 365 + leap_years_before(year);
    Ok(EPOCH + days_before_year + (day_of_year - 1))
}

/// Convert JDN to Jalali (Persian) date.
fn jdn_to_jalali(jdn: i64) -> Result<JDateTime, DateTimeError> {
    const EPOCH: i64 = 1_948_320;
    let days = jdn - EPOCH;
    if days < 0 {
        return Err(DateTimeError::InvalidDate(
            "date before 1/1/1 not supported".to_string(),
        ));
    }

    let mut year = (days / 365) + 1;
    loop {
        let start_days = (year - 1) * 365 + leap_years_before(year);
        if start_days > days {
            year -= 1;
            continue;
        }
        let days_in_year = if is_jalali_leap(year) { 366 } else { 365 };
        if start_days + days_in_year <= days {
            year += 1;
            continue;
        }
        break;
    }

    let day_of_year = days - ((year - 1) * 365 + leap_years_before(year)) + 1;
    let (month, day) = if day_of_year <= 186 {
        let month = (day_of_year - 1) / 31 + 1;
        let day = (day_of_year - 1) % 31 + 1;
        (month, day)
    } else {
        let d = day_of_year - 186;
        let month = (d - 1) / 30 + 7;
        let day = (d - 1) % 30 + 1;
        (month, day)
    };

    // Convert to strings – adjust if your JDateTime uses numeric fields
    Ok(JDateTime {
        year: year.to_string(),
        month: month.to_string(),
        day: day.to_string(),
        hour: None,
        minute: None,
        second: None,
    })
}

// ========================
//  Gregorian calendar
// ========================

/// Convert Gregorian date to JDN (integer, noon‑based).
fn gregorian_to_jdn(date: &DateTime) -> Result<i64, DateTimeError> {
    // Assume parse_date_gregorian returns (i32, i32, i32)
    let (y, m, d) = parse_date_gregorian(date)?;
    let year = y as i64;
    let month = m as i64;
    let day = d as i64;

    let a = (14 - month) / 12;
    let y = year + 4800 - a;
    let m = month + 12 * a - 3;
    let jdn = day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;
    Ok(jdn)
}

/// Convert JDN to Gregorian date.
fn jdn_to_gregorian(jdn: i64) -> DateTime {
    let a = jdn + 32044;
    let b = (4 * a + 3) / 146097;
    let c = a - (146097 * b) / 4;
    let d = (4 * c + 3) / 1461;
    let e = c - (1461 * d) / 4;
    let m = (5 * e + 2) / 153;
    let day = e - (153 * m + 2) / 5 + 1;
    let month = m + 3 - 12 * (m / 10);
    let year = 100 * b + d - 4800 + (m / 10);

    DateTime {
        year: year.to_string(),
        month: month.to_string(),
        day: day.to_string(),
        hour: None,
        minute: None,
        second: None,
    }
}

// ========================
//  Public API
// ========================

/// Convert Gregorian to Jalali.
pub fn to_jalali(greg: &DateTime) -> Result<JDateTime, DateTimeError> {
    let jdn = gregorian_to_jdn(greg)?;
    jdn_to_jalali(jdn)
}

/// Convert Jalali to Gregorian.
pub fn to_gregorian(jal: &JDateTime) -> Result<DateTime, DateTimeError> {
    let jdn = jalali_to_jdn(jal)?;
    Ok(jdn_to_gregorian(jdn))
}
