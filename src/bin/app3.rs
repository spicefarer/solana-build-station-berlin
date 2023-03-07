use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
fn main() {
    let style = include_str!("../style.css");
    let config = Config::new()
        .with_window(default_window())
        .with_custom_head(
            format!(
                r#"
        <title>Hello Solana</title>
        <style>{style}</style>
        "#
            )
            .into(),
        );
    dioxus_desktop::launch_cfg(solana_example::app3, config);
}

pub fn default_window() -> WindowBuilder {
    let builder = WindowBuilder::new();
    let s = LogicalSize::new(800., 600.);

    use dioxus_desktop::tao::menu::{MenuBar as Menu, MenuItem};
    let mut menu_bar_menu = Menu::new();
    let mut first_menu = Menu::new();
    first_menu.add_native_item(MenuItem::Copy);
    first_menu.add_native_item(MenuItem::Paste);
    first_menu.add_native_item(MenuItem::SelectAll);
    first_menu.add_native_item(MenuItem::CloseWindow);
    first_menu.add_native_item(MenuItem::Hide);
    first_menu.add_native_item(MenuItem::Quit);
    menu_bar_menu.add_submenu("Solana", true, first_menu);

    let builder = builder
        .with_title("Hello Solana")
        .with_menu(menu_bar_menu)
        .with_theme(Some(dioxus_desktop::tao::window::Theme::Dark))
        .with_inner_size(s);

    #[cfg(target_os = "macos")]
    {
        use dioxus_desktop::tao::platform::macos::WindowBuilderExtMacOS;
        builder.with_title_hidden(true)
    }
    #[cfg(not(target_os = "macos"))]
    builder
}
