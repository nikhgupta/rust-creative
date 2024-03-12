use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Dot {
    x: f32,
    y: f32,
    radius: f32,
    color: Rgba,
}

struct Model {
    seed: u64,
    dots: Vec<Dot>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let seed = random_range(0, 1000000);
    let mut dots = Vec::new();
    let mut rng = StdRng::seed_from_u64(seed);

    dots.push(Dot {
        x: 0.0,
        y: 0.0,
        radius: rng.gen_range(40.0..200.0),
        color: Rgba::new(
            rng.gen(),
            rng.gen(),
            rng.gen(),
            0.25 * 0.75 + rng.gen::<f32>(),
        ),
    });
    Model { seed, dots }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.seed = random_range(0, 1000000);
            let mut rng = StdRng::seed_from_u64(model.seed);
            model.dots[0].radius = rng.gen_range(40.0..200.0);
            model.dots[0].color = Rgba::new(
                rng.gen(),
                rng.gen(),
                rng.gen(),
                0.25 + 0.75 * rng.gen::<f32>(),
            );
        }
        _ => (),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let mut rng = StdRng::seed_from_u64(model.seed);

    draw.background().color(SNOW);

    let dot = &model.dots[0];
    draw.ellipse()
        .x_y(
            dot.x + rng.gen_range(-0.4..0.4) * WIDTH as f32 / 2.0,
            dot.y + rng.gen_range(-0.4..0.4) * HEIGHT as f32 / 2.0,
        )
        .radius(dot.radius)
        .color(dot.color);

    draw.to_frame(app, &frame).unwrap();
}
