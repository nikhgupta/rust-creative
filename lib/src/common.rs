pub trait Seedable {
    fn set_seed(&mut self, seed: u32) -> &mut Self;
    fn seed(&self) -> u32;
}
