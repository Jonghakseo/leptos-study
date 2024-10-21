use leptos::*;

#[component]
pub fn Profile(img_src: &'static str) -> impl IntoView {
  view! {
    <div>
      <img src=img_src height=100 width=100 />
    </div>
  }
}
