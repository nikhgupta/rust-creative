use nannou::rand::rngs::StdRng;
use nannou::rand::SeedableRng;

pub fn rng(seed: u64) -> StdRng {
    StdRng::seed_from_u64(seed)
}
