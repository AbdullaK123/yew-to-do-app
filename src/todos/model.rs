use serde::{Deserialize, Serialize};
use web_sys::{InputEvent, KeyboardEvent};
use yew::prelude::*;
use yew::{html, Component, Context, Html};
use crate::todos::components::{AddTodoForm, TodoHeader, TodoList};
use crate::ToDosMsg;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(PartialEq)]
pub struct ToDo{
    pub id: usize,
    pub text: String,
    pub completed: bool,
}

pub struct ToDosModel {
    pub todos: Vec<ToDo>,
    pub new_todo_text: String,
    pub next_id: usize
}

impl ToDosModel {
    pub fn new() -> Self {
        Self {
            todos: vec![],
            new_todo_text: String::new(),
            next_id: 0
        }
    }

    pub fn update_new_todo_text(&mut self, new_todo_text: String) -> bool {
        self.new_todo_text = new_todo_text;
        true
    }

    pub fn add_todo(&mut self, text: String) -> bool {
        if !text.trim().is_empty() {
            self.todos.push(ToDo {
                id: self.next_id,
                text: text.trim().to_string(),
                completed: false,
            });
            self.next_id += 1;
            self.new_todo_text.clear();
        }
        true
    }

    pub fn remove_todo(&mut self, id: usize) -> bool {
        self.todos.retain(|todo| todo.id != id);
        true
    }

    pub fn toggle_todo(&mut self, id: usize) -> bool {
        if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id == id ) {
            todo.completed = !todo.completed;
        }
        true
    }

    pub fn handle_message(&mut self, msg: ToDosMsg) -> bool {
        match msg {
            ToDosMsg::UpdateToDo(text) => self.update_new_todo_text(text),
            ToDosMsg::AddToDo(text) => self.add_todo(text),
            ToDosMsg::ToggleToDo(id) => self.toggle_todo(id),
            ToDosMsg::RemoveToDo(id) => self.remove_todo(id),
        }
    }
}

impl Component for ToDosModel {
    type Message = ToDosMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::new()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.handle_message(msg)
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

            html! {
            <div class="flex flex-col container mx-auto px-4">
                <TodoHeader total_count={self.todos.len()} />

                <AddTodoForm
                    value={self.new_todo_text.clone()}
                    onchange={link.callback(|text| ToDosMsg::UpdateToDo(text))}
                    onsubmit={link.callback(|text| ToDosMsg::AddToDo(text))}
                />

                <TodoList
                    todos={self.todos.clone()}
                    ontoggle={link.callback(|id| ToDosMsg::ToggleToDo(id))}
                    ondelete={link.callback(|id| ToDosMsg::RemoveToDo(id))}
                />
            </div>
        }
    }
}