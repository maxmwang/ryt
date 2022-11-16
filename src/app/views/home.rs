use cursive::views::{Dialog, LinearLayout};

use crate::app::components;

pub fn home_view() -> Dialog {
    let layout = Dialog::around(
        LinearLayout::vertical()
            .child(components::title::title_component())
            .child(components::input::input_component()),
    )
    .padding_lrtb(2, 2, 1, 1);

    layout
}
