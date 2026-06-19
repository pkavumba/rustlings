// Parse JSON, use iterators, and serialize a summary back to JSON.

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Task {
    title: String,
    completed: bool,
    tags: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize)]
struct TaskSummary {
    total: usize,
    completed: usize,
    open: usize,
    unique_tags: Vec<String>,
}

fn summarize_tasks(input: &str) -> serde_json::Result<String> {
    let tasks: Vec<Task> = serde_json::from_str(input)?;

    // TODO: Calculate these values from `tasks`.
    let summary = TaskSummary {
        total: 0,
        completed: 0,
        open: 0,
        unique_tags: Vec::new(),
    };

    serde_json::to_string(&summary)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_tasks_as_json() {
        let output = summarize_tasks(
            r#"[
                {"title":"read","completed":true,"tags":["rust","book"]},
                {"title":"practice","completed":false,"tags":["rust"]},
                {"title":"ship","completed":false,"tags":["project"]}
            ]"#,
        )
        .unwrap();

        assert_eq!(
            output,
            r#"{"total":3,"completed":1,"open":2,"unique_tags":["book","project","rust"]}"#,
        );
    }
}
