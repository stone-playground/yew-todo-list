use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct TodoListProps {
    pub todos: Vec<Todo>,
    pub on_completed_changed: Callback<String>,
    pub on_destroy: Callback<String>,
    pub on_todo_text_change: Callback<Todo>,
}

#[function_component(TodoList)]
pub fn todo_list(props: &TodoListProps) -> Html {
    let TodoListProps {
        todos,
        on_completed_changed,
        on_destroy,
        on_todo_text_change,
    } = props;

    let editing_todo = use_state::<Option<Todo>, _>(|| None);

    // log::info!("todos = {:#?}", todos);
    let todo_list_html: Vec<Html> = todos
        .iter()
        .map(|todo| {
            let todo = todo.clone();

            let on_checkbox_selected = {
                let on_completed_changed = on_completed_changed.clone();
                let todo = todo.clone();
                move |_| on_completed_changed.emit(todo.id.clone())
            };

            let on_destroy_click = {
                let on_destroy = on_destroy.clone();
                let todo = todo.clone();
                move |_| on_destroy.emit(todo.id.clone())
            };

            let on_editing_changed = {
                let editing_todo = editing_todo.clone();
                let todo = todo.clone();
                move |_| editing_todo.set(Some(todo.clone()))
            };

            let on_editor_changed = {
                let on_todo_text_change = on_todo_text_change.clone();
                let todo = todo.clone();
                let editing_todo = editing_todo.clone();

                move |e: Event| {
                    let target = e.target_dyn_into::<HtmlInputElement>();
                    if let Some(t) = target {
                        editing_todo.set(None);
                        on_todo_text_change.emit(Todo {
                            text: t.value(),
                            ..todo.clone()
                        })
                    }
                }
            };

            let is_editing = if let Some(selected) = &*editing_todo {
                selected.id == todo.id
            } else {
                false
            };

            html! {
                <li class={classes!(
                    todo.completed.then_some("completed"),
                    is_editing.then_some("editing")
                )}>
                    <div class="view">
                        <input
                            type="checkbox"
                            class="toggle"
                            onchange={on_checkbox_selected}
                            checked={todo.completed}
                        />
                        <label ondblclick={on_editing_changed}>{&todo.text}</label>
                        <button class="destroy" onclick={on_destroy_click}></button>
                    </div>
                    <input
                        class="edit"
                        value={todo.text}
                        onchange={on_editor_changed}
                    />
                </li>
            }
        })
        .collect();

    html! {
        <ul class="todo-list">
            { todo_list_html }
        </ul>
    }
}
