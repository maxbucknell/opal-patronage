use chrono::{Duration, Utc, TimeZone, DurationRound, DateTime, LocalResult};
use chrono_tz::{Australia, Tz};
use regex::Regex;

use crate::error::{Error, ErrorKind};

pub fn get_bounding_times(now: &DateTime<Utc>) -> (DateTime<Tz>, DateTime<Tz>) {
    let today = now.with_timezone(&Australia::NSW).duration_trunc(Duration::days(1)).unwrap();
    let min_start = Australia::NSW.with_ymd_and_hms(2020, 01, 01, 0, 0, 0).unwrap();
    let max_end = today - Duration::days(5);

    (min_start, max_end)
}

pub fn parse_and_clamp(min: DateTime<Tz>, max: DateTime<Tz>, date: &str) -> Result<DateTime<Tz>, Error> {
    parse_date(date).and_then(|d| Ok(d.clamp(min, max)))
}

fn parse_date(s: &str) -> Result<DateTime<Tz>, Error> {
    let [y, m, d] = date_string_to_components(s)
        .and_then(|c| components_to_numbers(c))?;

    let date = Australia::NSW.with_ymd_and_hms(y as i32, m as u32, d as u32, 0, 0, 0);

    match date {
        LocalResult::Single(d) => Ok(d),
        _ => Err(Error::new(ErrorKind::NoSingleDate(y, m, d)))
    }
}

fn date_string_to_components(s: &str) -> Result<[&str; 3], Error> {
    let re = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$").unwrap();
    let date = re.captures(s);

    match date {
        None => Err(Error::new(ErrorKind::CouldNotParseDate(s.to_string()))),
        Some(matches) => {
            let y = &matches.get(1).unwrap().as_str();
            let m = &matches.get(2).unwrap().as_str();
            let d = &matches.get(3).unwrap().as_str();

            Ok([y, m, d])
        }
    }
}

fn components_to_numbers(c: [&str; 3]) -> Result<[u16; 3], Error> {
    let mut r = [0, 0, 0];
    for i in 0..3 {
        let parsed = c[i].parse::<u16>();

        match parsed {
            Ok(n) => {
                r[i] = n;
            },
            Err(_) => {
                return Err(Error::new(ErrorKind::CouldNotParseNumericComponent(i, c[i].to_string())));
            }
        }
    }

    if r[1] < 1 || r[1] > 12 {
        Err(Error::new(ErrorKind::InvalidMonth(r[1])))
    } else if r[2] < 1 || r[2] > days_in_month(r[0], r[1]) {
        Err(Error::new(ErrorKind::InvalidDateForMonth(r[1], r[2])))
    } else {
        Ok(r)
    }
}

fn days_in_month(y: u16, m: u16) -> u16 {
    match m {
        2 => {
            if y % 400 == 0 {
                29
            } else if y % 100 == 0 {
                28
            } else if y % 4 == 0 {
                29
            } else {
                28
            }
        }
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => 30
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date() {
        assert_eq!(parse_date("2000-01-01").unwrap().to_rfc3339(), "2000-01-01T00:00:00+11:00");
        assert_eq!(parse_date("2023-11-04").unwrap().to_rfc3339(), "2023-11-04T00:00:00+11:00");
        assert_eq!(parse_date("2022-07-23").unwrap().to_rfc3339(), "2022-07-23T00:00:00+10:00");

        assert!(parse_date("2022-17-23").is_err());
    }

    #[test]
    fn test_date_string_to_components() {
        assert_eq!(date_string_to_components("1988-04-20").unwrap(), ["1988", "04", "20"]);
        assert_eq!(date_string_to_components("2000-02-01").unwrap(), ["2000", "02", "01"]);
        assert_eq!(date_string_to_components("1969-12-31").unwrap(), ["1969", "12", "31"]);

        assert!(date_string_to_components("2002-123-19").is_err());
        assert!(date_string_to_components("hello, world").is_err());
    }

    #[test]
    fn test_components_to_numbers() {
        assert_eq!(components_to_numbers(["2023", "11", "30"]).unwrap(), [2023, 11, 30]);
        assert_eq!(components_to_numbers(["1998", "12", "31"]).unwrap(), [1998, 12, 31]);
        assert_eq!(components_to_numbers(["0000", "01", "01"]).unwrap(), [0, 1, 1]);

        assert!(components_to_numbers(["0000", "00", "01"]).is_err());
        assert!(components_to_numbers(["1998", "12", "32"]).is_err());
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(2023, 1), 31);
        assert_eq!(days_in_month(2023, 2), 28);
        assert_eq!(days_in_month(2023, 3), 31);
        assert_eq!(days_in_month(2023, 4), 30);
        assert_eq!(days_in_month(2023, 5), 31);
        assert_eq!(days_in_month(2023, 6), 30);
        assert_eq!(days_in_month(2023, 7), 31);
        assert_eq!(days_in_month(2023, 8), 31);
        assert_eq!(days_in_month(2023, 9), 30);
        assert_eq!(days_in_month(2023, 10), 31);
        assert_eq!(days_in_month(2023, 11), 30);
        assert_eq!(days_in_month(2023, 12), 31);

        assert_eq!(days_in_month(2020, 2), 29);
        assert_eq!(days_in_month(2000, 2), 29);
        assert_eq!(days_in_month(2100, 2), 28);
    }
}
