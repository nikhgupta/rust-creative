// since this module is included in the main.rs file,
// we can refer to some of the types and functions from the main.rs file
// without having to import them by using the crate:: prefix.
// for example, we can refer to the App type and the random_range function.
use crate::particles::Particle;
use crate::{random_range, App};

use lib::utils::rng;
use nannou::rand::Rng;

const MARGIN: f32 = 40.0;

// The implementation and struct is similar to the previous example,
// but we explicitely mark public fields and methods that we want to expose.
pub struct Model {
    pub hue: f32,
    seed: u64,
    num: u32,
    particles: Vec<Particle>,
}

impl Model {
    pub fn new() -> Self {
        let particles = Vec::new();

        Model {
            hue: 0.0,
            seed: 0,
            num: 1,
            particles,
        }
    }

    pub fn reset(&mut self, app: &App) {
        self.reset_seed().reset_hue().reset_num(&app).generate(&app);
    }

    pub fn add_particles(&mut self, app: &App, num: i32) -> &mut Self {
        self.num = (self.num as i32 + num).max(1) as u32;
        self.generate(&app);
        self
    }

    pub fn display(&self, draw: &nannou::Draw) {
        for particle in &self.particles {
            particle.display(&draw);
        }
    }

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
        self.num = (rect.w() * rect.h() / 10000.0) as u32;
        self
    }

    fn generate(&mut self, app: &App) -> &mut Self {
        let rect = app.window_rect().pad(MARGIN);
        let mut rng = rng(self.seed);

        self.particles = Vec::new();

        for _ in 0..self.num {
            let particle = Particle::random(
                rng.gen_range(-0.5..0.5) * rect.w(),
                rng.gen_range(-0.5..0.5) * rect.h(),
                self.hue,
                &mut rng,
            );
            self.particles.push(particle);
        }

        self
    }
}
