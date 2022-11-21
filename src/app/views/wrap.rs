use cursive::view::IntoBoxedView;
use cursive::views::{Dialog, LinearLayout};

use crate::app::components::input::input_component;

pub fn wrap_view<V: IntoBoxedView + 'static>(view: V, title: &str) -> Dialog {
    let view = Dialog::around(
        LinearLayout::vertical()
            .child(view)
            .child(input_component()),
    )
    .padding_lrtb(2, 2, 1, 1);

    if !title.is_empty() {
        view.title(format!("ryt - {title}"))
    } else {
        view
    }
}
