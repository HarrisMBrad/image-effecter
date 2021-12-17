use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
  log(&"Greyscale called".into());

  let base64_to_vector = decode(encoded_file).unwrap();
  log(&"Image decoded".into());

  let mut _img = load_from_memory(&base64_to_vector).unwrap();
  log(&"Image loaded".into());

  _img = _img.grayscale();
  log(&"Grayscale effect applied".into());

  let mut buffer = vec![];
  _img.write_to(&mut buffer, Png).unwrap();
  log(&"New image written".into());

  let encoded_img = encode(&buffer);
  let data_url = format!("data:image/png;base64,{}", encoded_img);
  data_url
}
