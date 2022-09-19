use yew::prelude::*;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct NewTodoProps {
    pub value: AttrValue,
    pub on_change: Callback<Event>,
    pub inner_ref: NodeRef,
}

#[function_component(NewTodo)]
pub fn new_todo(
    NewTodoProps {
        value,
        on_change,
        inner_ref,
    }: &NewTodoProps,
) -> Html {
    let value = value.clone();
    // log::info!("new_todo value = {}", &value);
    html! {
        <input
            autofocus={true}
            class="new-todo"
            placeholder="What needs to be done?"
            value={value}
            onchange={on_change}
            ref={inner_ref.clone()}
        />
    }
}
