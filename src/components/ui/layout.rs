use crate::v;
use leptos::*;

use std::fmt::Display;

const EMPTY: &str = "NONE";

#[component]
pub fn Flex(
    children: Children,
    /// isVertical (default: false)
    #[prop(default = false)]
    vertical: bool,
    /// width px size or attribute
    #[prop(default = v!(EMPTY))]
    width: Box<dyn Display>,
    /// min width px size or attribute
    #[prop(default = v!(EMPTY))]
    min_width: Box<dyn Display>,
    /// max width px size or attribute
    #[prop(default = v!(EMPTY))]
    max_width: Box<dyn Display>,
    /// height px size or attribute
    #[prop(default = v!(EMPTY))]
    height: Box<dyn Display>,
    /// min height px size or attribute
    #[prop(default = v!(EMPTY))]
    min_height: Box<dyn Display>,
    /// max height px size or attribute
    #[prop(default = v!(EMPTY))]
    max_height: Box<dyn Display>,
    /// padding px size or attribute
    #[prop(default = v!(EMPTY))]
    padding: Box<dyn Display>,
    /// margin px size or attribute
    #[prop(default = v!(EMPTY))]
    margin: Box<dyn Display>,
    /// gap px size or attribute
    #[prop(default = v!(EMPTY))]
    gap: Box<dyn Display>,
) -> impl IntoView {
    let style = {
        let mut style = vec![String::from("display: flex;")];
        if vertical {
            style.push(String::from("flex-direction: column;"));
        }
        if width.to_string() != EMPTY {
            style.push(get_px_or_attribute("width", width));
        }
        if min_width.to_string() != EMPTY {
            style.push(get_px_or_attribute("min-width", min_width));
        }
        if max_width.to_string() != EMPTY {
            style.push(get_px_or_attribute("max-width", max_width));
        }
        if height.to_string() != EMPTY {
            style.push(get_px_or_attribute("height", height));
        }
        if min_height.to_string() != EMPTY {
            style.push(get_px_or_attribute("min-height", min_height));
        }
        if max_height.to_string() != EMPTY {
            style.push(get_px_or_attribute("max-height", max_height));
        }
        if padding.to_string() != EMPTY {
            style.push(get_px_or_attribute("padding", padding));
        }
        if margin.to_string() != EMPTY {
            style.push(get_px_or_attribute("margin", margin));
        }
        if gap.to_string() != EMPTY {
            style.push(get_px_or_attribute("gap", gap));
        }

        style.join(" ")
    };

    view! { <div style=style>{children()}</div> }
}

fn get_px_or_attribute(attribute: &str, value: Box<dyn Display>) -> String {
    if value.to_string().parse::<i32>().is_ok() {
        format!("{}: {}px;", attribute, value)
    } else {
        format!("{}: {};", attribute, value)
    }
}
