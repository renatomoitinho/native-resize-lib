use crate::commons::ImageResize;
use crate::commons::ImageRef;
use crate::commons::image::*;
use crate::commons::get_target_size;
use opencv::core;

use jni::JNIEnv;
use jni::objects::{GlobalRef, JClass, JObject, JString};
use jni::sys::{jbyteArray, jint, jlong, jstring};

pub fn get_size_ref(img_ref: &ImageRef) -> core::Size {
    core::Size {
        width: img_ref.width,
        height: img_ref.height
    }
}

fn get_size_rez(img_resize: ImageResize) -> core::Size {
    core::Size {
        width: img_resize.width,
        height: img_resize.height
    }
}

pub fn create_new_reference(env: JNIEnv, input: jbyteArray, obj: JObject) -> jlong {

    let global_ref = env.new_global_ref(obj).unwrap();
    let buffer: Vec<u8> = env.convert_byte_array(input).unwrap();
    let mut img_ref = ImageRef::new(buffer, Some(global_ref));
    img_ref.augment(env);
    Box::into_raw(Box::new(img_ref)) as jlong
}

pub unsafe fn box_resize(reference_id: jlong, width: i32, height: i32, quality: i32, format: &str) -> Result<Vec<u8>, opencv::Error> {
    let image_ref = &mut *(reference_id as *mut ImageRef);
    let mut result = core::Mat::default()?;
    image_ref.mat.copy_to(&mut result).unwrap();
    let img_size = get_target_size(get_size_ref(image_ref), width, height);
    result = resize(result, get_size_rez(img_size) ).unwrap();
    Ok(get_buffer(result, quality, format).unwrap())
}

pub unsafe fn destroy_reference(reference_id: jlong) {
    let image_ref = Box::from_raw(reference_id as *mut ImageRef);
}
