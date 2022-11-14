use cursive::Cursive;

pub fn load_config(siv: &mut Cursive) {
    // enable refresh on window resize
    siv.set_autorefresh(true);

    load_theme(siv);
    load_global_listeners(siv);
}

fn load_theme(siv: &mut Cursive) {
    use cursive::theme;
    use cursive::traits::With;

    siv.set_theme(theme::Theme {
        shadow: false,
        borders: theme::BorderStyle::Simple,
        palette: theme::Palette::default().with(|palette| {
            // use cursive::theme::BaseColor::*;
            use cursive::theme::Color::*;
            use cursive::theme::PaletteColor::*;

            // make app background transparent (inherit terminal color)
            palette[Background] = TerminalDefault;
            // make default view background transparent (inherit terminal color)
            palette[View] = TerminalDefault;

            palette[Primary] = TerminalDefault;
            palette[Secondary] = TerminalDefault;

            palette[TitlePrimary] = TerminalDefault;
        }),
    });
}

fn load_global_listeners(siv: &mut Cursive) {
    siv.add_global_callback('q', |s| s.quit());
}
