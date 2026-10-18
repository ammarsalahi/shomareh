use shomareh::{
    CountryName, DateTime, DateTimeError, Gregorian, Jalali, is_email, is_jalali_leap, is_phone,
    phone_country, to_english, to_persian,
};

#[test]
fn readme_snippets() {
    let jalali = DateTime::<Jalali>::set_date("۱۴۰۵/۰۲/۲۲").unwrap();
    assert_eq!(jalali.to_string(), "1405-02-22");

    let greg = DateTime::<Gregorian>::set_date("2026/05/12").unwrap();
    assert_eq!(greg.to_jalali().unwrap().to_string(), "1405-02-22");
    assert_eq!(jalali.to_gregorian().unwrap().to_string(), "2026-05-12");

    assert!(is_phone("+989123456789"));
    assert!(is_phone("۰۰۹۸۹۱۲۳۴۵۶۷۸۹"));
    assert_eq!(phone_country("+989123456789", None), Some("Iran"));
    assert!(is_email("me@gmail.com"));

    let dt = DateTime::<Gregorian>::set_datetime("2026/05/12 23:05:09").unwrap();
    assert_eq!(dt.hour(), Some("23"));

    let today_jalali = DateTime::<Jalali>::now().unwrap();
    assert!(!today_jalali.year().is_empty());
    let at_noon = DateTime::<Gregorian>::set_time("12:00:00").unwrap();
    assert_eq!(at_noon.hour(), Some("12"));

    assert!(DateTime::<Jalali>::new(1403, 12, 30).is_ok());
    assert!(DateTime::<Jalali>::new(1404, 12, 30).is_err());

    let dt = DateTime::<Jalali>::set_datetime("۱۴۰۵/۰۲/۲۲ ۰۹:۰۵:۰۰").unwrap();
    assert_eq!(dt.year(), "1405");
    assert_eq!(dt.month(), "02");
    assert_eq!(dt.day(), "22");
    assert_eq!(dt.hour(), Some("09"));
    assert_eq!(dt.to_string(), "1405-02-22 09:05:00");

    assert!(is_jalali_leap(1403));
    assert!(!is_jalali_leap(1404));

    assert_eq!(to_english("۱۴۰۵"), "1405");
    assert_eq!(to_persian("1405"), "۱۴۰۵");
    assert_eq!(to_english("١٤٠٥"), "1405");
    assert_eq!(to_persian("١٤٠٥"), "۱۴۰۵");
    assert_eq!(to_english("قیمت: ۱۲۳"), "قیمت: 123");

    assert!(is_phone("0098 912 345 6789"));
    assert!(!is_phone("123456789"));
    assert_eq!(
        phone_country("+989123456789", Some(CountryName::Persian)),
        Some("ایران")
    );
    assert_eq!(phone_country("+999999999999", None), None);

    assert!(is_email("me@gmail.com"));
    assert!(!is_email("example.com"));
    assert!(!is_email(""));

    match DateTime::<Gregorian>::set_date("2026/02/30") {
        Err(e @ DateTimeError::InvalidDay(_)) => assert!(e.to_string().contains("Invalid Day")),
        other => panic!("expected InvalidDay, got {other:?}"),
    }
}
