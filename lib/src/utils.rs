use nannou::rand::rngs::StdRng;
use nannou::rand::Rng;
use nannou::rand::SeedableRng;

pub fn rng(seed: u64) -> StdRng {
    StdRng::seed_from_u64(seed)
}

// random number between min and max with exponential distribution
// k is the exponential distribution parameter
// favor_min will favor min if true, max if false
pub fn exp_rng(rng: &mut StdRng, k: f32, min: f32, max: f32, favor_min: bool) -> f32 {
    let r = rng.gen::<f32>();
    let s = -(1.0 - r).ln() / k; // exponential distribution
    let t = if favor_min { s } else { 1.0 - s }; // favor min or max
    (min / max + t) * max
}

// wave function with parameters a, b, c, d
// https://cis700-procedural-graphics.github.io/files/color_2_14_17.pdf
// for smooth transitions between x and y
//
// a : shifts the entire curve up or down along the y-axis, i.e. a controls the offset of the wave
// b : scales the amplitude of the wave, i.e. b controls the height of the wave
// c : scales the frequency of the wave, i.e. c controls the width of the wave
// d : shifts the entire curve left or right along the x-axis, i.e. d controls the phase of the wave
//
// this function is really useful for creating smooth transitions between colors
//
pub fn wave(x: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    let pi = std::f32::consts::PI;
    a + b * (2.0 * pi * (c * x + d)).cos()
}

// wave function with default parameters and varying phase
pub fn wave_d(x: f32, d: f32) -> f32 {
    wave(x, 0.5, 0.5, 0.5, d)
}

// wave function with default parameters
pub fn wave_r(x: f32) -> f32 {
    wave(x, 0.5, 0.5, 0.5, 0.5)
}
