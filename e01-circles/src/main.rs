use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};

// use the color module from the color.rs file
// in the same directory
mod color;
// re-export the Color struct from the color module
// this way, we can use Color instead of color::Color
use color::Color;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const MARGIN: f32 = 40.0;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

fn rng(seed: u64) -> StdRng {
    StdRng::seed_from_u64(seed)
}

struct Particle {
    x: f32,
    y: f32,
    radius: f32,
    color: Rgba,
}

impl Particle {
    // display method to draw each particle
    fn display(&self, draw: &nannou::Draw) {
        draw.ellipse()
            .x_y(self.x, self.y)
            .radius(self.radius)
            .color(self.color);
    }
}

struct Model {
    seed: u64,
    hue: f32,
    num: u32,
    particles: Vec<Particle>,
}

impl Model {
    fn new() -> Self {
        let particles = Vec::new();

        Model {
            hue: 0.0,
            seed: 0,
            num: 1,
            particles,
        }
    }

    // fn rng(&self) -> StdRng {
    //     StdRng::seed_from_u64(self.seed)
    // }

    fn reset_seed(&mut self) -> &mut Self {
        self.seed = random_range(0, 1000000);
        self
    }

    fn reset_hue(&mut self) -> &mut Self {
        self.hue = rng(self.seed).gen_range(0.0..360.0);
        self
    }

    fn reset_num(&mut self, app: &App) -> &mut Self {
        let rect = app.window_rect().pad(MARGIN);
        // set num based on the area of the window
        self.num = (rect.w() * rect.h() / 10000.0) as u32;
        self
    }

    fn generate(&mut self, app: &App) -> &mut Self {
        let rect = app.window_rect().pad(MARGIN);
        let mut rng = rng(self.seed);

        self.particles = Vec::new();

        for _ in 0..self.num {
            let color = Color::randomize(&mut rng)
                .set_hue(self.hue)
                .set_alpha(1.0)
                .into();

            let dot = Particle {
                x: rng.gen_range(-0.5..0.5) * rect.w(),
                y: rng.gen_range(-0.5..0.5) * rect.h(),
                radius: rng.gen_range(1.0..64.0),
                color,
            };
            self.particles.push(dot);
        }

        self
    }

    fn display(&self, draw: &nannou::Draw) {
        for particle in &self.particles {
            particle.display(&draw);
        }
    }

    fn reset(&mut self, app: &App) {
        self.reset_seed().reset_hue().reset_num(&app).generate(&app);
    }

    fn add_particles(&mut self, app: &App, num: i32) -> &mut Self {
        self.num = (self.num as i32 + num).max(1) as u32;
        self.generate(&app);
        self
    }
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
