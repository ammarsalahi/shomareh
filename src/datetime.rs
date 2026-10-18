//! Jalali and Gregorian date/time types, validation, and conversion.

use crate::validator::to_english;
use chrono::{Datelike, Local, Timelike};
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

/// Errors produced while validating or converting dates and times.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateTimeError {
    /// The year component is missing or out of range.
    InvalidYear(String),
    /// The month component is missing or out of range.
    InvalidMonth(String),
    /// The day component is missing or out of range for the calendar.
    InvalidDay(String),
    /// The hour component is missing or out of range.
    InvalidHour(String),
    /// The minute component is missing or out of range.
    InvalidMinute(String),
    /// The second component is missing or out of range.
    InvalidSecond(String),
    /// The date string does not have the expected format.
    InvalidDate(String),
    /// The datetime string does not have the expected format.
    InvalidDateTime(String),
}

impl fmt::Display for DateTimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DateTimeError::InvalidYear(msg) => write!(f, "Invalid Year: {msg}"),
            DateTimeError::InvalidMonth(msg) => write!(f, "Invalid Month: {msg}"),
            DateTimeError::InvalidDay(msg) => write!(f, "Invalid Day: {msg}"),
            DateTimeError::InvalidHour(msg) => write!(f, "Invalid Hour: {msg}"),
            DateTimeError::InvalidMinute(msg) => write!(f, "Invalid Minute: {msg}"),
            DateTimeError::InvalidSecond(msg) => write!(f, "Invalid Second: {msg}"),
            DateTimeError::InvalidDate(msg) => write!(f, "Invalid Date: {msg}"),
            DateTimeError::InvalidDateTime(msg) => write!(f, "Invalid DateTime: {msg}"),
        }
    }
}

impl std::error::Error for DateTimeError {}

mod sealed {
    pub trait Sealed {}
}

/// A calendar system supported by [`DateTime`].
///
/// Implementations are sealed; use the provided [`Gregorian`] and [`Jalali`] markers.
pub trait Calendar: sealed::Sealed + Copy + Default + fmt::Debug + 'static {
    /// Number of days in `month` of `year` (0 when the month itself is invalid).
    #[doc(hidden)]
    fn max_days(year: i64, month: u8) -> u8;

    /// Today's date `(year, month, day)` in this calendar.
    #[doc(hidden)]
    fn today() -> Result<(i64, u8, u8), DateTimeError>;
}

/// The Gregorian (Western) calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Gregorian;

fn is_gregorian_leap(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

impl sealed::Sealed for Gregorian {}

impl Calendar for Gregorian {
    fn max_days(year: i64, month: u8) -> u8 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if is_gregorian_leap(year) => 29,
            2 => 28,
            _ => 0,
        }
    }

    fn today() -> Result<(i64, u8, u8), DateTimeError> {
        let now = Local::now();
        Ok((i64::from(now.year()), now.month() as u8, now.day() as u8))
    }
}

/// The Jalali (Persian / Solar Hijri) calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Jalali;

/// Check if a Jalali year is a leap year (33-year cycle).
pub fn is_jalali_leap(year: i64) -> bool {
    matches!((year + 33) % 33, 1 | 5 | 9 | 13 | 17 | 22 | 26 | 30)
}

/// Number of leap years strictly before the given Jalali year.
fn leap_years_before(year: i64) -> i64 {
    if year <= 1 {
        return 0;
    }
    let complete = year - 1;
    let cycles = complete / 33;
    let rem = complete % 33;
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
        _ => 0,
    };
    cycles * 8 + leaps_in_partial
}

impl sealed::Sealed for Jalali {}

impl Calendar for Jalali {
    fn max_days(year: i64, month: u8) -> u8 {
        match month {
            1..=6 => 31,
            7..=11 => 30,
            12 if is_jalali_leap(year) => 30,
            12 => 29,
            _ => 0,
        }
    }

    fn today() -> Result<(i64, u8, u8), DateTimeError> {
        let (year, month, day) = Gregorian::today()?;
        jdn_to_jalali(gregorian_to_jdn(year, month, day))
    }
}

/// Shared validation and parsing for [`DateTime`] components.
///
/// All inputs may use Persian digits; validation and storage always use English digits.
pub trait DateTimeValidator {
    /// Validates a year: 1-4 digits, value 1-9999.
    fn validate_year(year: &str) -> Result<(), DateTimeError> {
        parse_ranged(
            year,
            "year",
            None,
            1..=9999,
            "between 1-9999",
            DateTimeError::InvalidYear,
        )
        .map(|_| ())
    }

