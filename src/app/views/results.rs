use cursive::views::LinearLayout;

use crate::app::api::MyVideo;
use crate::app::components;

pub fn results_view(lst: Vec<MyVideo>) -> LinearLayout {
    let layout = LinearLayout::vertical()
        .child(components::list::list_component(lst))
        .child(components::input::input_component());

    layout
}
