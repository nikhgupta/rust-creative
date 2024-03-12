use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const MARGIN: f32 = 40.0;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
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
    num: u32,
    particles: Vec<Particle>,
}

impl Model {
    fn new() -> Self {
        let particles = Vec::new();

        Model {
            seed: 0,
            num: 100,
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

    fn generate(&mut self, app: &App) -> &mut Self {
        self.particles = Vec::new();
        let mut rng = StdRng::seed_from_u64(self.seed);

        // instead of relying on fixed width and height,
        // we want to use window dimensions on the fly.
        // added a margin to the window dimensions to avoid particles
        // from sticking to the edges of the window.
        let rect = app.window_rect().pad(MARGIN);

        // we want to add a bunch of particles to the model
        for _ in 0..self.num {
            let dot = Particle {
                x: rng.gen_range(-0.5..0.5) * rect.w(),
                y: rng.gen_range(-0.5..0.5) * rect.h(),
                radius: rng.gen_range(40.0..200.0),
                color: Rgba::new(
                    rng.gen(),
                    rng.gen(),
                    rng.gen(),
                    0.25 * 0.75 + rng.gen::<f32>(),
                ),
            };
            self.particles.push(dot);
        }

        self
    }

    // instead, of using a for loop to draw each particle in view,
    // we can write a display function in the impl block of Model
    // and delegate the drawing to each particle.
    // this way, we can keep the view function clean and simple.
    fn display(&self, draw: &nannou::Draw) {
        for particle in &self.particles {
            // delegate the drawing to each particle
            particle.display(&draw);
        }
    }

    fn reset(&mut self, app: &App) {
        self.reset_seed().generate(&app);
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
    model.reset_seed().generate(app);
    model
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.reset(app);
        }
        // add keyboard controls to increase or decrease the number of particles
        // for now, this resets all particles.
        // later, we will add or remove particles without resetting all of them.
        Key::Up => {
            model.num += 10;
            model.reset(app);
        }
        Key::Down => {
            if model.num > 10 {
                model.num -= 10;
                model.reset(app);
            }
        }
        _ => (),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(SNOW);

    // use display method to draw all particles
    model.display(&draw);

    draw.to_frame(app, &frame).unwrap();
}
