mod utils;

extern crate imagelib;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn blur(v: &str) -> String {
    imagelib::transform_image_b64(v, imagelib::Operation::Blur)
}

#[wasm_bindgen]
pub fn invert(v: &str) -> String {
    imagelib::transform_image_b64(v, imagelib::Operation::Invert)
}

#[wasm_bindgen]
pub fn greyscale(v: &str) -> String {
    imagelib::transform_image_b64(v, imagelib::Operation::Greyscale)
}

#[wasm_bindgen]
pub fn rotate(v: &str) -> String {
    imagelib::transform_image_b64(v, imagelib::Operation::Rotate)
}

#[wasm_bindgen]
pub fn vertical_flip(v: &str) -> String {
    imagelib::transform_image_b64(v, imagelib::Operation::VFlip)
}

#[wasm_bindgen]
pub fn horizontal_flip(v: &str) -> String {
    imagelib::transform_image_b64(v, imagelib::Operation::HFlip)
}