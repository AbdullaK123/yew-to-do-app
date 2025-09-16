use web_sys::{InputEvent, KeyboardEvent};
use yew::{html, Html, TargetCast};
use yew::{function_component, Callback, Properties};
use crate::ToDo;

// Props for our function components
#[derive(Properties, PartialEq)]
pub struct TodoHeaderProps {
    pub total_count: usize,
}

#[derive(Properties, PartialEq)]
pub struct AddTodoFormProps {
    pub value: String,
    pub onchange: Callback<String>,
    pub onsubmit: Callback<String>,
}

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
    pub todo: ToDo,
    pub ontoggle: Callback<usize>,
    pub ondelete: Callback<usize>,
}

#[derive(Properties, PartialEq)]
pub struct TodoListProps {
    pub todos: Vec<ToDo>,
    pub ontoggle: Callback<usize>,
    pub ondelete: Callback<usize>,
}

// Function components
#[function_component]
pub fn TodoHeader(props: &TodoHeaderProps) -> Html {
    html! {
        <div class="text-center py-8">
            <h1 class="text-4xl font-bold text-gray-800 mb-2">{"Todo App"}</h1>
            <p class="text-lg text-gray-600">{format!("You have {} todos", props.total_count)}</p>
        </div>
    }
}

#[function_component]
pub fn AddTodoForm(props: &AddTodoFormProps) -> Html {
    html! {
        <div class="flex gap-2 mb-6">
            <input
                type="text"
                placeholder="What needs to be done?"
                value={props.value.clone()}
                class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                oninput={props.onchange.reform(|e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    input.value()
                })}
                onkeypress={{
                    let onsubmit = props.onsubmit.clone();
                    let value = props.value.clone();
                    Callback::from(move |e: KeyboardEvent| {
                        if e.key() == "Enter" {
                            e.prevent_default();
                            onsubmit.emit(value.clone());
                        }
                    })
                }}
            />
            <button
                class="px-6 py-2 bg-blue-500 hover:bg-blue-600 text-white font-semibold rounded-lg transition-colors duration-200"
                onclick={{
                    let onsubmit = props.onsubmit.clone();
                    let value = props.value.clone();
                    Callback::from(move |_| onsubmit.emit(value.clone()))
                }}
            >
                {"Add"}
            </button>
        </div>
    }
}

#[function_component]
pub fn TodoItem(props: &TodoItemProps) -> Html {
    let todo = &props.todo;

    let text_class = if todo.completed {
        "flex-1 text-gray-500 line-through"
    } else {
        "flex-1 text-gray-800"
    };

    html! {
        <li class="flex items-center gap-3 p-4 bg-white rounded-lg shadow-sm border border-gray-200 mb-2">
            <label class="flex items-center cursor-pointer">
                <input
                    type="checkbox"
                    checked={todo.completed}
                    class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500"
                    onclick={{
                        let ontoggle = props.ontoggle.clone();
                        let id = todo.id;
                        Callback::from(move |_| ontoggle.emit(id))
                    }}
                />
                <span class="ml-2 text-sm text-gray-600">{"Completed"}</span>
            </label>

            <span class={text_class}>
                {todo.text.clone()}
            </span>

            <span class="text-xs text-gray-400 bg-gray-100 px-2 py-1 rounded">
                {format!("ID: {}", todo.id)}
            </span>

            <button
                class="px-3 py-1 bg-red-500 hover:bg-red-600 text-white text-sm font-medium rounded transition-colors duration-200"
                onclick={{
                    let ondelete = props.ondelete.clone();
                    let id = todo.id;
                    Callback::from(move |_| ondelete.emit(id))
                }}
            >
                {"Delete"}
            </button>
        </li>
    }
}

#[function_component]
pub fn TodoList(props: &TodoListProps) -> Html {
    html! {
        <div class="space-y-2">
            if props.todos.is_empty() {
                <div class="text-center py-8 text-gray-500">
                    <p class="text-lg">{"No todos yet"}</p>
                    <p class="text-sm">{"Add one above to get started!"}</p>
                </div>
            } else {
                <ul class="space-y-2">
                    { for props.todos.iter().map(|todo| html! {
                        <TodoItem
                            todo={todo.clone()}
                            ontoggle={props.ontoggle.clone()}
                            ondelete={props.ondelete.clone()}
                        />
                    })}
                </ul>
            }
        </div>
    }
}