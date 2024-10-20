mod components;

use components::profile::Profile;
use leptos::leptos_dom::logging::console_log;
use leptos::web_sys::{console, MouseEvent};
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let increment = move |e: MouseEvent| {
        set_count.update(|prev| *prev += 1);
        console::info_1(&e);
    };
    console_log("hello world");

    view! {
        <div class:wrapper=move || count() % 2 == 1>
            <Profile img_src="https://avatar.iran.liara.run/public/boy?username=Ash" />
            <button on:click=increment>"Click me: " {count}</button>
        </div>
    }
}
