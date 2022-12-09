use std::io::Cursor;

use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;

use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"grayscale called".into());
    
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"image loaded".into());

    img = img.grayscale();
    log(&"Gray scale effect applied".into());

    let mut buffer = vec![];
    img.write_to(&mut Cursor::new(&mut buffer), Png).unwrap();
    log(&"New image written".into());

    let endocded_img = encode(&buffer);
    let data_url = format!(
        "data:image/png;base64,{}",
        endocded_img
    );

    data_url
}