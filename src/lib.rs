use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use std::io::Cursor;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale called.".into());
    let base64_to_vec: Vec<u8> = decode(encoded_file).expect("Failed to decode base64");
    log(&"Image decoded".into());

    let mut img = load_from_memory(&base64_to_vec).expect("faild to load image");
    log(&"Image loaded".into());

    img = img.grayscale();
    log(&"Image grayscaled".into());

    let mut buffer = vec![];
    img.write_to(&mut Cursor::new(&mut buffer), Png)
        .expect("Failed to write image");
    log(&"Image written".into());

    let encoded_img = encode(&buffer);
    format!("data:image/png;base64,{}", encoded_img)
}
