#[derive(Debug, Clone)]
pub enum ToDosMsg {
    AddToDo(String),
    ToggleToDo(usize),
    RemoveToDo(usize),
    UpdateToDo(String)
}