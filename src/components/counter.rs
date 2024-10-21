use crate::components::ui::layout::Flex;
use leptos::web_sys::{console, MouseEvent};
use leptos::*;

#[component]
pub fn Counter() -> impl IntoView {
  let (count, set_count) = create_signal(0);

  let increment = move |e: MouseEvent| {
    set_count.update(|prev| *prev += 1);
    console::log_1(&e);
  };

  view! {
    <Flex vertical=true>
      <h1>"Counter"</h1>
      <h2>{count}</h2>
      <button on:click=increment>"Increment"</button>
      <button on:click=move |_| set_count(count.get() - 1)>"Decrement"</button>
    </Flex>
  }
}
