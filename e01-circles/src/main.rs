mod model;
mod particles;

use lib::colors::Color;
use model::Model;
use nannou::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const MARGIN: f32 = 40.0;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .resized(window_resized)
        .build()
        .unwrap();

    let w = WIDTH as f32 - 2.0 * MARGIN;
    let h = HEIGHT as f32 - 2.0 * MARGIN;

    let mut model = Model::new();
    model.reset(w, h);
    model
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    let rect = app.window_rect().pad(MARGIN);

    let w = rect.w();
    let h = rect.h();

    match key {
        Key::Space => {
            model.reset(w, h);
        }
        Key::Up => {
            model.num(model.num + 5).generate_particles(w, h);
        }
        Key::Down => {
            if model.num > 1 {
                model.num(model.num - 5).generate_particles(w, h);
            }
        }
        _ => (),
    }
}

fn window_resized(_app: &App, model: &mut Model, dim: Vec2) {
    let (w, h) = (dim.x - 2.0 * MARGIN, dim.y - 2.0 * MARGIN);
    model.reset_num(w, h).generate_particles(w, h);
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let bg_color: Rgba = Color::new(0.98, 0.05, model.hue, 1.0).into();
    draw.background().color(bg_color);

    model.display(&draw);
    draw.to_frame(app, &frame).unwrap();
}
