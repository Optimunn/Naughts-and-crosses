use piston_window::*;
#[cfg(not(debug_assertions))]
use std::env;
mod game;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const WIDTH_: f64 = 680.0;
const HIGHT_: f64 = 680.0;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Naughts and crosses", [WIDTH_, HIGHT_])
        .exit_on_esc(true)
        .resizable(false)
        .samples(4)
        .graphics_api(OpenGL::V3_2)
        .build()
        .unwrap();

    #[cfg(debug_assertions)]
    let glyphs = window
        .load_font("assets/Arial.ttf").unwrap();

    #[cfg(not(debug_assertions))]
    let exe_path = env::current_exe().unwrap();
    #[cfg(not(debug_assertions))]
    let exe_dir = exe_path.parent().unwrap();
    #[cfg(not(debug_assertions))]
    let font_path = exe_dir.join("../Assets/Arial.ttf");
    #[cfg(not(debug_assertions))]
    let glyphs = window
        .load_font(font_path).unwrap();

    let mut apl = game::App::new(WIDTH_, HIGHT_, glyphs);
    let mut position = [0.0, 0.0];


    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, d| {
            clear(WHITE, g);
            apl.draw(&c, g, d);
        });

        if let Some(pos) = event.mouse_cursor_args()
        {
            position = pos;
        }
        
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            apl.on_click(position);
        }

        event.update(|arg| {
            apl.update(arg.dt, position);
        });
    }
}