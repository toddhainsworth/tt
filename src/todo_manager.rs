use crate::models::todo::{Todo, TodoStore};
use std::fs;
use std::path::PathBuf;

pub struct TodoManager {
    todos: Vec<Todo>,
    file_path: PathBuf,
}

impl TodoManager {
    pub fn new() -> Result<Self, String> {
        let file_path = Self::get_file_path()?;
        let mut manager = Self {
            todos: Vec::new(),
            file_path,
        };

        // Try to load existing todos, but don't fail if file doesn't exist
        if let Err(e) = manager.load_from_file() {
            eprintln!("⚠️  Warning: Could not load existing todos: {e}");
            eprintln!("   Starting with empty todo list.");
        }

        Ok(manager)
    }

    fn get_file_path() -> Result<PathBuf, String> {
        dirs::home_dir()
            .ok_or_else(|| "Could not determine home directory".to_string())
            .map(|home| home.join(".tt.json"))
    }

    pub fn load_from_file(&mut self) -> Result<(), String> {
        if !self.file_path.exists() {
            return Ok(()); // File doesn't exist yet, that's fine
        }

        let content =
            fs::read_to_string(&self.file_path).map_err(|e| format!("Failed to read file: {e}"))?;

        let todo_store: TodoStore =
            serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {e}"))?;

        self.todos = todo_store.todos;
        Ok(())
    }

    pub fn save_to_file(&self) -> Result<(), String> {
        let todo_store = TodoStore {
            todos: self.todos.clone(),
        };

        let json = serde_json::to_string_pretty(&todo_store)
            .map_err(|e| format!("Failed to serialize todos: {e}"))?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {e}"))?;
        }

        fs::write(&self.file_path, json).map_err(|e| format!("Failed to write file: {e}"))?;

