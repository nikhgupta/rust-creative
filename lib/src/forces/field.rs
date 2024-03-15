use nannou::{
    math::map_range,
    noise::{NoiseFn, Seedable},
    prelude::{Vec2, TAU},
    rand::random_range,
};

pub trait ForceField {
    fn get(&self, i: f64, j: f64) -> Vec2;
}

pub struct PerlinField {
    pub strength: f64,
    pub scale: f64,
    pub noise: nannou::noise::Perlin,
}

impl PerlinField {
    pub fn new(strength: f64, scale: f64, seed: u32) -> Self {
        PerlinField {
            scale,
            strength,
            noise: nannou::noise::Perlin::new().set_seed(seed),
        }
    }
}

impl ForceField for PerlinField {
    fn get(&self, i: f64, j: f64) -> Vec2 {
        let noise = self.noise.get([i * self.scale, j * self.scale]);
        let noise = map_range(noise, 0.0, 1.0, 0.0, TAU);
        Vec2::new(
            noise.cos() * self.strength as f32,
            noise.sin() * self.strength as f32,
        )
    }
}

pub struct AttractorField {
    strength: f64,
    i: f64,
    j: f64,
}

impl AttractorField {
    pub fn new(strength: f64, i: usize, j: usize) -> Self {
        AttractorField {
            i: i as f64,
            j: j as f64,
            strength,
        }
    }
}

impl ForceField for AttractorField {
    fn get(&self, i: f64, j: f64) -> Vec2 {
        let dx = self.i - i;
        let dy = self.j - j;
        let distance = (dx * dx + dy * dy).sqrt();
        let decay = (self.strength / distance.max(1.0).powf(1.0 / 3.0)) as f32;
        let angle = dy.atan2(dx) as f32;
        return Vec2::new(angle.cos() * decay, angle.sin() * decay);
    }
}

pub struct GradientField {
    pub strength: f64,
    pub angle: f64,
}

impl GradientField {
    pub fn new(strength: f64, angle: f64) -> Self {
        GradientField { strength, angle }
    }
}

impl ForceField for GradientField {
    fn get(&self, _i: f64, _j: f64) -> Vec2 {
        Vec2::new(self.angle.cos() as f32, self.angle.sin() as f32) * self.strength as f32
    }
}

pub struct RandomField {
    pub strength: f64,
}

impl RandomField {
    pub fn new(strength: f64) -> Self {
        RandomField { strength }
    }
}

impl ForceField for RandomField {
    fn get(&self, _i: f64, _j: f64) -> Vec2 {
        let angle = random_range(0.0, TAU as f32);
        Vec2::new(
            angle.cos() * self.strength as f32,
            angle.sin() * self.strength as f32,
        )
    }
}
