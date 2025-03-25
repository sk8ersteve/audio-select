use druid::widget::Controller;
use druid::{AppLauncher, Screen, WindowDesc};
use druid::{Env, Event, EventCtx, InternalEvent, Widget, WidgetExt};
use mouse_position::mouse_position::Mouse;

mod data;
mod pulsewrapper;
mod theme;
mod ui;

use data::AppState;

pub const MAIN_WINDOW_WIDTH: f64 = 300.0;
pub const MAIN_WINDOW_HEIGHT: f64 = 400.0;

fn main() {
    let state = AppState::new();

    let main_window = WindowDesc::new(ui::build_ui().controller(WindowController))
        .window_size((MAIN_WINDOW_WIDTH, MAIN_WINDOW_HEIGHT))
        .resizable(false)
        .set_position(get_position())
        .title("Audio Select");

    AppLauncher::with_window(main_window)
        .configure_env(theme::setup)
        .launch(state)
        .expect("Failed to launch application");
}

pub fn get_position() -> (f64, f64) {
    let druid::Rect { x0, x1, y0, y1 } = Screen::get_display_rect();
    let middle_x = (x0 + x1) / 2.0;
    let middle_y = (y0 + y1) / 2.0;
    // match Mouse::get_mouse_position() {
    //     Mouse::Position { x, y } => { println!("{}, {}", x, y); },
    //     Mouse::Error => (),
    // };
    match Mouse::get_mouse_position() {
        Mouse::Position { x, y } => (
            num::clamp(
                (x as f64) - MAIN_WINDOW_WIDTH / 2.0,
                x0 + 5.0,
                x1 - MAIN_WINDOW_WIDTH - 5.0,
            ),
            if (y as f64) < middle_y {
                (y as f64) + 10.0
            } else {
                (y as f64) - MAIN_WINDOW_HEIGHT - 45.0
            },
        ),
        Mouse::Error => (
            middle_x - (MAIN_WINDOW_WIDTH / 2.0),
            middle_y - (MAIN_WINDOW_HEIGHT / 2.0),
        ),
    }
}

struct WindowController;

impl<W: Widget<AppState>> Controller<AppState, W> for WindowController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        if let Event::Internal(InternalEvent::MouseLeave) = event {
            if data.close_on_leave {
                ctx.window().close();
            }
        }
        child.event(ctx, event, data, env)
    }
}