    /// Validates a month: exactly 2 digits, value 01-12.
    fn validate_month(month: &str) -> Result<(), DateTimeError> {
        parse_ranged(
            month,
            "month",
            Some(2),
            1..=12,
            "between 01-12",
            DateTimeError::InvalidMonth,
        )
        .map(|_| ())
    }

    /// Validates a day: exactly 2 digits, value 01-31.
    fn validate_day(day: &str) -> Result<(), DateTimeError> {
        parse_ranged(
            day,
            "day",
            Some(2),
            1..=31,
            "between 01-31",
            DateTimeError::InvalidDay,
        )
        .map(|_| ())
    }

    /// Validates an hour: value 00-23.
    fn validate_hour(hour: &str) -> Result<(), DateTimeError> {
        parse_ranged(
            hour,
            "hour",
            None,
            0..=23,
            "between 00-23",
            DateTimeError::InvalidHour,
        )
        .map(|_| ())
    }

    /// Validates a minute: exactly 2 digits, value 00-59.
    fn validate_minute(minute: &str) -> Result<(), DateTimeError> {
        parse_ranged(
            minute,
            "minute",
            Some(2),
            0..=59,
            "between 00-59",
            DateTimeError::InvalidMinute,
        )
        .map(|_| ())
    }

    /// Validates a second: exactly 2 digits, value 00-59.
    fn validate_second(second: &str) -> Result<(), DateTimeError> {
        parse_ranged(
            second,
            "second",
            Some(2),
            0..=59,
            "between 00-59",
            DateTimeError::InvalidSecond,
        )
        .map(|_| ())
    }

    /// Parses and validates a `YYYY/MM/DD` or `YYYY-MM-DD` date string.
    ///
    /// Persian digits are accepted. Returns `(year, month, day)` with month and day
    /// zero-padded to two digits.
    fn parse_validate_date(date: &str) -> Result<(String, String, String), DateTimeError> {
        let parts: Vec<&str> = date.split(['/', '-']).collect();
        let [year, month, day]: [&str; 3] = parts.try_into().map_err(|_| {
            DateTimeError::InvalidDate(format!(
                "'{date}' must be in format YYYY/MM/DD or YYYY-MM-DD"
            ))
        })?;

        Self::validate_year(year)?;
        Self::validate_month(month)?;
        Self::validate_day(day)?;

        Ok((
            normalize_year(year),
            pad_normalized(month),
            pad_normalized(day),
        ))
    }

    /// Parses and validates an `HH:MM:SS` time string.
    ///
    /// Returns `(hour, minute, second)` with each component zero-padded to two digits.
    fn parse_validate_time(time: &str) -> Result<(String, String, String), DateTimeError> {
        let parts: Vec<&str> = time.split(':').collect();
        let [hour, minute, second]: [&str; 3] = parts.try_into().map_err(|_| {
            DateTimeError::InvalidDate(format!("'{time}' must be in format HH:MM:SS"))
        })?;

        Self::validate_hour(hour)?;
        Self::validate_minute(minute)?;
        Self::validate_second(second)?;

        Ok((
            pad_normalized(hour),
            pad_normalized(minute),
            pad_normalized(second),
        ))
    }
}

fn require_digits(
    input: &str,
    field: &str,
    err: fn(String) -> DateTimeError,
) -> Result<u32, DateTimeError> {
    let input = to_english(input);
    if input.is_empty() {
        return Err(err(format!("{field} can not be empty")));
    }
    if !input.chars().all(|c| c.is_ascii_digit()) {
        return Err(err(format!("'{input}' contains non-digit characters")));
    }
    input
        .parse()
        .map_err(|_| err(format!("'{input}' is not a valid number")))
}

fn parse_ranged(
    input: &str,
    field: &str,
    width: Option<usize>,
    range: std::ops::RangeInclusive<u32>,
    range_label: &str,
    err: fn(String) -> DateTimeError,
) -> Result<String, DateTimeError> {
    let value = require_digits(input, field, err)?;
    if let Some(width) = width {
        let digits = to_english(input);
        if digits.len() != width {
            return Err(err(format!(
                "{field} must be {width} digits, got '{digits}'"
            )));
        }
    }
    if !range.contains(&value) {
        return Err(err(format!("{field} must be {range_label}, got {value}")));
    }
    Ok(value.to_string())
}

