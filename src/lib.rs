use leptos::leptos_dom::logging::console_log;
use leptos::web_sys::MouseEvent;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let increment = move |e: MouseEvent| {
        set_count(count() + 1);
        web_sys::console::log_1(&e);
    };
    console_log("hello world");

    view! {
        <div>
            <button on:click=increment>"Click me: " {count}</button>
        </div>
    }
}
