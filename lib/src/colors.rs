use nannou::color::white_point::D65;
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
        let l = rng.gen_range(0.0..1.0);
        let c = rng.gen_range(0.0..1.0);
        let h = rng.gen_range(0.0..1.0);
        let a = rng.gen_range(0.0..1.0);
        Color::new(l, c, h, a)
    }

    pub fn set_hue(&self, hue: f32) -> Self {
        Color::new(self.l, self.c, hue, self.a)
    }

    pub fn set_alpha(&self, alpha: f32) -> Self {
        Color::new(self.l, self.c, self.h, alpha)
    }
}

// Lcha expects a generic Wp (whitepoint).
// D65 is a standard whitepoint.
// we can use generics for Color class to specify whitepoint,
// but it's not necessary for this example.
// Primarily, its too early to use generics in our journey, IMO.
impl Into<Lcha<D65, f32>> for Color {
    fn into(self) -> Lcha<D65, f32> {
        Lcha::new(self.l * 100.0, self.c * 128.0, self.h * 360.0, self.a)
    }
}

// use the Color to lcha conversion to convert to rgba
impl Into<Rgba> for Color {
    fn into(self) -> Rgba {
        let lcha: Lcha<D65, f32> = self.into();
        lcha.into()
    }
}
