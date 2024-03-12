// include the particles module
// which can then be used in the model module
// by using the crate::particles::Particle; syntax
mod model;
mod particles;

use lib::colors::Color;
use model::Model;
use nannou::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

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
        .build()
        .unwrap();

    // these lines should be able to get combined into one line
    // however, it returns &mut Model instead of Model
    // which annoys rust-analyzer
    // for now, I am using this, but will try to use the builder pattern later
    let mut model = Model::new();
    model.reset(app);
    model
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.reset(app);
        }
        Key::Up => {
            model.add_particles(app, 1);
        }
        Key::Down => {
            model.add_particles(app, -1);
        }
        _ => (),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let bg_color: Rgba = Color::new(0.94, 0.05, model.hue, 1.0).into();
    draw.background().color(bg_color);

    // use display method to draw all particles
    model.display(&draw);

    draw.to_frame(app, &frame).unwrap();
}
