use nannou::color::{Lcha, Rgba};
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

    fn reset_hue(&mut self) -> &mut Self {
        self.hue = rng(self.seed).gen_range(0.0..360.0);
        self
    }

    fn generate(&mut self, app: &App) -> &mut Self {
        let rect = app.window_rect().pad(MARGIN);
        let mut rng = rng(self.seed);

        self.particles = Vec::new();

        for _ in 0..self.num {
            // you can find details about the LCHA in nannou here:
            // https://docs.rs/nannou/latest/nannou/color/struct.Lch.html#fields
            // we have a hue, and want to generate particles with varying
            // lightness and chroma, so we use LCHA
            // for particles, we want neither too dark nor too light
            let color: Rgba = Lcha::new(
                rng.gen_range(40.0..90.0),
                rng.gen_range(40.0..60.0),
                self.hue,
                0.25 * 0.75 + rng.gen::<f32>(),
            )
            // notice the .into() here
            // this is a trait that converts from one type to another
            // in this case, it converts from Lcha to Rgba
            // which is expected by our Particle struct
            .into();

            let dot = Particle {
                x: rng.gen_range(-0.5..0.5) * rect.w(),
                y: rng.gen_range(-0.5..0.5) * rect.h(),
                radius: rng.gen_range(10.0..80.0),
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
        self.reset_seed().reset_hue().generate(&app);
    }

    fn add_particles(&mut self, app: &App, num: i32) -> &mut Self {
        // python: self.num = max(1, self.num + num);
        // the typecasting here is wrong since we are casting num to i32
        // and then back to u32, which would give wrong results for large
        // numbers, **silently**.
        // But, for our sample problem, this works without complications.
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
            // since the rng generator is initialized with the seed
            // in generate fn, we can just call generate again
            // to add/remove particles and not reset existing
            // particles.
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

    // instead of using SNOW by default,
    // lets hook into the LCHA color space
    // and use the hue in the model in a very light background
    let bg_color: Rgba = Lcha::new(90.0, 12.8, model.hue, 1.0).into();
    draw.background().color(bg_color);

    // use display method to draw all particles
    model.display(&draw);

    draw.to_frame(app, &frame).unwrap();
}
