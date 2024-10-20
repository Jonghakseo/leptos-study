use crate::components::counter::Counter;
use crate::components::profile::Profile;
use crate::components::ui::layout::Flex;

use crate::v;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Flex vertical=true width=v!("110") height=v!(100) gap=v!(8)>
            <h1>"Home"</h1>
            <Profile img_src="https://avatar.iran.liara.run/public/boy?username=Ash" />
            <Counter />
        </Flex>
    }
}
