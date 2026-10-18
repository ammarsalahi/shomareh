//! Persian digit conversion plus phone and email validation.

use csv::ReaderBuilder;
use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;

/// ASCII digits (`0`-`9`).
const EN_DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
/// Persian (Farsi) digits, `U+06F0..=U+06F9`.
const FA_DIGITS: [char; 10] = ['۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹'];

/// Numeric value `0..=9` of an English, Persian, or Arabic-Indic digit.
///
/// The three numeral systems each occupy a contiguous `char` range, so the value can be
/// recovered with a subtraction instead of a lookup table. Arabic-Indic digits
/// (`U+0660..=U+0669`) commonly appear alongside Persian ones in mixed text.
fn digit_value(c: char) -> Option<u32> {
    match c {
        '0'..='9' => Some(c as u32 - '0' as u32),
        '۰'..='۹' => Some(c as u32 - '۰' as u32),
        '٠'..='٩' => Some(c as u32 - '٠' as u32),
        _ => None,
    }
}

fn map_digits(input: &str, to: &[char; 10]) -> String {
    input
        .chars()
        .map(|c| digit_value(c).map_or(c, |d| to[d as usize]))
        .collect()
}

/// Converts Persian (Farsi) and Arabic-Indic digits to English digits, leaving other
/// characters unchanged.
///
/// ```
/// use shomareh::to_english;
/// assert_eq!(to_english("۱۴۰۵"), "1405");
/// assert_eq!(to_english("١٤٠٥"), "1405");
/// ```
pub fn to_english(input: &str) -> String {
    map_digits(input, &EN_DIGITS)
}

/// Converts English and Arabic-Indic digits to Persian (Farsi) digits, leaving other
/// characters unchanged.
///
/// ```
/// use shomareh::to_persian;
/// assert_eq!(to_persian("1405"), "۱۴۰۵");
/// assert_eq!(to_persian("١٤٠٥"), "۱۴۰۵");
/// ```
pub fn to_persian(input: &str) -> String {
    map_digits(input, &FA_DIGITS)
}

static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .expect("email regex must be valid")
});

/// Returns `true` if the string looks like a valid email address.
///
/// ```
/// use shomareh::is_email;
/// assert!(is_email("me@gmail.com"));
/// assert!(!is_email("example.com"));
/// ```
pub fn is_email(email: &str) -> bool {
    !email.is_empty() && EMAIL_REGEX.is_match(email)
}

#[derive(Debug, Deserialize)]
struct RawPhone {
    code: u32,
    min_len: u8,
    max_len: u8,
    /// Semicolon-separated national leading digits, e.g. `"6;7;8"` (empty means "any").
    prefixes: Option<String>,
    english_name: String,
    persian_name: String,
}

/// A parsed phone row.
///
/// The bundled CSV (`data/ph.csv`, columns
/// `code,min_len,max_len,prefixes,english_name,persian_name`) stores the *national*
/// number length (country calling code excluded) and, optionally, semicolon-separated
/// leading digits of the national number. Trunk prefixes (a leading `0`) are dropped
/// in international format, so they must not appear in `prefixes`.
///
/// Country codes that cannot be told apart by the code alone are distinguished by
/// prefix: the NANP (`+1`) members by area code, and the shared `+7` by mobile block
/// (Russia `9xx` vs Kazakhstan `7xx`). The bare `+1` row for the United States is kept
/// prefix-free so it acts as the fallback for unassigned area codes. NANP area-code
/// assignments follow the North American Numbering Plan Administrator (NANPA).
struct Phone {
    code: String,
    prefixes: Vec<String>,
    min_len: usize,
    max_len: usize,
    english_name: String,
    persian_name: String,
}

