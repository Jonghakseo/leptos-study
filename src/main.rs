use leptos::*;
use my_leptos::App;

fn main() {
  console_error_panic_hook::set_once();
  mount_to_body(|| view! { <App /> });
}
