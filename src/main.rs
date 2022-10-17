use gloo_net::http::Request;
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

mod filter;
mod footer;
mod info;
mod new_todo;
mod route;
mod todo_list;

use crate::filter::*;
use crate::footer::*;
use crate::info::Info;
use crate::new_todo::*;
use crate::todo_list::*;
use route::AppRoute;

#[derive(PartialEq, Properties)]
pub struct AppProps {
    pub filter: Filter,
}

#[function_component]
fn App(props: &AppProps) -> Html {
    let todos = use_state(Vec::new);
    {
        let todos = todos.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_todos = Request::get("./mock-data/items.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    // log::info!("Fetched data {:#?}", &fetched_todos);
                    todos.set(fetched_todos);
                });
                || ()
            },
            (),
        )
    }

    let new_todo_text = use_state(|| AttrValue::from(""));
    let on_new_todo_text_change = {
        let todos = todos.clone();
        let new_todo_text = new_todo_text.clone();
        move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = input {
                let input_value = input.value();
                log::info!("input value = {}", input_value);
                // new_todo_text.set(input.value());
                new_todo_text.set("".into());
                let mut new_todos = vec![Todo {
                    id: Uuid::new_v4().to_string(),
                    text: input_value,
                    completed: false,
                }];
                new_todos.extend(todos.iter().map(|item: &Todo| item.to_owned()));
                todos.set(new_todos);
            }
        }
    };

    let on_completed_changed = {
        let todos = todos.clone();
        move |id: String| {
            todos.set(
                todos
                    .iter()
                    .map(|todo: &Todo| {
                        if id == todo.id {
                            Todo {
                                completed: !todo.completed,
                                ..todo.clone()
                            }
                        } else {
                            todo.to_owned()
                        }
                    })
                    .collect(),
            )
        }
    };

    let on_clear_completed = {
        let todos = todos.clone();
        move |_| {
            let new_todos = todos
                .iter()
                .map(|t| t.to_owned())
                .filter(|t| !t.completed)
                .collect();

            log::info!("Fetched data {:#?}", &new_todos);
            todos.set(new_todos)
        }
    };

    let toggle_all = use_state(|| false);
    {
        let toggle_all = toggle_all.clone();
        use_effect_with_deps(
            move |todos| {
                let count = todos.iter().filter(|t| !t.completed).count();
                log::info!("count === {}", count);
                toggle_all.set(count == 0);
            },
            todos.clone(),
        );
    }

    let on_toggle_all_change = {
        let toggle_all = toggle_all.clone();
        let todos = todos.clone();
        move |_| {
            log::info!(
                "toggle_all = {}, !*toggle_all={}",
                *toggle_all,
                !*toggle_all
            );
            let new_toggle_all = !*toggle_all;
            toggle_all.set(new_toggle_all);
            todos.set(
                todos
                    .iter()
                    .map(|item: &Todo| Todo {
                        completed: new_toggle_all,
                        ..item.to_owned()
                    })
                    .collect(),
            )
        }
    };

    let on_destroy = {
        let todos = todos.clone();
        move |id| {
            todos.set(
                todos
                    .iter()
                    .filter(|t| t.id != id)
                    .map(|t| t.to_owned())
                    .collect::<Vec<Todo>>(),
            )
        }
    };

    let on_todo_text_change = {
        let todos = todos.clone();
        move |todo: Todo| {
            todos.set(
                todos
                    .iter()
                    .map(|t: &Todo| {
                        if t.id == todo.id {
                            todo.clone()
                        } else {
                            t.to_owned()
                        }
                    })
                    .collect(),
            )
        }
    };

    let toggle_all = *toggle_all;
    let todos = &*todos;
    let new_todo_text = &*new_todo_text;

    let new_todo_input_ref = NodeRef::default();
    {
        let new_todo_input_ref = new_todo_input_ref.clone();
        use_effect_with_deps(
            move |_| {
                if let Some(e) = new_todo_input_ref.cast::<HtmlInputElement>() {
                    let r = e.focus();
                    log::info!("focus result = {:?}", r);
                }
                || ()
            },
            (),
        );
    }
    let filtered_todos: Vec<Todo> = {
        match props.filter {
            Filter::All => todos.iter().map(|t| t.to_owned()).collect(),
            Filter::Active => todos
                .iter()
                .filter(|t| !t.completed)
                .map(|t| t.to_owned())
                .collect(),
            Filter::Completed => todos
                .iter()
                .filter(|t| t.completed)
                .map(|t| t.to_owned())
                .collect(),
        }
    };

    html! {
        <>
            <div class="todoapp">
                <header class="header">
                    <h1>{"todos"}</h1>
                    <NewTodo
                        inner_ref={new_todo_input_ref}
                        value={new_todo_text}
                        on_change={on_new_todo_text_change} />
                </header>
                <section class="main">
                    <input
                        id="toggle-all"
                        type="checkbox"
                        class="toggle-all"
                        onchange={on_toggle_all_change}
                        checked={toggle_all}
                    />
                    <label for="toggle-all"></label>
                    <TodoList
                        todos={filtered_todos}
                        on_completed_changed={on_completed_changed}
                        {on_destroy}
                        {on_todo_text_change}
                    />
                </section>
                <Footer
                    filter={props.filter.clone()}
                    active_todo_count={todos.iter().filter(|it| !it.completed).count()}
                    completed_todo_count={todos.iter().filter(|it| it.completed).count()}
                    on_clear_completed={on_clear_completed}
                />
            </div>
            <Info />
        </>
    }
}

fn switch_filter(route: AppRoute) -> Html {
    let filter = match route {
        AppRoute::Home | AppRoute::All => Filter::All,
        AppRoute::Active => Filter::Active,
        AppRoute::Completed => Filter::Completed,
    };

    html! {
        <App filter={filter} />
    }
}

#[function_component]
fn AppWithRouter() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={switch_filter} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Hello Yew!");
    yew::Renderer::<AppWithRouter>::new().render();
}