fn pad_normalized(input: &str) -> String {
    let value: u32 = to_english(input).parse().unwrap_or(0);
    format!("{value:02}")
}

/// Converts to English digits and drops leading zeros so that string- and
/// numeric-constructed dates agree (e.g. `"0001"` and `new(1, ..)` both yield `"1"`).
fn normalize_year(input: &str) -> String {
    let digits = to_english(input);
    let trimmed = digits.trim_start_matches('0');
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

fn pad2(value: u8) -> String {
    format!("{value:02}")
}

/// A date (and optional time) in calendar `C`, e.g. [`Gregorian`] or [`Jalali`].
///
/// Components are stored as English-digit strings; month, day and time components
/// are always zero-padded to two digits.
///
/// ```
/// use shomareh::{DateTime, Jalali};
///
/// let date = DateTime::<Jalali>::set_date("۱۴۰۵/۰۲/۲۲").unwrap();
/// assert_eq!(date.year(), "1405");
/// assert_eq!(date.month(), "02");
/// assert_eq!(date.day(), "22");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateTime<C: Calendar> {
    year: String,
    month: String,
    day: String,
    hour: Option<String>,
    minute: Option<String>,
    second: Option<String>,
    calendar: PhantomData<C>,
}

/// A Gregorian calendar date/time. Alias for `DateTime<Gregorian>`.
pub type GregorianDateTime = DateTime<Gregorian>;

/// A Jalali (Persian) calendar date/time. Alias for `DateTime<Jalali>`.
pub type JDateTime = DateTime<Jalali>;

impl<C: Calendar> DateTimeValidator for DateTime<C> {}

impl<C: Calendar> DateTime<C> {
    fn from_parts(
        year: i64,
        month: u8,
        day: u8,
        hour: Option<String>,
        minute: Option<String>,
        second: Option<String>,
    ) -> Self {
        Self {
            year: year.to_string(),
            month: pad2(month),
            day: pad2(day),
            hour,
            minute,
            second,
            calendar: PhantomData,
        }
    }

    fn from_strings(
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
            calendar: PhantomData,
        }
    }

    fn parts(&self) -> Result<(i64, u8, u8), DateTimeError> {
        let year =
            i64::from_str(&self.year).map_err(|_| DateTimeError::InvalidYear(self.year.clone()))?;
        let month = u8::from_str(&self.month)
            .map_err(|_| DateTimeError::InvalidMonth(self.month.clone()))?;
        let day =
            u8::from_str(&self.day).map_err(|_| DateTimeError::InvalidDay(self.day.clone()))?;
        Ok((year, month, day))
    }

    fn validate_calendar_date(&self) -> Result<(), DateTimeError> {
        let (year, month, day) = self.parts()?;
        if day == 0 || day > C::max_days(year, month) {
            return Err(DateTimeError::InvalidDay(format!(
                "day {} is out of range for {}-{} in this calendar",
                self.day, self.year, self.month
            )));
        }
        Ok(())
    }

    /// Creates a date from numeric components, validating against calendar `C`.
    ///
    /// Month and day must fit the calendar (e.g. Esfand 30 is only valid in a leap
    /// Jalali year).
    pub fn new(year: i64, month: u8, day: u8) -> Result<Self, DateTimeError> {
        if !(1..=12).contains(&month) {
            return Err(DateTimeError::InvalidMonth(month.to_string()));
        }
        let this = Self::from_parts(year, month, day, None, None, None);
        this.validate_calendar_date()?;
        Ok(this)
    }

    /// Parses a `YYYY/MM/DD` or `YYYY-MM-DD` date string (Persian digits accepted).
    pub fn set_date(date: &str) -> Result<Self, DateTimeError> {
        let (year, month, day) = Self::parse_validate_date(date)?;
        let this = Self::from_strings(year, month, day, None, None, None);
        this.validate_calendar_date()?;
        Ok(this)
    }

    /// Parses a `YYYY/MM/DD HH:MM:SS` datetime string (Persian digits accepted).
    pub fn set_datetime(datetime: &str) -> Result<Self, DateTimeError> {
        let parts: Vec<&str> = datetime.split_whitespace().collect();
        let [date, time]: [&str; 2] = parts.try_into().map_err(|_| {
            DateTimeError::InvalidDateTime(format!(
                "'{datetime}' must be in format YYYY/MM/DD HH:MM:SS"
            ))
        })?;

        let (year, month, day) = Self::parse_validate_date(date)?;
        let (hour, minute, second) = Self::parse_validate_time(time)?;

        let this = Self::from_strings(year, month, day, Some(hour), Some(minute), Some(second));
        this.validate_calendar_date()?;
        Ok(this)
    }

    /// Builds a datetime for today (in calendar `C`) with the given `HH:MM:SS` time.
    pub fn set_time(time: &str) -> Result<Self, DateTimeError> {
        let (year, month, day) = C::today()?;
        let (hour, minute, second) = Self::parse_validate_time(time)?;
        Ok(Self::from_parts(
            year,
            month,
            day,
            Some(hour),
            Some(minute),
            Some(second),
        ))
    }

    /// Current date and time in calendar `C` and the local timezone.
    pub fn now() -> Result<Self, DateTimeError> {
        let (year, month, day) = C::today()?;
        let now = Local::now();
        Ok(Self::from_parts(
            year,
            month,
            day,
            Some(pad2(now.hour() as u8)),
            Some(pad2(now.minute() as u8)),
            Some(pad2(now.second() as u8)),
        ))
    }

    /// The year as an English-digit string.
    pub fn year(&self) -> &str {
        &self.year
    }

    /// The zero-padded month as an English-digit string.
    pub fn month(&self) -> &str {
        &self.month
    }

    /// The zero-padded day as an English-digit string.
    pub fn day(&self) -> &str {
        &self.day
    }

    /// The zero-padded hour, if a time was set.
    pub fn hour(&self) -> Option<&str> {
        self.hour.as_deref()
    }

    /// The zero-padded minute, if a time was set.
    pub fn minute(&self) -> Option<&str> {
        self.minute.as_deref()
    }

    /// The zero-padded second, if a time was set.
    pub fn second(&self) -> Option<&str> {
        self.second.as_deref()
    }
}

