pub mod datetime;
pub mod validator;

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Local, Timelike};

    use crate::{
        datetime::{DateTime, DateTimeValidator, JDateTime, to_gregorian, to_jalali},
        validator::to_english,
    };

    use super::*;

    #[test]
    fn test_convertion() {
        assert_eq!(validator::to_persian("+1234"), "+۱۲۳۴");
        assert_eq!(validator::to_english("۱۲۳۴۵۶۷۸۹۰"), "1234567890");
    }

    #[test]
    fn test_validation() {
        assert_eq!(validator::is_phone("+989123456789"), true);
        assert_eq!(validator::is_phone("123456789"), false);
        assert_eq!(
            validator::get_phone_validation("+989123456789", None),
            (true, "Iran")
        );
        assert_eq!(validator::is_email("me@gmail.com"), true);
        assert_eq!(validator::is_email("example.com"), false);
        assert_ne!(
            validator::get_phone_validation("+979123456789", None),
            (true, "Iran")
        );
    }

    #[test]
    fn test_valiation_year() {
        assert!(<DateTime as DateTimeValidator>::validate_year("2026").is_ok());
        assert!(<DateTime as DateTimeValidator>::validate_year("1400").is_ok());
        assert!(<DateTime as DateTimeValidator>::validate_year("0001").is_ok());
        assert!(<DateTime as DateTimeValidator>::validate_year("1").is_ok());

        assert!(<DateTime as DateTimeValidator>::validate_year("").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_year("year").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_year("-1").is_err());
    }

    #[test]
    fn test_valiation_month() {
        assert!(<DateTime as DateTimeValidator>::validate_month("01").is_ok());
        assert!(<DateTime as DateTimeValidator>::validate_month("12").is_ok());
        assert!(<DateTime as DateTimeValidator>::validate_month("07").is_ok());

        assert!(<DateTime as DateTimeValidator>::validate_month("").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_month("month").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_month("-1").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_month("7").is_err());
    }
    #[test]
    fn test_valiation_day() {
        assert!(<DateTime as DateTimeValidator>::validate_day("01").is_ok());
        assert!(<DateTime as DateTimeValidator>::validate_day("12").is_ok());

        assert!(<DateTime as DateTimeValidator>::validate_day("").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_day("day").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_day("-1").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_day("7").is_err());
    }
    #[test]
    fn test_valiation_hour() {
        assert!(<DateTime as DateTimeValidator>::validate_hour("01").is_ok());
        assert!(<DateTime as DateTimeValidator>::validate_hour("12").is_ok());
        assert!(<DateTime as DateTimeValidator>::validate_hour("23").is_ok());

        assert!(<DateTime as DateTimeValidator>::validate_hour("").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_hour("month").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_hour("-1").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_hour("45").is_err());
    }

    #[test]
    fn test_valiation_minute() {
        assert!(<DateTime as DateTimeValidator>::validate_minute("01").is_ok());
        assert!(<DateTime as DateTimeValidator>::validate_minute("30").is_ok());

        assert!(<DateTime as DateTimeValidator>::validate_minute("").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_minute("month").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_minute("-1").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_minute("7").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_minute("67").is_err());
    }

    #[test]
    fn test_valiation_second() {
        assert!(<DateTime as DateTimeValidator>::validate_second("01").is_ok());
        assert!(<DateTime as DateTimeValidator>::validate_second("47").is_ok());

        assert!(<DateTime as DateTimeValidator>::validate_second("").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_second("month").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_second("-1").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_second("7").is_err());
        assert!(<DateTime as DateTimeValidator>::validate_minute("77").is_err());
    }

    #[test]
    fn test_parse_validate_date() {
        let date = DateTime::parse_validate_date("۲۰۲۶/۰۱/۰۱");
        assert!(date.is_ok());

        let (y, m, d) = date.unwrap();

        assert_eq!(y, to_english("2026"));
        assert_eq!(m, to_english("01"));
        assert_eq!(d, to_english("01"));
    }

    #[test]
    fn test_parse_validate_datetime() {
        let date = DateTime::parse_validate_datetime("۲۰۲۶/۰۱/۰۱ ۱۲:۱۲:۱۲").unwrap();

        assert_eq!(date.year(), to_english("2026"));
        assert_eq!(date.month(), to_english("01"));
        assert_eq!(date.day(), to_english("01"));
        assert_eq!(date.hour(), Some(to_english("12")));
        assert_eq!(date.minute(), Some(to_english("12")));
        assert_eq!(date.second(), Some(to_english("12")));
    }

    #[test]
    fn test_parse_validate_time() {
        let (hour, minute, second) = DateTime::parse_validate_time("۱۲:۱۲:۱۲").unwrap();
        assert_eq!(hour, to_english("12"));
        assert_eq!(minute, to_english("12"));
        assert_eq!(second, to_english("12"));
    }

    #[test]
    fn test_set_date() {
        let date = DateTime::set_date("2026/05/17").unwrap();
        assert_eq!(date.year(), to_english("2026"));
        assert_eq!(date.month(), to_english("05"));
        assert_eq!(date.day(), to_english("17"));
    }

    #[test]
    fn test_set_times() {
        let times = DateTime::set_time("12:12:12").unwrap();
        assert_eq!(times.hour(), Some(to_english("12")));
        assert_eq!(times.minute(), Some(to_english("12")));
        assert_eq!(times.second(), Some(to_english("12")));
    }

    #[test]
    fn test_now() {
        let now_dt = DateTime::now();
        let local_now = Local::now();

        assert_eq!(now_dt.year(), format!("{}", local_now.year()));
        assert_eq!(now_dt.month(), format!("{}", local_now.month()));
        assert_eq!(now_dt.day(), format!("{}", local_now.day()));
        assert_eq!(now_dt.hour(), Some(format!("{}", local_now.hour())));
        assert_eq!(now_dt.minute(), Some(format!("{}", local_now.minute())));
        assert_eq!(now_dt.second(), Some(format!("{}", local_now.second())));
    }

    #[test]
    fn test_date_convertion() {
        let grn_date = DateTime::set_date("2026/05/12").unwrap();

        let jalai = to_jalali(&grn_date).unwrap();

        assert_eq!(jalai.year(), String::from("1405"));
        assert_eq!(jalai.month(), String::from("2"));
        assert_eq!(jalai.day(), String::from("22"));

        let jal_date = JDateTime::set_date("1405/02/22").unwrap();

        let grn = to_gregorian(&jal_date).unwrap();

        assert_eq!(grn.year(), String::from("2026"));
        assert_eq!(grn.month(), String::from("5"));
        assert_eq!(grn.day(), String::from("12"));
    }
}
