use lib::{common::Seedable, forces::field::*, forces::map::FlowField};
use nannou::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const MARGIN: f32 = 0.0;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Model {
    seed: u32,
    width: f32,
    height: f32,
    flowfield: FlowField,
}

impl Seedable for Model {
    fn seed(&self) -> u32 {
        self.seed
    }

    fn set_seed(&mut self, seed: u32) -> &mut Self {
        self.seed = seed;
        self.flowfield.set_seed(seed);
        self
    }
}

impl Model {
    fn new(app: &App, seed: u32) -> Self {
        let (width, height) = get_window_size(app);
        let mut flowfield = FlowField::new(width, height, 20.0, 1.0);
        flowfield.set_seed(seed);

        Model {
            seed,
            width,
            height,
            flowfield,
        }
    }

    fn reset_seed(&mut self) -> &mut Self {
        self.set_seed(random_range(0, 1000000000))
    }

    fn reset(&mut self, w: f32, h: f32) -> &mut Self {
        self.width = w;
        self.height = h;
        self.flowfield = self.flowfield.reset(w, h);
        self.set_field();

        self
    }

    fn set_field(&mut self) -> &mut Self {
        self.flowfield.zero();
        let pt1 = self.index_at(0.7, 0.7);
        let pt2 = self.index_at(0.3, 0.3);

        self.flowfield
            .merge(&AttractorField::new(-1.0, pt1[0], pt1[1]))
            .merge(&AttractorField::new(-1.0, pt2[0], pt2[1]))
            .merge(&PerlinField::new(0.5, 0.015, self.seed));
        self
    }

    fn index_at(&self, xs: f32, ys: f32) -> [usize; 2] {
        self.flowfield
            .index_at([xs * self.width as f32, ys * self.height as f32])
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .resized(window_resized)
        .build()
        .unwrap();

    let seed = random_range(0, 1000000000);
    let mut model = Model::new(app, seed);
    model.set_field();
    model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Update the flowfield here
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let gdraw = draw
        .scale_y(-1.0)
        .x_y(model.width / -2.0 + 0.5, model.height / -2.0 + 0.5);

    draw.background().color(STEELBLUE);
    model.flowfield.display(&gdraw, false, None);
    gdraw.ellipse().x_y(0.0, 0.0).color(RED).w_h(10.0, 10.0);
    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    let (w, h) = get_window_size(app);

    match key {
        Key::Space => {
            model.reset_seed().reset(w, h);
        }
        _ => (),
    }
}

fn window_resized(_app: &App, model: &mut Model, dim: Vec2) {
    let (w, h) = (dim.x - 2.0 * MARGIN, dim.y - 2.0 * MARGIN);
    model.reset(w, h);
}

fn get_window_size(app: &App) -> (f32, f32) {
    let rect = app.window_rect().pad(MARGIN);
    return (rect.w(), rect.h());
}
