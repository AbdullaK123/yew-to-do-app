use serde::{Deserialize, Serialize};
use web_sys::{InputEvent, KeyboardEvent};
use yew::prelude::*;
use yew::{html, Component, Context, Html};
use crate::ToDosMsg;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
}

impl Component for ToDosModel {
    type Message = ToDosMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::new()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ToDosMsg::UpdateToDo( text) => self.update_new_todo_text(text),
            ToDosMsg::AddToDo(text) => self.add_todo(text),
            ToDosMsg::ToggleToDo(id) => self.toggle_todo(id),
            ToDosMsg::RemoveToDo(id) => self.remove_todo(id),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let link = ctx.link();

        html! {
             <div>
                <h1>{"Todo App"}</h1>
                <p>{format!("You have {} todos", self.todos.len())}</p>
                <div>
                    <input
                        type="text"
                        placeholder="What needs to be done?"
                        value={self.new_todo_text.clone()}
                        oninput={link.callback(|e: InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            ToDosMsg::UpdateToDo(input.value())
                        })}
                        onkeypress={
                            link.callback({
                                let text = self.new_todo_text.clone();
                                move |e: KeyboardEvent| {
                                    if e.key() == "Enter" {
                                        e.prevent_default();
                                        ToDosMsg::AddToDo(text.clone())
                                    } else {
                                        // Just update with current value (no-op)
                                        ToDosMsg::UpdateToDo(text.clone())
                                    }
                                }
                            })
                       }
                    />
                    <button
                        onclick={link.callback({
                            let text = self.new_todo_text.clone();
                            move |_| ToDosMsg::AddToDo(text.clone())
                        })}
                    >
                        {"Add"}
                    </button>
                    <div>
                        {for self.todos.iter().map(|todo| {
                            let todo_id = todo.id; // Capture the ID as an owned value
                            html! {
                                <li key={todo.id}>
                                    <span style={if todo.completed { "text-decoration: line-through;" } else { "" }}>
                                        {todo.text.clone()}
                                    </span>
                                    <span>{format!("(ID: {}, Completed {})", todo.id, if todo.completed {"Yes"} else {"No"})}</span>
                                    <label>
                                        <input
                                            type="checkbox"
                                            checked={todo.completed}
                                            onclick={link.callback(move |_| ToDosMsg::ToggleToDo(todo_id))}
                                        />
                                        { "Completed "}
                                    </label>
                                    <button
                                        onclick={link.callback(move |_| ToDosMsg::RemoveToDo(todo_id))}
                                        style="margin-left: 10px; background-color: #ff4444; color: white; border: none; padding: 2px 8px; cursor: pointer;"
                                    >
                                        {"Delete"}
                                    </button>
                                </li>
                            }
                        })}
                    </div>
                </div>
            </div>
        }
    }
}