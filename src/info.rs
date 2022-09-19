use yew::prelude::*;

#[function_component(Info)]
pub fn info() -> Html {
    html! {
        <footer class="info">
            <p>{ "Double-click to edit a todo" }</p>
            <p>{ "Created by " }
                <a href="http://github.com/SunHuawei/">
                    {"SunHuawei"}
                </a>
            </p>
        </footer>
    }
}
