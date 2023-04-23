use druid::{AppLauncher, Screen, WindowDesc};
use mouse_position::mouse_position::Mouse;
use num;

mod data;
mod pulsewrapper;
mod theme;
mod ui;

use data::AppState;

pub const MAIN_WINDOW_WIDTH: f64 = 300.0;
pub const MAIN_WINDOW_HEIGHT: f64 = 400.0;

fn main() {
    let mut state = AppState::new();

    let main_window = WindowDesc::new(ui::build_ui())
        .window_size((MAIN_WINDOW_WIDTH, MAIN_WINDOW_HEIGHT))
        .resizable(false)
        .set_position(get_position())
        .title("My first Druid App");

    AppLauncher::with_window(main_window)
        .configure_env(theme::setup)
        .launch(state)
        .expect("Failed to launch application");
}

pub fn get_position() -> (f64, f64) {
    let screen_size = Screen::get_display_rect();
    let middle_x = (screen_size.x0 + screen_size.x1) / 2.0;
    let middle_y = (screen_size.y0 + screen_size.y1) / 2.0;
    match Mouse::get_mouse_position() {
        Mouse::Position { x, y } => { println!("{}, {}", x, y); },
        Mouse::Error => (),
    };
    match Mouse::get_mouse_position() {
        Mouse::Position { x, y } => {(
            num::clamp((x as f64) - MAIN_WINDOW_WIDTH / 2.0, screen_size.x0 + 5.0, screen_size.x1 - MAIN_WINDOW_WIDTH - 5.0),
            if (y as f64) < middle_y {
                (y as f64) + 5.0
            } else {
                (y as f64) - MAIN_WINDOW_HEIGHT - 35.0
            }
        )},
        Mouse::Error => {(
            middle_x - (MAIN_WINDOW_WIDTH / 2.0),
            middle_y - (MAIN_WINDOW_HEIGHT / 2.0)
        )},
    }
}