impl Phone {
    fn new(raw: RawPhone) -> Self {
        let prefixes = raw
            .prefixes
            .as_deref()
            .map(|prefixes| {
                prefixes
                    .split(';')
                    .map(str::trim)
                    .filter(|prefix| !prefix.is_empty())
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default();

        Self {
            code: raw.code.to_string(),
            prefixes,
            min_len: usize::from(raw.min_len),
            max_len: usize::from(raw.max_len),
            english_name: raw.english_name,
            persian_name: raw.persian_name,
        }
    }

    /// Longest `code + prefix` start string, used to order specific rows first.
    fn max_start_len(&self) -> usize {
        self.code.len() + self.prefixes.iter().map(String::len).max().unwrap_or(0)
    }

    /// Does `normalized` (digits only, no `+`) belong to this row?
    fn matches(&self, normalized: &str) -> bool {
        let national_len = normalized.len().saturating_sub(self.code.len());
        if !(self.min_len..=self.max_len).contains(&national_len) {
            return false;
        }
        let Some(national) = normalized.strip_prefix(self.code.as_str()) else {
            return false;
        };
        self.prefixes.is_empty()
            || self
                .prefixes
                .iter()
                .any(|prefix| national.starts_with(prefix))
    }
}

/// Language used when reporting a phone number's country.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CountryName {
    /// Report the English country name.
    #[default]
    English,
    /// Report the Persian (Farsi) country name.
    Persian,
}

static PHONE_DATA: LazyLock<Vec<Phone>> = LazyLock::new(|| {
    const CSV: &str = include_str!("data/ph.csv");

    let raw: Vec<RawPhone> = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(CSV.as_bytes())
        .deserialize()
        .collect::<Result<_, csv::Error>>()
        .expect("bundled phone data (src/data/ph.csv) must be valid");

    let mut phones: Vec<Phone> = raw.into_iter().map(Phone::new).collect();

    // Longest prefix first so that overlapping prefixes match correctly.
    phones.sort_by_key(|phone| std::cmp::Reverse(phone.max_start_len()));
    phones
});

fn normalize_phone(phone: &str) -> String {
    let digits: String = to_english(phone)
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    // "00" is the international call prefix (e.g. 0098... == +98...).
    digits.strip_prefix("00").unwrap_or(&digits).to_string()
}

fn lookup_phone(phone: &str) -> Option<&'static Phone> {
    let normalized = normalize_phone(phone);
    if normalized.is_empty() {
        return None;
    }
    PHONE_DATA.iter().find(|row| row.matches(&normalized))
}

/// Returns `true` if the string is a valid international phone number.
///
/// Persian digits, `+`, spaces and dashes are accepted; only digits are compared.
///
/// ```
/// use shomareh::is_phone;
/// assert!(is_phone("+989123456789"));
/// assert!(is_phone("۰۰۹۸۹۱۲۳۴۵۶۷۸۹"));
/// assert!(!is_phone("123456789"));
/// ```
pub fn is_phone(phone: &str) -> bool {
    lookup_phone(phone).is_some()
}

