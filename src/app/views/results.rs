use cursive::views::{Dialog, LinearLayout};

use crate::app::api::MyVideo;
use crate::app::components;

pub fn results_view(lst: Vec<MyVideo>) -> Dialog {
    let layout = Dialog::around(
        LinearLayout::vertical()
            .child(components::list::list_component(lst))
            .child(components::input::input_component()),
    )
    .title("ryt")
    .padding_lrtb(2, 2, 1, 1);

    layout
}
