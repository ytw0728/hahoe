pub mod basics;
pub mod webgl;
pub mod interaction;

pub use basics::GuiBasics;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast};

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) -> () {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
