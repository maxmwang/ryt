use cursive::views::LinearLayout;

use crate::app::components;

pub fn home_view() -> LinearLayout {
    let layout = LinearLayout::vertical()
        .child(components::title::title_view())
        .child(components::input::input_view());

    layout
}
