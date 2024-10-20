mod components;
mod pages;
use leptos::*;
use pages::home::HomePage;

#[macro_export]
macro_rules! v {
    ($value:expr) => {
        Box::new($value)
    };
}

#[component]
pub fn App() -> impl IntoView {
    view! { <HomePage /> }
}
