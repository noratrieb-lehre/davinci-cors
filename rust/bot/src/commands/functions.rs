//! This module contains *pure* functions handling command logic.
//! This makes parts of this bot easily testable

use chrono::{DateTime, Datelike, FixedOffset, NaiveDateTime, TimeZone, Utc, Weekday};
use dto::Timetable;

pub fn from_utc_timestamp(time: i64) -> DateTime<Utc> {
    Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(time / 1000, 0))
}

pub fn from_utc_to_cest(utc: DateTime<Utc>) -> DateTime<FixedOffset> {
    chrono::FixedOffset::east(2 * 3600).from_utc_datetime(&utc.naive_utc())
}

pub fn absolute_time_as_weekday(now: chrono::DateTime<Utc>) -> (i64, Weekday) {
    let day_0 = now.date().and_hms(0, 0, 0);
    let diff = (now - day_0).num_milliseconds();
    let weekday = now.weekday();

    (diff, weekday)
}

pub fn wie_lange_noch(
    table: &Timetable,
    time: i64,
    day_of_week: Weekday,
) -> (Option<&dto::Lesson>, Option<&dto::Lesson>) {
    let day_table = &table[day_of_week as usize]; // Will never panic
    let cur_lesson = day_table
        .iter()
        .find(|lesson| lesson.start < time && lesson.end > time);

    let next = day_table.iter().find(|lesson| {
        (cur_lesson.map(|ls| ls.start) < Some(lesson.start)) || (lesson.start > time)
    });
    (cur_lesson, next)
}

#[cfg(test)]
mod test {
    use chrono::*;
    use dto::{Lesson, Timetable};

    fn monday_10am() -> DateTime<Utc> {
        Utc.datetime_from_str("2021-07-26 10:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap()
    }

    fn monday_1s() -> DateTime<Utc> {
        Utc.datetime_from_str("2021-07-26 00:00:01", "%Y-%m-%d %H:%M:%S")
            .unwrap()
    }

    #[test]
    fn weekday_monday() {
        let day = monday_10am();

        let (timestamp, weekday) = super::absolute_time_as_weekday(day);
        assert_eq!(weekday, Weekday::Mon);
        let ten_am = 10 * 60 * 60 * 1000;
        assert_eq!(timestamp, ten_am);
    }

    #[test]
    fn wielangenoch_no_lesson() {
        let timetable: Timetable = [vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
        let (timestamp, weekday) = super::absolute_time_as_weekday(monday_10am());

        let (lesson, _) = super::wie_lange_noch(&timetable, timestamp, weekday);
        assert_eq!(None, lesson);
    }

    #[test]
    fn wielangenoch_in_lesson() {
        let target_lesson = Lesson {
            subject: "schaffen".to_string(),
            description: "none".to_string(),
            start: 500,
            end: 1500,
        };
        let next_lesson = Lesson {
            subject: "schaffen".to_string(),
            description: "none".to_string(),
            start: 1600,
            end: 2000,
        };
        let timetable: Timetable = [
            vec![target_lesson.clone(), next_lesson.clone()],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];
        let (timestamp, weekday) = super::absolute_time_as_weekday(monday_1s());
        let (lesson, next) = super::wie_lange_noch(&timetable, timestamp, weekday);

        assert_eq!(timestamp, 1000);
        assert_eq!(weekday, Weekday::Mon);
        assert_eq!(Some(&target_lesson), lesson);
        assert_eq!(Some(&next_lesson), next);
    }

    #[test]
    fn wielangenoch_next_lesson() {
        let target_lesson = Lesson {
            subject: "schaffen".to_string(),
            description: "none".to_string(),
            start: 1500,
            end: 2000,
        };
        let timetable: Timetable = [
            vec![target_lesson.clone()],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];
        let (timestamp, weekday) = super::absolute_time_as_weekday(monday_1s());
        let (lesson, next) = super::wie_lange_noch(&timetable, timestamp, weekday);

        assert_eq!(timestamp, 1000);
        assert_eq!(weekday, Weekday::Mon);
        assert_eq!(None, lesson);
        assert_eq!(Some(&target_lesson), next);
    }
}
