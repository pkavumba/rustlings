// Use `thiserror` to create ergonomic, typed application errors.

use thiserror::Error;

#[derive(Debug, Error)]
enum TaskError {
    #[error("task title cannot be empty")]
    EmptyTitle,
    #[error("invalid task id: {0}")]
    InvalidId(String),
    #[error("invalid task json")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, serde::Deserialize, PartialEq)]
struct Task {
    id: u32,
    title: String,
}

fn parse_task(input: &str) -> Result<Task, TaskError> {
    let task: Task = serde_json::from_str(input)?;

    if task.title.trim().is_empty() {
        return Err(TaskError::EmptyTitle);
    }

    Ok(task)
}

fn parse_id(input: &str) -> Result<u32, TaskError> {
    input
        .parse()
        .map_err(|_| TaskError::InvalidId(input.to_string()))
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_task() {
        assert_eq!(
            parse_task(r#"{"id":7,"title":"learn thiserror"}"#).unwrap(),
            Task {
                id: 7,
                title: "learn thiserror".to_string(),
            },
        );
    }

    #[test]
    fn rejects_empty_title() {
        assert!(matches!(
            parse_task(r#"{"id":7,"title":"   "}"#),
            Err(TaskError::EmptyTitle),
        ));
    }

    #[test]
    fn converts_json_errors() {
        assert!(matches!(parse_task("not json"), Err(TaskError::Json(_))));
    }

    #[test]
    fn reports_invalid_ids() {
        assert!(matches!(
            parse_id("abc"),
            Err(TaskError::InvalidId(id)) if id == "abc",
        ));
    }
}
