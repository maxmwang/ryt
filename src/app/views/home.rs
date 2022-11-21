use cursive::views::Dialog;

use crate::app::{components::title, views::wrap::wrap_view};

pub fn home_view() -> Dialog {
    wrap_view(title::title_component(), "")
}
