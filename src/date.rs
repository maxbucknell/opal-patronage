use chrono::{Duration, Utc, TimeZone, DurationRound, DateTime, LocalResult};
use chrono_tz::{Australia, Tz};
use regex::Regex;

use crate::error::{Error, ErrorKind};

pub fn parse_and_clamp(min: DateTime<Tz>, max: DateTime<Tz>, date: &str) -> Result<DateTime<Tz>, Error> {
    parse_date(date).and_then(|d| Ok(d.clamp(min, max)))
}

pub fn get_bounding_times(now: &DateTime<Utc>) -> (DateTime<Tz>, DateTime<Tz>) {
    let today = now.with_timezone(&Australia::NSW).duration_trunc(Duration::days(1)).unwrap();
    let min_start = Australia::NSW.with_ymd_and_hms(2020, 01, 01, 0, 0, 0).unwrap();
    let max_end = today - Duration::days(5);

    (min_start, max_end)
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

fn parse_date(s: &str) -> Result<DateTime<Tz>, Error> {
    let [y, m, d] = date_string_to_components(s)
        .and_then(|c| components_to_numbers(c))?;

    let date = Australia::NSW.with_ymd_and_hms(y as i32, m as u32, d as u32, 0, 0, 0);

    match date {
        LocalResult::Single(d) => Ok(d),
        _ => Err(Error::new(ErrorKind::NoSingleDate(y, m, d)))
    }
}
