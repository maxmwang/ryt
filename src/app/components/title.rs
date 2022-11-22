use cursive::{
    view::Resizable,
    views::{ResizedView, TextView},
};

pub fn title_component() -> ResizedView<TextView> {
    let title_text = (vec![
        r"         _    _        _        _       ",
        r"        /\ \ /\ \     /\_\     /\ \     ",
        r"       /  \ \\ \ \   / / /     \_\ \    ",
        r"      / /\ \ \\ \ \_/ / /      /\__ \   ",
        r"     / / /\ \_\\ \___/ /      / /_ \ \  ",
        r"    / / /_/ / / \ \ \_/      / / /\ \ \ ",
        r"   / / /__\/ /   \ \ \      / / /  \/_/ ",
        r"  / / /_____/     \ \ \    / / /        ",
        r" / / /\ \ \        \ \ \  / / /         ",
        r"/ / /  \ \ \        \ \_\/_/ /          ",
        r"\/_/    \_\/         \/_/\_\/           ",
    ])
    .join("\n");

    TextView::new(title_text).center().full_screen()
}
