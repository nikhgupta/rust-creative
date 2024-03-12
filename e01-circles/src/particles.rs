use lib::colors::Color;

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
        let color = Color::randomize(rng).set_hue(hue).set_alpha(1.0).into();

        Particle {
            x,
            y,
            color,
            radius: rng.gen_range(1.0..64.0),
        }
    }

    pub fn display(&self, draw: &nannou::Draw) {
        draw.ellipse()
            .x_y(self.x, self.y)
            .radius(self.radius)
            .color(self.color);
    }
}
