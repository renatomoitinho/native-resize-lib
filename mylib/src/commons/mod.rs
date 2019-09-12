pub mod image;
pub mod boxes;

use jni::JNIEnv;
use jni::objects::{GlobalRef};
use openssl::hash::{Hasher, MessageDigest};
use opencv::core;

#[derive(Debug, Clone, Copy)]
pub struct ImageResize {
    pub width: i32,
    pub height: i32,
    pub vertical_border: i32,
    pub horizontal_border: i32
}

pub struct ImageRef {
    pub mat: Option<core::Mat>,
    pub width: i32,
    pub height: i32,
    callback: Option<GlobalRef>
}

impl ImageRef {
    pub fn new (buffer: Vec<u8>, global_ref: Option<GlobalRef>) -> ImageRef {

        let mat_src = image::load_buffer(&buffer).unwrap();
        let (real_height, real_width ) = (mat_src.rows().unwrap(), mat_src.cols().unwrap());

        ImageRef {
            mat: Some(mat_src),
            width: real_width,
            height: real_height,
            callback: global_ref
        }
    }

    pub fn update_mat(&mut self, src: &core::Mat) {
        let mut result = core::Mat::default().unwrap();
        src.copy_to(&mut result).unwrap();
        self.mat = Some(result);
        self.update_size();
    }

    fn update_size(&mut self) {

        let src = self.mat.as_ref().unwrap();

        self.width = src.cols().unwrap();
        self.height = src.rows().unwrap();
    }

    pub fn call_reference(&mut self, env: &JNIEnv) {

        let gl = self.callback.as_ref();

        env.call_method(gl.unwrap().as_obj(), "setWidth", "(I)V", &[self.width.into()], ).unwrap();
        env.call_method(gl.unwrap().as_obj(), "setHeight", "(I)V", &[self.height.into()], ).unwrap();
    }
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