impl<C: Calendar> fmt::Display for DateTime<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)?;
        if let (Some(hour), Some(minute), Some(second)) = (&self.hour, &self.minute, &self.second) {
            write!(f, " {hour}:{minute}:{second}")?;
        }
        Ok(())
    }
}

// ========================
//  Julian Day Number core
// ========================

fn gregorian_to_jdn(year: i64, month: u8, day: u8) -> i64 {
    let a = (14 - i64::from(month)) / 12;
    let y = year + 4800 - a;
    let m = i64::from(month) + 12 * a - 3;
    i64::from(day) + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
}

fn jdn_to_gregorian(jdn: i64) -> (i64, u8, u8) {
    let a = jdn + 32044;
    let b = (4 * a + 3) / 146_097;
    let c = a - (146_097 * b) / 4;
    let d = (4 * c + 3) / 1461;
    let e = c - (1461 * d) / 4;
    let m = (5 * e + 2) / 153;
    let day = e - (153 * m + 2) / 5 + 1;
    let month = m + 3 - 12 * (m / 10);
    let year = 100 * b + d - 4800 + m / 10;
    (year, month as u8, day as u8)
}

const JALALI_EPOCH: i64 = 1_948_320;

fn jalali_to_jdn(year: i64, month: u8, day: u8) -> i64 {
    let day_of_year = if month <= 6 {
        (i64::from(month) - 1) * 31 + i64::from(day)
    } else {
        186 + (i64::from(month) - 7) * 30 + i64::from(day)
    };
    let days_before_year = (year - 1) * 365 + leap_years_before(year);
    JALALI_EPOCH + days_before_year + (day_of_year - 1)
}

fn jdn_to_jalali(jdn: i64) -> Result<(i64, u8, u8), DateTimeError> {
    let days = jdn - JALALI_EPOCH;
    if days < 0 {
        return Err(DateTimeError::InvalidDate(
            "dates before 1/1/1 are not supported".to_string(),
        ));
    }

    let mut year = days / 365 + 1;
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
        (
            ((day_of_year - 1) / 31 + 1) as u8,
            ((day_of_year - 1) % 31 + 1) as u8,
        )
    } else {
        let offset = day_of_year - 186;
        (((offset - 1) / 30 + 7) as u8, ((offset - 1) % 30 + 1) as u8)
    };

    Ok((year, month, day))
}

// ========================
//  Public conversion API
// ========================

