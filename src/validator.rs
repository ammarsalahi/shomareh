use csv::ReaderBuilder;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

const EN_DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const FA_DIGITS: [char; 10] = ['۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹'];

// convert persian number characters to english
pub fn to_english(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            FA_DIGITS
                .iter()
                .position(|&d| d == c)
                .map(|i| EN_DIGITS[i])
                .unwrap_or(c)
        })
        .collect()
}

// convert english number characters to persian
pub fn to_persian(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            EN_DIGITS
                .iter()
                .position(|&d| d == c)
                .map(|i| FA_DIGITS[i])
                .unwrap_or(c)
        })
        .collect()
}

// to get email validation
pub fn is_email(email: &str) -> bool {
    if email.is_empty() {
        return false;
    }
    let email_rgx = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_rgx.is_match(email)
}

#[derive(Debug, Deserialize, Clone)]
struct Phone {
    code: i32,
    min_len: i32,
    max_len: i32,
    prefixes: Option<i32>,
    english_name: String,
    persian_name: String,
}

impl Phone {
    fn start_str(&self) -> String {
        match self.prefixes {
            Some(p) => format!("{}{}", self.code, p),
            None => self.code.to_string(),
        }
    }
}

pub enum CountryName {
    English,
    Persian,
}

static PHONE_DATA: Lazy<Vec<Phone>> = Lazy::new(|| {
    let mut data_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("./src/data/ph.csv")
        .expect("Failed to read phone data");
    let mut phones: Vec<Phone> = data_reader
        .deserialize::<Phone>()
        .filter_map(|r| {
            r.map_err(|e| {
                eprintln!("warning: bad row in csv:{e}!");
            })
            .ok()
        })
        .collect();
    // Sort by longest prefix first to handle overlaps correctly
    phones.sort_by(|a, b| b.start_str().len().cmp(&a.start_str().len()));
    phones
});

static PHONE_PREFIX_REGEX: Lazy<Regex> = Lazy::new(|| {
    let patterns = PHONE_DATA.iter().map(|p| p.start_str()).collect::<Vec<_>>();
    Regex::new(&format!("^({})\\d+$", patterns.join("|"))).expect("Invalid regex")
});

fn normalize_phone(phone: &str) -> String {
    let with_english = to_english(phone);
    with_english
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect()
}
// to get phone regex validation
pub fn is_phone(phone: &str) -> bool {
    let phone = normalize_phone(phone);
    if !PHONE_PREFIX_REGEX.is_match(&phone) {
        return false;
    }
    for row in PHONE_DATA.iter() {
        let start = row.start_str();
        if phone.starts_with(&start) {
            let len = phone.len() as i32; // FIXED: use phone length, not row.len()
            if len >= row.min_len && len <= row.max_len {
                return true;
            }
        }
    }
    false
}

// to get phone regex validation and phone country code
pub fn get_phone_validation(phone: &str, name: Option<CountryName>) -> (bool, &str) {
    let phone = normalize_phone(phone);
    if !PHONE_PREFIX_REGEX.is_match(&phone) {
        return (false, "");
    }
    for row in PHONE_DATA.iter() {
        let start = row.start_str();
        if phone.starts_with(&start) {
            let len = phone.len() as i32; // FIXED: use phone length
            if len >= row.min_len && len <= row.max_len {
                let cntry_code = match name {
                    Some(CountryName::English) => &row.english_name,
                    Some(CountryName::Persian) => &row.persian_name,
                    None => &row.english_name,
                };
                return (true, cntry_code);
            }
        }
    }
    (false, "")
}
