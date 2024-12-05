use std::time::Duration;

use druid::{widget::Label, AppLauncher, Env, Point, Screen, Widget, WidgetExt, WindowDesc};

pub fn show(s: &str) {
    let screen = Screen::get_monitors()[0].virtual_rect();
    let x = screen.x0 + (screen.x1 - screen.x0) / 2.0;
    let y = screen.y0 + (screen.y1 - screen.y0) / 2.0;

    let main_window = WindowDesc::new(ui_builder(s))
        .window_size((600.0, 400.0))
        .title("Smart Road")
        .set_position(Point::new(x - 300.0, y - 200.0));

    AppLauncher::with_window(main_window)
        .delegate(MyAppDelegate)
        .launch(())
        .expect("Launch failed");
}

struct MyAppDelegate;

impl druid::AppDelegate<()> for MyAppDelegate {
    fn event(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        _window_id: druid::WindowId,
        event: druid::Event,
        _data: &mut (),
        _env: &Env,
    ) -> Option<druid::Event> {
        match event {
            druid::Event::WindowCloseRequested => {
                ctx.submit_command(druid::commands::CLOSE_WINDOW);
                std::thread::sleep(Duration::from_millis(10));
                std::process::exit(0);
            }
            druid::Event::KeyDown(ref key_event) => {
                if key_event.key == druid::keyboard_types::Key::Escape {
                    ctx.submit_command(druid::commands::CLOSE_WINDOW);
                    std::thread::sleep(Duration::from_millis(10));
                    std::process::exit(0);
                }
            }
            _ => {}
        }
        Some(event)
    }
}

fn ui_builder(s: &str) -> impl Widget<()> {
    Label::new(s).center()
}
