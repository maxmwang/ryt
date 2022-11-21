use cursive::views::Dialog;

use crate::app::{api::MyVideo, components::list::list_component, views::wrap::wrap_view};

const TITLE: &str = "Results";

pub fn results_view(lst: Vec<MyVideo>) -> Dialog {
    wrap_view(list_component(lst), TITLE)
}
