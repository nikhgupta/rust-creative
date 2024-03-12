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
    particles: Vec<Dot>,
}

// convert the earlier code to use an impl block. (do compare the diff side by side.)
// https://doc.rust-lang.org/rust-by-example/trait/impl.html
// this allows us to group methods together
// and use the `self` keyword to refer to the struct
impl Model {
    fn new() -> Self {
        let seed = 0;
        let particles = Vec::new();

        // the following is a shorthand for
        // Model { seed: seed, particles: particles }
        // you can initialize the values here as well, e.g.
        // Model { seed: 0, particles: Vec::new() }
        Model { seed, particles }
    }

    // fn rng(&self) -> StdRng {
    //     StdRng::seed_from_u64(self.seed)
    // }

    fn reset_seed(&mut self) -> &mut Self {
        self.seed = random_range(0, 1000000);
        // Rust returns the last value in the block
        // (be careful that it does not end with a semicolon)
        // this allows us to chain methods together and
        // hook into the builder pattern
        self
    }

    fn generate(&mut self) -> &mut Self {
        self.particles = Vec::new();
        let mut rng = StdRng::seed_from_u64(self.seed);

        // for now we will add a single dot in this refactor
        // we will add a loop later to introduce more dots
        let dot = Dot {
            x: rng.gen_range(-0.4..0.4) * WIDTH as f32 / 2.0,
            y: rng.gen_range(-0.4..0.4) * HEIGHT as f32 / 2.0,
            radius: rng.gen_range(40.0..200.0),
            color: Rgba::new(
                rng.gen(),
                rng.gen(),
                rng.gen(),
                // f32 required here.
                // vs-code allows us to see the function signature,
                // which is helpful in such cases and helped in understanding
                // how to define the type for this function.
                0.25 * 0.75 + rng.gen::<f32>(),
            ),
        };

        // we can use the push method to add a single dot to the vector
        // remember to set self.particles to Vec::new() before adding the dot
        // otherwise, the dots will accumulate on pressing Space
        self.particles.push(dot);

        // return self to allow chaining methods
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
    model.reset_seed().generate();
    model
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            // look how simple it became to reset the state!
            // if this starts getting complex, we can add a method to the Model
            // for resetting the entire state and move this code over there.
            model.reset_seed().generate();
        }
        // unmatched keys are ignored
        _ => (),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(SNOW);

    // looping through the particles and drawing them based on their properties
    for particle in &model.particles {
        draw.ellipse()
            .x_y(particle.x, particle.y)
            .radius(particle.radius)
            .color(particle.color);
    }

    draw.to_frame(app, &frame).unwrap();
}
