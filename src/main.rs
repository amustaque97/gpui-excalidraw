use gpui::{
    App, AppContext, Application, Bounds, KeyBinding, Render, Window, WindowBounds, WindowOptions,
    actions, div, px, size,
};

// setup actions for quit
actions!(mushu_draw, [Quit]);

struct DrawMac {}

impl Render for DrawMac {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
    }
}

fn main() {
    let app = Application::new();

    app.run(move |ctx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(800.0)), ctx);
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };
        let window = ctx.open_window(window_options, move |window: &mut Window, app: &mut App| {
            app.new(|_| DrawMac {})
        });

        // if no window is created exit the app
        if window.is_err() {
            std::process::exit(1);
        }
        ctx.activate(true);
        ctx.on_action(|_: &Quit, app| app.quit());
        ctx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        ctx.bind_keys([KeyBinding::new("ctrl-c", Quit, None)]);
    });
}
