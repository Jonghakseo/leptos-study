use crate::components::ui::layout::Flex;
use crate::v;
use leptos::leptos_dom::logging::console_log;
use leptos::web_sys::{console, MouseEvent};
use leptos::*;

#[component]
pub fn Counter() -> impl IntoView {
  let (count, set_count) = create_signal(0);

  let increment = move |e: MouseEvent| {
    set_count.update(|prev| *prev += 1);
    console_log(&(count.get().to_string()));
    console::log_1(&e);
  };

  console_log(&(count.get().to_string()));

  fn print(_: MouseEvent) {
    console_log("test");
  }

  view! {
    <Flex vertical=true gap=v!(8)>
      <h1>"Counter"</h1>
      <h2>{count}</h2>
      <button on:click=increment>"Increment"</button>
      <button on:click=move |_| set_count(count.get() - 1)>"Decrement"</button>
      <button on:click=print>"test"</button>
    </Flex>
  }
}
