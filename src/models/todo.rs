use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
    pub created_at: String, // ISO 8601 format
}

impl Default for Todo {
    fn default() -> Self {
        let now: DateTime<Utc> = Utc::now();
        Self {
            title: String::new(),
            completed: false,
            created_at: now.to_rfc3339(),
        }
    }
}

impl Todo {
    pub fn new(title: String) -> Self {
        let now: DateTime<Utc> = Utc::now();
        Self {
            title,
            completed: false,
            created_at: now.to_rfc3339(),
        }
    }

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }

    pub fn set_completed(&mut self, value: bool) {
        self.completed = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo() {
        let title = "Test todo".to_string();
        let todo = Todo::new(title.clone());
        
        assert_eq!(todo.title, title);
        assert_eq!(todo.completed, false);
        assert!(!todo.created_at.is_empty());
    }

    #[test]
    fn test_toggle_completed() {
        let mut todo = Todo::new("Test".to_string());
        
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
        let mut todo = Todo::new("Test".to_string());
        
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
    fn test_default_todo() {
        let todo = Todo::default();
        
        assert_eq!(todo.title, "");
        assert_eq!(todo.completed, false);
        assert!(!todo.created_at.is_empty());
    }
} 