/// Converts a Gregorian date to the Jalali calendar, preserving the time components.
///
/// ```
/// use shomareh::{to_jalali, DateTime, Gregorian};
///
/// let greg = DateTime::<Gregorian>::set_date("2026/05/12").unwrap();
/// let jalali = to_jalali(&greg).unwrap();
/// assert_eq!((jalali.year(), jalali.month(), jalali.day()), ("1405", "02", "22"));
/// ```
pub fn to_jalali(greg: &DateTime<Gregorian>) -> Result<DateTime<Jalali>, DateTimeError> {
    let (year, month, day) = greg.parts()?;
    let (jy, jm, jd) = jdn_to_jalali(gregorian_to_jdn(year, month, day))?;
    Ok(DateTime::from_parts(
        jy,
        jm,
        jd,
        greg.hour.clone(),
        greg.minute.clone(),
        greg.second.clone(),
    ))
}

/// Converts a Jalali date to the Gregorian calendar, preserving the time components.
///
/// ```
/// use shomareh::{to_gregorian, DateTime, Jalali};
///
/// let jalali = DateTime::<Jalali>::set_date("1405/02/22").unwrap();
/// let greg = to_gregorian(&jalali).unwrap();
/// assert_eq!((greg.year(), greg.month(), greg.day()), ("2026", "05", "12"));
/// ```
pub fn to_gregorian(jal: &DateTime<Jalali>) -> Result<DateTime<Gregorian>, DateTimeError> {
    let (year, month, day) = jal.parts()?;
    let (gy, gm, gd) = jdn_to_gregorian(jalali_to_jdn(year, month, day));
    Ok(DateTime::from_parts(
        gy,
        gm,
        gd,
        jal.hour.clone(),
        jal.minute.clone(),
        jal.second.clone(),
    ))
}

impl DateTime<Gregorian> {
    /// Converts this date to the Jalali calendar.
    pub fn to_jalali(&self) -> Result<DateTime<Jalali>, DateTimeError> {
        to_jalali(self)
    }
}

