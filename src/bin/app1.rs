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
    dioxus_desktop::launch_cfg(solana_example::app1, config);
}

pub fn default_window() -> WindowBuilder {
    let builder = WindowBuilder::new();
    let s = LogicalSize::new(800., 600.);
    builder
        .with_title("Hello Solana")
        .with_theme(Some(dioxus_desktop::tao::window::Theme::Dark))
        .with_inner_size(s)
}
