use leptos::*;

#[component]
pub fn Profile(img_src: &'static str) -> impl IntoView {
    view! {
        <div>
            <h1>"Profile"</h1>
            <img src=img_src height=100 width=100></img>
        </div>
    }
}
