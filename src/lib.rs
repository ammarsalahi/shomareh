//! Persian (Farsi) utilities for Rust: Jalali/Gregorian date conversion and
//! phone/email validation, with full support for Persian digits.
//!
//! # Examples
//!
//! ```
//! use shomareh::{DateTime, Gregorian, Jalali, is_email, is_phone, phone_country};
//!
//! // Jalali <-> Gregorian conversion
//! let greg = DateTime::<Gregorian>::set_date("2026/05/12").unwrap();
//! let jalali = greg.to_jalali().unwrap();
//! assert_eq!(jalali.to_string(), "1405-02-22");
//!
//! // Persian digits are accepted everywhere
//! let date = DateTime::<Jalali>::set_date("۱۴۰۵/۰۲/۲۲").unwrap();
//! assert_eq!(date.day(), "22");
//!
//! // Phone & email validation
//! assert!(is_phone("+989123456789"));
//! assert_eq!(phone_country("+989123456789", None), Some("Iran"));
//! assert!(is_email("me@gmail.com"));
//! ```
#![warn(missing_docs)]

pub mod datetime;
pub mod validator;

pub use datetime::{
    Calendar, DateTime, DateTimeError, DateTimeValidator, Gregorian, GregorianDateTime, JDateTime,
    Jalali, is_jalali_leap, to_gregorian, to_jalali,
};
pub use validator::{CountryName, is_email, is_phone, phone_country, to_english, to_persian};
