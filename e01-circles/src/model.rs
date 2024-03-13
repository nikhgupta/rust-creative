use crate::particles::Particle;
use crate::random_range;

use lib::utils::rng;
use nannou::rand::Rng;

pub struct Model {
    pub hue: f32,
    pub num: u32,
    pub seed: u64,
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

    pub fn reset_num(&mut self, w: f32, h: f32) -> &mut Self {
        let r = rng(self.seed).gen_range(1000.0..10000.0);
        self.num = (w * h / r).max(1.0) as u32;
        self
    }

    pub fn reset(&mut self, w: f32, h: f32) -> &mut Self {
        self.reset_num(w, h)
            .reset_seed()
            .reset_hue()
            .generate_particles(w, h);
        self
    }

    pub fn num(&mut self, num: u32) -> &mut Self {
        self.num = num.max(1);
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
        self.hue = rng(self.seed).gen_range(0.0..1.0);
        self
    }

    pub fn generate_particles(&mut self, w: f32, h: f32) -> &mut Self {
        let mut rng = rng(self.seed);

        self.particles = Vec::new();

        for _ in 0..self.num {
            let particle = Particle::random(
                rng.gen_range(-0.5..0.5) * w,
                rng.gen_range(-0.5..0.5) * h,
                self.hue,
                &mut rng,
            );
            self.particles.push(particle);
        }

        self
    }
}
