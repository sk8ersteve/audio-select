use druid::{AppLauncher, Screen, WindowDesc};
use mouse_position::mouse_position::Mouse;

mod data;
mod pulsewrapper;
mod theme;
mod ui;

use data::AppState;

fn main() {
    let mut state = AppState::new();
    let screen_size = Screen::get_display_rect();
    let position = Mouse::get_mouse_position();

    let main_window = WindowDesc::new(ui::build_ui())
        .window_size((300.0, 400.0))
        .resizable(false)
        .title("My first Druid App");

    let main_window = match position {
        Mouse::Position { x, y } => main_window.set_position((x as f64, y as f64)),
        Mouse::Error => main_window,
    };

    AppLauncher::with_window(main_window)
        .configure_env(theme::setup)
        .launch(state)
        .expect("Failed to launch application");
}