        // Set file permissions on Unix-like systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&self.file_path)
                .map_err(|e| format!("Failed to get file metadata: {e}"))?
                .permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&self.file_path, perms)
                .map_err(|e| format!("Failed to set file permissions: {e}"))?;
        }

        Ok(())
    }

    pub fn add_todo(&mut self, title: String, priority: u8) -> Result<Todo, String> {
        let todo = Todo::new(title, priority)?;
        let todo_clone = todo.clone();
        self.todos.push(todo);
        self.save_to_file()?;
        Ok(todo_clone)
    }

    pub fn edit_todo(
        &mut self,
        id: usize,
        title: Option<String>,
        priority: Option<u8>,
    ) -> Result<(), String> {
        if id >= self.todos.len() {
            return Err(format!("Todo with id {id} not found"));
        }
        if let Some(new_title) = title {
            self.todos[id].title = new_title;
        }
        if let Some(new_priority) = priority {
            self.todos[id].set_priority(new_priority)?;
        }
        self.save_to_file()
    }

    pub fn validate_priority(priority: u8) -> Result<(), String> {
        Todo::validate_priority(priority)
    }

    pub fn list_todos(&self) -> Vec<Todo> {
        self.todos.clone()
    }

    pub fn mark_completed(&mut self, id: usize) -> Result<(), String> {
        if id >= self.todos.len() {
            return Err(format!("Todo with id {id} not found"));
        }
        self.todos[id].set_completed(true);

        // Auto-save after modification
        self.save_to_file()
    }

    pub fn mark_incomplete(&mut self, id: usize) -> Result<(), String> {
        if id >= self.todos.len() {
            return Err(format!("Todo with id {id} not found"));
        }
        self.todos[id].set_completed(false);

        // Auto-save after modification
        self.save_to_file()
    }

    pub fn toggle_completed(&mut self, id: usize) -> Result<(), String> {
        if id >= self.todos.len() {
            return Err(format!("Todo with id {id} not found"));
        }
        self.todos[id].toggle_completed();

        // Auto-save after modification
        self.save_to_file()
    }

    pub fn delete_todo(&mut self, id: usize) -> Result<(), String> {
        if id >= self.todos.len() {
            return Err(format!("Todo with id {id} not found"));
        }
        self.todos.remove(id);

        // Auto-save after modification
        self.save_to_file()
    }

    pub fn get_todo(&self, id: usize) -> Option<&Todo> {
        self.todos.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_manager() -> TodoManager {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join(".tt.json");

        TodoManager {
            todos: Vec::new(),
            file_path,
        }
    }

    #[test]
    fn test_new_todo_manager() {
        let manager = create_test_manager();
        assert_eq!(manager.list_todos().len(), 0);
        assert!(manager.list_todos().is_empty());
    }

    #[test]
    fn test_add_todo() {
        let mut manager = create_test_manager();
        let todo = manager.add_todo("Test todo".to_string(), 1).unwrap();
        assert_eq!(todo.title, "Test todo");
        assert_eq!(todo.completed, false);
        assert_eq!(manager.list_todos().len(), 1);
    }

    #[test]
    fn test_list_todos() {
        let mut manager = create_test_manager();
        manager.add_todo("Todo 1".to_string(), 1).unwrap();
        manager.add_todo("Todo 2".to_string(), 1).unwrap();
        let todos = manager.list_todos();
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].title, "Todo 1");
        assert_eq!(todos[1].title, "Todo 2");
    }

    #[test]
    fn test_mark_completed() {
        let mut manager = create_test_manager();
        manager.add_todo("Test todo".to_string(), 1).unwrap();

        // Mark as completed
        assert!(manager.mark_completed(0).is_ok());
        assert!(manager.get_todo(0).unwrap().completed);

        // Try to mark non-existent todo
        assert!(manager.mark_completed(1).is_err());
    }

    #[test]
    fn test_mark_incomplete() {
        let mut manager = create_test_manager();
        manager.add_todo("Test todo".to_string(), 1).unwrap();

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
        let mut manager = create_test_manager();
        manager.add_todo("Test todo".to_string(), 1).unwrap();

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
        let mut manager = create_test_manager();
        manager.add_todo("Todo 1".to_string(), 1).unwrap();
        manager.add_todo("Todo 2".to_string(), 1).unwrap();
        assert_eq!(manager.list_todos().len(), 2);
        // Delete first todo
        assert!(manager.delete_todo(0).is_ok());
        assert_eq!(manager.list_todos().len(), 1);
        assert_eq!(manager.get_todo(0).unwrap().title, "Todo 2");
        // Try to delete non-existent todo
        assert!(manager.delete_todo(1).is_err());
    }

    #[test]
    fn test_get_todo() {
        let mut manager = create_test_manager();
        manager.add_todo("Test todo".to_string(), 1).unwrap();

        // Get existing todo
        let todo = manager.get_todo(0);
        assert!(todo.is_some());
        assert_eq!(todo.unwrap().title, "Test todo");

        // Get non-existent todo
        let todo = manager.get_todo(1);
        assert!(todo.is_none());
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join(".tt.json");
        // Create manager and add todos
        let mut manager = TodoManager {
            todos: Vec::new(),
            file_path: file_path.clone(),
        };
        manager.add_todo("Test todo 1".to_string(), 1).unwrap();
        manager.add_todo("Test todo 2".to_string(), 1).unwrap();
        manager.mark_completed(0).unwrap();
        // Verify file was created
        assert!(file_path.exists());
        // Create new manager and load from file
        let mut new_manager = TodoManager {
            todos: Vec::new(),
            file_path,
        };
        new_manager.load_from_file().unwrap();
        // Verify todos were loaded correctly
        assert_eq!(new_manager.list_todos().len(), 2);
        assert_eq!(new_manager.get_todo(0).unwrap().title, "Test todo 1");
        assert_eq!(new_manager.get_todo(0).unwrap().completed, true);
        assert_eq!(new_manager.get_todo(1).unwrap().title, "Test todo 2");
        assert_eq!(new_manager.get_todo(1).unwrap().completed, false);
    }
}
