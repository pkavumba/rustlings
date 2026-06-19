// Work with dates using `chrono` and classify task deadlines.

use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Task {
    title: String,
    due: NaiveDate,
    completed: bool,
}

#[derive(Debug, PartialEq)]
enum DeadlineStatus {
    Completed,
    Overdue,
    DueToday,
    Upcoming,
}

fn deadline_status(task: &Task, today: NaiveDate) -> DeadlineStatus {
    if task.completed {
        DeadlineStatus::Completed
    } else if task.due < today {
        DeadlineStatus::Overdue
    } else if task.due == today {
        DeadlineStatus::DueToday
    } else {
        DeadlineStatus::Upcoming
    }
}

fn parse_task(input: &str) -> serde_json::Result<Task> {
    serde_json::from_str(input)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn today() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 6, 19).unwrap()
    }

    #[test]
    fn parses_iso_date_from_json() {
        let task = parse_task(r#"{"title":"ship","due":"2026-06-19","completed":false}"#).unwrap();
        assert_eq!(task.due, today());
    }

    #[test]
    fn classifies_deadlines() {
        let overdue =
            parse_task(r#"{"title":"old","due":"2026-06-18","completed":false}"#).unwrap();
        let due_today =
            parse_task(r#"{"title":"now","due":"2026-06-19","completed":false}"#).unwrap();
        let upcoming =
            parse_task(r#"{"title":"next","due":"2026-06-20","completed":false}"#).unwrap();
        let completed =
            parse_task(r#"{"title":"done","due":"2026-06-18","completed":true}"#).unwrap();

        assert_eq!(deadline_status(&overdue, today()), DeadlineStatus::Overdue);
        assert_eq!(
            deadline_status(&due_today, today()),
            DeadlineStatus::DueToday
        );
        assert_eq!(
            deadline_status(&upcoming, today()),
            DeadlineStatus::Upcoming
        );
        assert_eq!(
            deadline_status(&completed, today()),
            DeadlineStatus::Completed
        );
    }
}