impl DateTime<Jalali> {
    /// Converts this date to the Gregorian calendar.
    pub fn to_gregorian(&self) -> Result<DateTime<Gregorian>, DateTimeError> {
        to_gregorian(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_year() {
        assert!(DateTime::<Gregorian>::validate_year("2026").is_ok());
        assert!(DateTime::<Gregorian>::validate_year("۱۴۰۰").is_ok());
        assert!(DateTime::<Gregorian>::validate_year("1").is_ok());

        assert!(DateTime::<Gregorian>::validate_year("").is_err());
        assert!(DateTime::<Gregorian>::validate_year("year").is_err());
        assert!(DateTime::<Gregorian>::validate_year("-1").is_err());
    }

    #[test]
    fn validate_month() {
        assert!(DateTime::<Gregorian>::validate_month("01").is_ok());
        assert!(DateTime::<Gregorian>::validate_month("۱۲").is_ok());

        assert!(DateTime::<Gregorian>::validate_month("").is_err());
        assert!(DateTime::<Gregorian>::validate_month("-1").is_err());
        assert!(DateTime::<Gregorian>::validate_month("7").is_err());
        assert!(DateTime::<Gregorian>::validate_month("13").is_err());
    }

    #[test]
    fn validate_day() {
        assert!(DateTime::<Gregorian>::validate_day("01").is_ok());
        assert!(DateTime::<Gregorian>::validate_day("31").is_ok());

        assert!(DateTime::<Gregorian>::validate_day("").is_err());
        assert!(DateTime::<Gregorian>::validate_day("-1").is_err());
        assert!(DateTime::<Gregorian>::validate_day("7").is_err());
        assert!(DateTime::<Gregorian>::validate_day("32").is_err());
    }

    #[test]
    fn validate_hour() {
        assert!(DateTime::<Gregorian>::validate_hour("00").is_ok());
        assert!(DateTime::<Gregorian>::validate_hour("23").is_ok());

        assert!(DateTime::<Gregorian>::validate_hour("").is_err());
        assert!(DateTime::<Gregorian>::validate_hour("-1").is_err());
        assert!(DateTime::<Gregorian>::validate_hour("24").is_err());
    }

    #[test]
    fn validate_minute_and_second() {
        assert!(DateTime::<Gregorian>::validate_minute("00").is_ok());
        assert!(DateTime::<Gregorian>::validate_minute("59").is_ok());
        assert!(DateTime::<Gregorian>::validate_minute("60").is_err());
        assert!(DateTime::<Gregorian>::validate_second("60").is_err());
        assert!(DateTime::<Gregorian>::validate_second("").is_err());
    }

    #[test]
    fn parse_date_accepts_persian_digits() {
        let (y, m, d) = DateTime::<Gregorian>::parse_validate_date("۱۴۰۵/۰۲/۲۲").unwrap();
        assert_eq!((y.as_str(), m.as_str(), d.as_str()), ("1405", "02", "22"));
    }

    #[test]
    fn parse_accepts_arabic_indic_digits() {
        let dt = DateTime::<Gregorian>::set_datetime("٢٠٢٦/٠٥/١٢ ٠٩:٠٥:٠٠").unwrap();
        assert_eq!(dt.to_string(), "2026-05-12 09:05:00");
        // Calendar-aware validation still applies with Arabic-Indic digits.
        assert!(DateTime::<Jalali>::set_date("١٤٠٣/١٢/٣٠").is_ok());
        assert!(DateTime::<Jalali>::set_date("١٤٠٤/١٢/٣٠").is_err());
    }

    #[test]
    fn year_is_normalized() {
        let from_str = DateTime::<Gregorian>::set_date("0001/01/01").unwrap();
        let from_num = DateTime::<Gregorian>::new(1, 1, 1).unwrap();
        assert_eq!(from_str.year(), "1");
        assert_eq!(from_str, from_num);
    }

    #[test]
    fn parse_datetime_and_time() {
        let dt = DateTime::<Gregorian>::set_datetime("2026/01/01 12:12:12").unwrap();
        assert_eq!(dt.hour(), Some("12"));
        assert_eq!(dt.minute(), Some("12"));
        assert_eq!(dt.second(), Some("12"));

        let (h, m, s) =
            <DateTime<Gregorian> as DateTimeValidator>::parse_validate_time("۹:05:00").unwrap();
        assert_eq!((h.as_str(), m.as_str(), s.as_str()), ("09", "05", "00"));
    }

    #[test]
    fn calendar_specific_day_limits() {
        // 12 Esfand is 30 only in a leap Jalali year; 1403 is leap, 1404 is not.
        assert!(DateTime::<Jalali>::set_date("1403/12/30").is_ok());
        assert!(DateTime::<Jalali>::set_date("1404/12/30").is_err());
        assert!(DateTime::<Gregorian>::set_date("2026/02/29").is_err());
        assert!(DateTime::<Gregorian>::set_date("2024/02/29").is_ok());
    }

    #[test]
    fn jalali_leap_years() {
        assert!(is_jalali_leap(1403));
        assert!(!is_jalali_leap(1404));
        assert!(!is_jalali_leap(1405));
    }

    #[test]
    fn gregorian_to_jalali_and_back() {
        let greg = DateTime::<Gregorian>::set_date("2026/05/12").unwrap();
        let jalali = greg.to_jalali().unwrap();
        assert_eq!(
            (jalali.year(), jalali.month(), jalali.day()),
            ("1405", "02", "22")
        );

        let back = jalali.to_gregorian().unwrap();
        assert_eq!(
            (back.year(), back.month(), back.day()),
            ("2026", "05", "12")
        );
    }

    #[test]
    fn conversion_preserves_time() {
        let greg = DateTime::<Gregorian>::set_datetime("2026/05/12 23:05:09").unwrap();
        let jalali = to_jalali(&greg).unwrap();
        assert_eq!(jalali.hour(), Some("23"));
        assert_eq!(jalali.minute(), Some("05"));
        assert_eq!(jalali.second(), Some("09"));
    }

    #[test]
    fn jdn_roundtrip_over_leap_boundary() {
        for jdn in [1_948_320, 2_460_000, 2_461_000, 2_500_000] {
            let (y, m, d) = jdn_to_jalali(jdn).unwrap();
            assert_eq!(jalali_to_jdn(y, m, d), jdn);
            let (gy, gm, gd) = jdn_to_gregorian(jdn);
            assert_eq!(gregorian_to_jdn(gy, gm, gd), jdn);
        }
    }

    #[test]
    fn now_is_consistent_with_chrono() {
        let now = DateTime::<Gregorian>::now().unwrap();
        let local = Local::now();
        assert_eq!(now.year(), local.year().to_string());
        assert_eq!(now.month(), format!("{:02}", local.month()));
        assert_eq!(now.day(), format!("{:02}", local.day()));
    }

    #[test]
    fn display_formats_date_and_time() {
        let dt = DateTime::<Gregorian>::set_datetime("2026/05/12 23:05:09").unwrap();
        assert_eq!(dt.to_string(), "2026-05-12 23:05:09");
        let date = DateTime::<Gregorian>::set_date("2026/05/12").unwrap();
        assert_eq!(date.to_string(), "2026-05-12");
    }
}
