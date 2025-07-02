use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
    pub created_at: String, // ISO 8601 format
    #[serde(default = "default_priority")]
    pub priority: u8, // 1-4, where 1 is highest priority
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TodoStore {
    pub todos: Vec<Todo>,
}

fn default_priority() -> u8 {
    4 // Default to lowest priority for backward compatibility
}

impl Default for Todo {
    fn default() -> Self {
        let now: DateTime<Utc> = Utc::now();
        Self {
            title: String::new(),
            completed: false,
            created_at: now.to_rfc3339(),
            priority: default_priority(),
        }
    }
}

impl Todo {
    pub fn new(title: String, priority: u8) -> Result<Self, String> {
        Self::validate_priority(priority)?;
        let now: DateTime<Utc> = Utc::now();
        Ok(Self {
            title,
            completed: false,
            created_at: now.to_rfc3339(),
            priority,
        })
    }

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }

    pub fn set_completed(&mut self, value: bool) {
        self.completed = value;
    }

    pub fn set_priority(&mut self, priority: u8) -> Result<(), String> {
        Self::validate_priority(priority)?;
        self.priority = priority;
        Ok(())
    }

    pub fn validate_priority(priority: u8) -> Result<(), String> {
        if !(1..=4).contains(&priority) {
            return Err(format!("Priority must be between 1 and 4, got {priority}",));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo() {
        let title = "Test todo".to_string();
        let todo = Todo::new(title.clone(), 4).unwrap();

        assert_eq!(todo.title, title);
        assert_eq!(todo.completed, false);
        assert_eq!(todo.priority, 4);
        assert!(!todo.created_at.is_empty());
    }

    #[test]
    fn test_new_todo_with_priority() {
        let title = "Test todo".to_string();
        let todo = Todo::new(title.clone(), 1).unwrap();

        assert_eq!(todo.title, title);
        assert_eq!(todo.completed, false);
        assert_eq!(todo.priority, 1);
        assert!(!todo.created_at.is_empty());
    }

    #[test]
    fn test_new_todo_with_invalid_priority() {
        let title = "Test todo".to_string();
        assert!(Todo::new(title.clone(), 0).is_err());
        assert!(Todo::new(title, 5).is_err());
    }

    #[test]
    fn test_toggle_completed() {
        let mut todo = Todo::new("Test".to_string(), 4).unwrap();

        // Initially false
        assert_eq!(todo.completed, false);

        // Toggle to true
        todo.toggle_completed();
        assert_eq!(todo.completed, true);

        // Toggle back to false
        todo.toggle_completed();
        assert_eq!(todo.completed, false);
    }

    #[test]
    fn test_set_completed() {
        let mut todo = Todo::new("Test".to_string(), 4).unwrap();

        // Initially false
        assert_eq!(todo.completed, false);

        // Set to true
        todo.set_completed(true);
        assert_eq!(todo.completed, true);

        // Set to false
        todo.set_completed(false);
        assert_eq!(todo.completed, false);
    }

    #[test]
    fn test_set_priority() {
        let mut todo = Todo::new("Test".to_string(), 4).unwrap();
        assert_eq!(todo.priority, 4);

        // Set valid priorities
        assert!(todo.set_priority(1).is_ok());
        assert_eq!(todo.priority, 1);

        assert!(todo.set_priority(3).is_ok());
        assert_eq!(todo.priority, 3);

        // Set invalid priorities
        assert!(todo.set_priority(0).is_err());
        assert!(todo.set_priority(5).is_err());
        assert_eq!(todo.priority, 3); // Should remain unchanged
    }

    #[test]
    fn test_validate_priority() {
        // Valid priorities
        assert!(Todo::validate_priority(1).is_ok());
        assert!(Todo::validate_priority(2).is_ok());
        assert!(Todo::validate_priority(3).is_ok());
        assert!(Todo::validate_priority(4).is_ok());

        // Invalid priorities
        assert!(Todo::validate_priority(0).is_err());
        assert!(Todo::validate_priority(5).is_err());
        assert!(Todo::validate_priority(255).is_err());
    }

    #[test]
    fn test_default_todo() {
        let todo = Todo::default();

        assert_eq!(todo.title, "");
        assert_eq!(todo.completed, false);
        assert_eq!(todo.priority, 4);
        assert!(!todo.created_at.is_empty());
    }
}
