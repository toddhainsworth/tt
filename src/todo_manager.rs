use crate::models::todo::Todo;

pub struct TodoManager {
    todos: Vec<Todo>,
}

impl TodoManager {
    pub fn new() -> Self {
        Self { todos: Vec::new() }
    }

    pub fn add_todo(&mut self, title: String) -> Todo {
        let todo = Todo::new(title);
        let todo_clone = todo.clone();
        self.todos.push(todo);
        todo_clone
    }

    pub fn list_todos(&self) -> Vec<Todo> {
        self.todos.clone()
    }

    pub fn mark_completed(&mut self, id: usize) -> Result<(), String> {
        if id >= self.todos.len() {
            return Err(format!("Todo with id {} not found", id));
        }
        self.todos[id].set_completed(true);
        Ok(())
    }

    pub fn mark_incomplete(&mut self, id: usize) -> Result<(), String> {
        if id >= self.todos.len() {
            return Err(format!("Todo with id {} not found", id));
        }
        self.todos[id].set_completed(false);
        Ok(())
    }

    pub fn toggle_completed(&mut self, id: usize) -> Result<(), String> {
        if id >= self.todos.len() {
            return Err(format!("Todo with id {} not found", id));
        }
        self.todos[id].toggle_completed();
        Ok(())
    }

    pub fn delete_todo(&mut self, id: usize) -> Result<(), String> {
        if id >= self.todos.len() {
            return Err(format!("Todo with id {} not found", id));
        }
        self.todos.remove(id);
        Ok(())
    }

    pub fn get_todo(&self, id: usize) -> Option<&Todo> {
        self.todos.get(id)
    }

    pub fn count(&self) -> usize {
        self.todos.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo_manager() {
        let manager = TodoManager::new();
        assert_eq!(manager.count(), 0);
        assert!(manager.list_todos().is_empty());
    }

    #[test]
    fn test_add_todo() {
        let mut manager = TodoManager::new();
        let todo = manager.add_todo("Test todo".to_string());
        
        assert_eq!(todo.title, "Test todo");
        assert_eq!(todo.completed, false);
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_list_todos() {
        let mut manager = TodoManager::new();
        manager.add_todo("Todo 1".to_string());
        manager.add_todo("Todo 2".to_string());
        
        let todos = manager.list_todos();
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].title, "Todo 1");
        assert_eq!(todos[1].title, "Todo 2");
    }

    #[test]
    fn test_mark_completed() {
        let mut manager = TodoManager::new();
        manager.add_todo("Test todo".to_string());
        
        // Mark as completed
        assert!(manager.mark_completed(0).is_ok());
        assert!(manager.get_todo(0).unwrap().completed);
        
        // Try to mark non-existent todo
        assert!(manager.mark_completed(1).is_err());
    }

    #[test]
    fn test_mark_incomplete() {
        let mut manager = TodoManager::new();
        manager.add_todo("Test todo".to_string());
        
        // Mark as completed first
        manager.mark_completed(0).unwrap();
        assert!(manager.get_todo(0).unwrap().completed);
        
        // Mark as incomplete
        assert!(manager.mark_incomplete(0).is_ok());
        assert!(!manager.get_todo(0).unwrap().completed);
        
        // Try to mark non-existent todo
        assert!(manager.mark_incomplete(1).is_err());
    }

    #[test]
    fn test_toggle_completed() {
        let mut manager = TodoManager::new();
        manager.add_todo("Test todo".to_string());
        
        // Initially false
        assert!(!manager.get_todo(0).unwrap().completed);
        
        // Toggle to true
        assert!(manager.toggle_completed(0).is_ok());
        assert!(manager.get_todo(0).unwrap().completed);
        
        // Toggle back to false
        assert!(manager.toggle_completed(0).is_ok());
        assert!(!manager.get_todo(0).unwrap().completed);
        
        // Try to toggle non-existent todo
        assert!(manager.toggle_completed(1).is_err());
    }

    #[test]
    fn test_delete_todo() {
        let mut manager = TodoManager::new();
        manager.add_todo("Todo 1".to_string());
        manager.add_todo("Todo 2".to_string());
        
        assert_eq!(manager.count(), 2);
        
        // Delete first todo
        assert!(manager.delete_todo(0).is_ok());
        assert_eq!(manager.count(), 1);
        assert_eq!(manager.get_todo(0).unwrap().title, "Todo 2");
        
        // Try to delete non-existent todo
        assert!(manager.delete_todo(1).is_err());
    }

    #[test]
    fn test_get_todo() {
        let mut manager = TodoManager::new();
        manager.add_todo("Test todo".to_string());
        
        // Get existing todo
        let todo = manager.get_todo(0);
        assert!(todo.is_some());
        assert_eq!(todo.unwrap().title, "Test todo");
        
        // Get non-existent todo
        let todo = manager.get_todo(1);
        assert!(todo.is_none());
    }

    #[test]
    fn test_count() {
        let mut manager = TodoManager::new();
        assert_eq!(manager.count(), 0);
        
        manager.add_todo("Todo 1".to_string());
        assert_eq!(manager.count(), 1);
        
        manager.add_todo("Todo 2".to_string());
        assert_eq!(manager.count(), 2);
        
        manager.delete_todo(0).unwrap();
        assert_eq!(manager.count(), 1);
    }
} 