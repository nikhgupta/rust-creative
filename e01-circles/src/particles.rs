use lib::colors::Color;
use lib::utils::exp_rng;
use nannou::color::Rgba;
use nannou::rand::rngs::StdRng;
use nannou::rand::Rng;

pub struct Particle {
    x: f32,
    y: f32,
    radius: f32,
    color: Rgba,
}

impl Particle {
    pub fn random(x: f32, y: f32, hue: f32, rng: &mut StdRng) -> Self {
        let l = rng.gen_range(0.70..0.95);
        let c = rng.gen_range(0.15..0.35);
        let color = Color::new(l, c, hue, 1.0).into();
        let radius = exp_rng(rng, 20.0, 2.0, 256.0, true);

        Particle {
            x,
            y,
            color,
            radius,
        }
    }

    pub fn display(&self, draw: &nannou::Draw) {
        draw.ellipse()
            .x_y(self.x, self.y)
            .radius(self.radius)
            .color(self.color);
    }
}
