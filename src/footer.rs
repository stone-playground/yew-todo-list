use crate::filter::*;
use crate::route::*;
use strum::IntoEnumIterator;
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct FooterProps {
    pub filter: Filter,
    pub active_todo_count: usize,
    pub completed_todo_count: usize,
    pub on_clear_completed: Callback<MouseEvent>,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let filter_to_route = |f: &Filter| -> AppRoute {
        match f {
            Filter::All => AppRoute::All,
            Filter::Active => AppRoute::Active,
            Filter::Completed => AppRoute::Completed,
        }
    };

    let filters_html = Filter::iter().map(|f| {
        html! {
            <li>
                <AppLink
                    to={filter_to_route(&f)}
                    classes={classes!((f == props.filter).then_some("selected"))}
                >
                    {f}
                </AppLink>
            </li>
        }
    });

    html! {
        <footer class="footer">
            <span class="todo-count">
                {props.active_todo_count}{" items left"}
            </span>
            <ul class="filters">
                { for filters_html  }
            </ul>
            {
                if props.completed_todo_count > 0 {
                    html! {
                        <button class="clear-completed" onclick={props.on_clear_completed.clone()}>
                            { "Clear completed "}
                        </button>
                    }
                } else {
                    html! { }
                }
            }
        </footer>
    }
}
