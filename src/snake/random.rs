#[cfg(target_arch = "wasm32")]
use macroquad::rand::gen_range;

#[cfg(not(target_arch = "wasm32"))]
use rand::Rng;

#[cfg(target_arch = "wasm32")]
pub fn random_range(min: i16, max: i16) -> i16 {
    gen_range(min, max)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn random_range(min: i16, max: i16) -> i16 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}