/// Validates a phone number and returns its country name.
///
/// Returns `None` when the number does not match any known country. Within the North
/// American Numbering Plan the country is resolved from the area code; numbers whose
/// area code is not assigned to any other member default to the United States.
///
/// ```
/// use shomareh::{phone_country, CountryName};
/// assert_eq!(phone_country("+989123456789", None), Some("Iran"));
/// assert_eq!(
///     phone_country("+989123456789", Some(CountryName::Persian)),
///     Some("ایران")
/// );
/// assert_eq!(phone_country("+979123456789", None), None);
/// ```
pub fn phone_country(phone: &str, language: Option<CountryName>) -> Option<&'static str> {
    let row = lookup_phone(phone)?;
    Some(match language.unwrap_or_default() {
        CountryName::English => &row.english_name,
        CountryName::Persian => &row.persian_name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_conversion() {
        assert_eq!(to_persian("+1234"), "+۱۲۳۴");
        assert_eq!(to_english("۱۲۳۴۵۶۷۸۹۰"), "1234567890");
        assert_eq!(to_english("بدون رقم"), "بدون رقم");
    }

    #[test]
    fn arabic_indic_digit_conversion() {
        // Arabic-Indic digits (U+0660..=U+0669) are recognized by both directions.
        assert_eq!(to_english("٠١٢٣٤٥٦٧٨٩"), "0123456789");
        assert_eq!(to_english("مبلغ ١٢٣ تومان"), "مبلغ 123 تومان");
        assert_eq!(to_persian("٠١٢٣٤٥٦٧٨٩"), "۰۱۲۳۴۵۶۷۸۹");
        assert_eq!(to_persian("15٠"), "۱۵۰");
        // Persian and Arabic-Indic digits can be mixed in a single string.
        assert_eq!(to_english("۱۴٠٥/٠٢/۲۲"), "1405/02/22");
        // Non-digits are still left untouched.
        assert_eq!(to_english("abc ٠"), "abc 0");
    }

    #[test]
    fn phone_accepts_arabic_indic_digits() {
        assert!(is_phone("+٩٨٩١٢٣٤٥٦٧٨٩"));
        assert_eq!(phone_country("+٩٨٩١٢٣٤٥٦٧٨٩", None), Some("Iran"));
    }

    #[test]
    fn phone_validation() {
        assert!(is_phone("+989123456789"));
        assert!(is_phone("00989123456789"));
        assert!(is_phone("+۹۸۹۱۲۳۴۵۶۷۸۹"));
        assert!(is_phone("+98 912 345 6789"));
        assert!(!is_phone("123456789"));
        assert!(!is_phone("+979123456789"));
        assert!(!is_phone(""));
        assert!(!is_phone("not a phone"));
    }

    #[test]
    fn phone_validation_single_prefix_countries() {
        // Length is the national number length, not code + national.
        assert_eq!(phone_country("+14155552671", None), Some("United States"));
        assert_eq!(phone_country("+447123456789", None), Some("United Kingdom"));
        assert_eq!(phone_country("+8613800138000", None), Some("China"));
        assert_eq!(phone_country("+819012345678", None), Some("Japan"));
    }

    #[test]
    fn phone_validation_multi_prefix_countries() {
        // Rows with semicolon-separated prefixes must not be dropped.
        assert_eq!(phone_country("+33612345678", None), Some("France"));
        assert_eq!(phone_country("+33712345678", None), Some("France"));
        assert_eq!(phone_country("+4915123456789", None), Some("Germany"));
        assert_eq!(phone_country("+4916123456789", None), Some("Germany"));
        assert_eq!(phone_country("+27721234567", None), Some("South Africa"));
    }

    #[test]
    fn phone_validation_rejects_wrong_length() {
        assert!(!is_phone("+141555526"));
        assert!(!is_phone("+141555526711"));
        assert!(!is_phone("+98912345678"));
        assert!(!is_phone("+9891234567890"));
    }

    #[test]
    fn all_csv_rows_are_loaded() {
        // A malformed or silently skipped row would make these counts disagree.
        let csv = include_str!("data/ph.csv");
        let data_rows = csv
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count()
            .saturating_sub(1);
        assert_eq!(PHONE_DATA.len(), data_rows, "some CSV rows were dropped");
    }

    #[test]
    fn dataset_schema_is_valid() {
        use std::collections::HashSet;

        for row in PHONE_DATA.iter() {
            let name = &row.english_name;
            assert!(!name.trim().is_empty(), "row with empty English name");
            assert!(
                !row.persian_name.trim().is_empty(),
                "{name}: empty Persian name"
            );
            assert!(
                row.min_len >= 4,
                "{name}: min_len {} is too small",
                row.min_len
            );
            assert!(row.min_len <= row.max_len, "{name}: min_len > max_len");
            assert!(
                row.max_len <= 15,
                "{name}: max_len {} exceeds the E.164 limit",
                row.max_len
            );

            let mut seen = HashSet::new();
            for prefix in &row.prefixes {
                assert!(
                    prefix.chars().all(|c| c.is_ascii_digit()),
                    "{name}: non-digit prefix '{prefix}'"
                );
                assert!(
                    !prefix.starts_with('0'),
                    "{name}: trunk prefix '{prefix}' must be stripped"
                );
                assert!(
                    prefix.len() <= row.min_len,
                    "{name}: prefix '{prefix}' longer than min_len {}",
                    row.min_len
                );
                assert!(seen.insert(prefix), "{name}: duplicate prefix '{prefix}'");
            }
        }
    }

    #[test]
    fn every_start_string_maps_to_one_country() {
        use std::collections::HashMap;

        let mut owners: HashMap<String, &str> = HashMap::new();
        for row in PHONE_DATA.iter() {
            let starts: Vec<String> = if row.prefixes.is_empty() {
                vec![row.code.clone()]
            } else {
                row.prefixes
                    .iter()
                    .map(|prefix| format!("{}{}", row.code, prefix))
                    .collect()
            };
            for start in starts {
                if let Some(previous) = owners.insert(start.clone(), &row.english_name) {
                    panic!(
                        "start string '{start}' is shared by '{previous}' and '{}'",
                        row.english_name
                    );
                }
            }
        }
    }

    #[test]
    fn every_row_prefix_produces_a_valid_number() {
        // Smoke test: for each row, build a shortest valid number from its prefix
        // and make sure at least one row accepts it.
        for row in PHONE_DATA.iter() {
            let target_total = row.code.len() + row.min_len;
            let prefixes: Vec<&str> = if row.prefixes.is_empty() {
                vec![""]
            } else {
                row.prefixes.iter().map(String::as_str).collect()
            };
            for prefix in prefixes {
                let mut number = format!("{}{prefix}", row.code);
                if number.len() > target_total {
                    continue;
                }
                number.push_str(&"9".repeat(target_total - number.len()));
                assert!(
                    is_phone(&number),
                    "generated number for '{}' should validate: {number}",
                    row.english_name
                );
            }
        }
    }

    #[test]
    fn nanp_countries_are_disambiguated() {
        // Non-US members of the North American Numbering Plan are matched by area code.
        assert_eq!(phone_country("+14165550123", None), Some("Canada"));
        assert_eq!(phone_country("+16045551234", None), Some("Canada"));
        assert_eq!(phone_country("+19025551234", None), Some("Canada"));
        assert_eq!(phone_country("+18766543210", None), Some("Jamaica"));
        assert_eq!(
            phone_country("+18095551234", None),
            Some("Dominican Republic")
        );
        assert_eq!(phone_country("+17875551234", None), Some("Puerto Rico"));
        assert_eq!(
            phone_country("+18685551234", None),
            Some("Trinidad and Tobago")
        );
        assert_eq!(phone_country("+12465551234", None), Some("Barbados"));
        assert_eq!(phone_country("+12425551234", None), Some("Bahamas"));
        // +1 numbers whose area code is not assigned abroad fall back to the US.
        assert_eq!(phone_country("+14155552671", None), Some("United States"));
    }

    #[test]
    fn corrected_rows_map_to_expected_country() {
        // Mexico: uniform 10 digits; the legacy mobile "1" prefix was abolished in 2020.
        assert_eq!(phone_country("+525512345678", None), Some("Mexico"));
        // Gabon (7-digit NSN) / Congo (9-digit NSN): the trunk "0" is not part of the NSN.
        assert_eq!(phone_country("+2416123456", None), Some("Gabon"));
        assert_eq!(phone_country("+242061234567", None), Some("Congo"));
        // Ivory Coast expanded its NSN from 8 to 10 digits in 2021.
        assert_eq!(phone_country("+2250701234567", None), Some("Ivory Coast"));
        // Russia's mobile block is +7 9xx; Kazakhstan uses the +7 7xx block.
        assert_eq!(phone_country("+79123456789", None), Some("Russia"));
        assert_eq!(phone_country("+77012345678", None), Some("Kazakhstan"));
    }

    #[test]
    fn phone_country_lookup() {
        assert_eq!(phone_country("+989123456789", None), Some("Iran"));
        assert_eq!(
            phone_country("+989123456789", Some(CountryName::Persian)),
            Some("ایران")
        );
        assert_eq!(phone_country("+979123456789", None), None);
    }

    #[test]
    fn email_validation() {
        assert!(is_email("me@gmail.com"));
        assert!(is_email("first.last+tag@sub.example.co"));
        assert!(!is_email("example.com"));
        assert!(!is_email(""));
        assert!(!is_email("me@.com"));
    }
}
