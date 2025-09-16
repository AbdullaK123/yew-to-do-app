#[cfg(test)]
mod tests {
    use yew_to_do_app::{ToDosModel, ToDosMsg};
    use super::*;

    #[test]
    fn test_add_todo() {
        let mut model = ToDosModel::new();

        // Test adding a todo
        model.handle_message(ToDosMsg::AddToDo("Buy groceries".to_string()));

        assert_eq!(model.todos.len(), 1);
        assert_eq!(model.todos[0].text, "Buy groceries");
        assert_eq!(model.todos[0].completed, false);
        assert_eq!(model.todos[0].id, 0);
        assert_eq!(model.next_id, 1);
        assert_eq!(model.new_todo_text, ""); // Should be cleared
    }

    #[test]
    fn test_toggle_todo() {
        let mut model = ToDosModel::new();

        // Add a todo first
        model.handle_message(ToDosMsg::AddToDo("Test todo".to_string()));

        // Toggle it
        model.handle_message(ToDosMsg::ToggleToDo(0));

        assert_eq!(model.todos[0].completed, true);

        // Toggle it back
        model.handle_message(ToDosMsg::ToggleToDo(0));

        assert_eq!(model.todos[0].completed, false);
    }

    #[test]
    fn test_remove_todo() {
        let mut model = ToDosModel::new();

        // Add two todos
        model.handle_message(ToDosMsg::AddToDo("First todo".to_string()));
        model.handle_message(ToDosMsg::AddToDo("Second todo".to_string()));

        assert_eq!(model.todos.len(), 2);

        // Remove the first one
        model.handle_message(ToDosMsg::RemoveToDo(0));

        assert_eq!(model.todos.len(), 1);
        assert_eq!(model.todos[0].text, "Second todo");
    }
}