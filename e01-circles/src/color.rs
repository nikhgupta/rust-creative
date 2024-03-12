use nannou::color::{Lcha, Rgba};
use nannou::rand::rngs::StdRng;
use nannou::rand::Rng;

pub struct Color {
    l: f32,
    c: f32,
    h: f32,
    a: f32,
}

impl Color {
    pub fn new(l: f32, c: f32, h: f32, a: f32) -> Self {
        Color { l, c, h, a }
    }

    pub fn randomize(rng: &mut StdRng) -> Self {
        let l = rng.gen_range(0.5..0.9);
        let c = rng.gen_range(0.05..0.15);
        let h = rng.gen_range(0.0..360.0);
        let a = rng.gen_range(0.25..1.0);
        Color::new(l, c, h, a)
    }

    pub fn set_hue(&self, hue: f32) -> Self {
        Color::new(self.l, self.c, hue, self.a)
    }

    pub fn set_alpha(&self, alpha: f32) -> Self {
        Color::new(self.l, self.c, self.h, alpha)
    }
}

impl Into<Rgba> for Color {
    fn into(self) -> Rgba {
        Lcha::new(self.l * 100.0, self.c * 128.0, self.h * 360.0, self.a).into()
    }
}
