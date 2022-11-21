use cursive::views::Dialog;

use crate::app::{components::help_list, views::wrap::wrap_view};

const TITLE: &str = "Help";

pub fn help_view() -> Dialog {
    wrap_view(help_list::help_list_component(), TITLE)
}
