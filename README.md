# shomareh

Persian (Farsi) utilities for Rust: Jalali/Gregorian date conversion and phone/email validation, with full support for Persian and Arabic-Indic digits everywhere.

- **Jalali ↔ Gregorian** conversion via the Julian Day Number algorithm
- **Calendar-aware validation** (leap years, Esfand/Farvardin day limits)
- **Phone validation** for 200+ country calling codes, bundled as data (no runtime file I/O)
- **Email validation**
- **Persian and Arabic-Indic digit support** in every input: `۱۴۰۵/۰۲/۲۲` and `١٤٠٥/٠٢/٢٢` both work anywhere a date is accepted

## Installation

```toml
[dependencies]
shomareh = "0.1"
```

Requires Rust 1.85+ (edition 2024).

## Quick start

```rust
use shomareh::{DateTime, Gregorian, Jalali, is_email, is_phone, phone_country};

// Persian digits are accepted in every parser
let jalali = DateTime::<Jalali>::set_date("۱۴۰۵/۰۲/۲۲").unwrap();
assert_eq!(jalali.to_string(), "1405-02-22");

// Jalali <-> Gregorian
let greg = DateTime::<Gregorian>::set_date("2026/05/12").unwrap();
assert_eq!(greg.to_jalali().unwrap().to_string(), "1405-02-22");
assert_eq!(jalali.to_gregorian().unwrap().to_string(), "2026-05-12");

// Phones and emails
assert!(is_phone("+989123456789"));
assert!(is_phone("۰۰۹۸۹۱۲۳۴۵۶۷۸۹")); // 0098... == +98...
assert_eq!(phone_country("+989123456789", None), Some("Iran"));
assert!(is_email("me@gmail.com"));
```

## Dates and times

`DateTime<C>` is generic over a calendar marker: `Gregorian` or `Jalali`.
`JDateTime` is an alias for `DateTime<Jalali>`.

```rust
use shomareh::{DateTime, Gregorian, Jalali};

// Parse dates (YYYY/MM/DD or YYYY-MM-DD, Persian or English digits)
let date = DateTime::<Gregorian>::set_date("2026/05/12")?;

// Parse full datetimes and times
let dt = DateTime::<Gregorian>::set_datetime("2026/05/12 23:05:09")?;
assert_eq!(dt.hour(), Some("23"));

// Today / now in each calendar and the local timezone
let today_jalali = DateTime::<Jalali>::now()?;
let at_noon = DateTime::<Gregorian>::set_time("12:00:00")?;

// Numeric constructors are validated against the calendar:
// 12 Esfand may have 29 or 30 days depending on leap year
assert!(DateTime::<Jalali>::new(1403, 12, 30).is_ok());  // 1403 is leap
assert!(DateTime::<Jalali>::new(1404, 12, 30).is_err());
```

Accessors return borrowed English-digit strings; month, day and time components
are always zero-padded to two digits:

```rust
let dt = DateTime::<Jalali>::set_datetime("۱۴۰۵/۰۲/۲۲ ۰۹:۰۵:۰۰")?;
assert_eq!(dt.year(), "1405");
assert_eq!(dt.month(), "02");
assert_eq!(dt.day(), "22");
assert_eq!(dt.hour(), Some("09"));
assert_eq!(dt.to_string(), "1405-02-22 09:05:00");
```

### Leap years

```rust
use shomareh::is_jalali_leap;

assert!(is_jalali_leap(1403));
assert!(!is_jalali_leap(1404));
```

## Digit conversion

```rust
use shomareh::{to_english, to_persian};

assert_eq!(to_english("۱۴۰۵"), "1405");
assert_eq!(to_persian("1405"), "۱۴۰۵");
// Arabic-Indic digits (٠-٩, U+0660..U+0669) are accepted too
assert_eq!(to_english("١٤٠٥"), "1405");
assert_eq!(to_persian("١٤٠٥"), "۱۴۰۵");
// non-digit characters are left unchanged
assert_eq!(to_english("قیمت: ۱۲۳"), "قیمت: 123");
```

## Phone validation

Numbers are normalized before matching: Persian digits, `+`, spaces, dashes and
a leading `00` international prefix are all accepted.

```rust
use shomareh::{is_phone, phone_country, CountryName};

assert!(is_phone("+989123456789"));
assert!(is_phone("0098 912 345 6789"));
assert!(!is_phone("123456789"));

assert_eq!(phone_country("+989123456789", None), Some("Iran"));
assert_eq!(
    phone_country("+989123456789", Some(CountryName::Persian)),
    Some("ایران")
);
assert_eq!(phone_country("+999999999999", None), None);
```

Country data ships inside the binary (`src/data/ph.csv`), so validation works
without filesystem access. Numbers that share a country code are disambiguated
where possible: `+1` is resolved by North American area code (Canada, Jamaica,
Dominican Republic, Puerto Rico, Trinidad and Tobago, Barbados, Bahamas — falling
back to the United States for unassigned area codes), and `+7` by mobile block
(Russia `9xx` vs Kazakhstan `7xx`).

## Email validation

```rust
use shomareh::is_email;

assert!(is_email("me@gmail.com"));
assert!(!is_email("example.com"));
assert!(!is_email(""));
```

## Error handling

All parsing and conversion APIs return `Result<_, DateTimeError>`:

```rust
use shomareh::{DateTime, DateTimeError, Gregorian};

match DateTime::<Gregorian>::set_date("2026/02/30") {
    Err(e @ DateTimeError::InvalidDay(_)) => println!("rejected: {e}"),
    _ => unreachable!(),
}
```

Errors distinguish invalid year, month, day, hour, minute, second, date format
and datetime format, and implement `std::error::Error`.

## API overview

| Item | Description |
| --- | --- |
| `DateTime<C>` | Date (+ optional time) in calendar `C` |
| `Gregorian`, `Jalali` | Calendar markers; `JDateTime = DateTime<Jalali>` |
| `DateTime::set_date` / `set_datetime` / `set_time` / `now` / `new` | Validated constructors |
| `to_jalali` / `to_gregorian` | Calendar conversion (preserves time) |
| `is_jalali_leap` | Jalali leap-year check (33-year cycle) |
| `DateTimeValidator` | Reusable component validators (`validate_year`, …) |
| `to_english` / `to_persian` | Digit conversion |
| `is_phone` / `phone_country` | Phone validation and country lookup |
| `is_email` | Email validation |

## License

MIT OR Apache-2.0.

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).
