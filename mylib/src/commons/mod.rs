pub mod image;

use openssl::hash::{Hasher, MessageDigest};
use opencv::core;

#[derive(Debug, Clone, Copy)]
pub struct ImageResize {
    pub width: i32,
    pub height: i32,
    pub vertical_border: i32,
    pub horizontal_border: i32
}

pub fn get_hash(buffer: &[u8]) -> String {
    let mut h = Hasher::new(MessageDigest::md5()).unwrap();
    h.update(buffer).unwrap();
    hex::encode(h.finish().unwrap())
}

pub fn get_target_size(img_ref: core::Size, width: i32 , height: i32 ) -> ImageResize {

    let radio: f32 = min( width as f32/ img_ref.width as f32, height as f32/ img_ref.height as f32);
    let mut new_width : i32 = (img_ref.width as f32 * radio) as i32;
    let mut new_height : i32 = (img_ref.height as f32 * radio) as i32;
    let mut v_border = 0;
    let mut h_border = 0;

    if new_height > new_width {
        let border: f32 = (new_height - new_width) as f32 / 2.0;
        h_border = border as i32;
        new_width += (border as i32 % 1) * 2;
    } else if new_width > new_height {
        let border: f32 = (new_width - new_height) as f32 / 2.0;
        v_border = {border as i32};
        new_height += ((border as i32) % 1) * 2;
    } else {
        v_border = 0;
        h_border = 0;
    }

    ImageResize {
        width: new_width,
        height: new_height,
        vertical_border: v_border,
        horizontal_border: h_border
    }
}

fn min(n1: f32, n2: f32) -> f32 {

    if n1 < n2 {
        n1
    } else if n2 < n1 {
        n2
    } else {
        n1
    }
}