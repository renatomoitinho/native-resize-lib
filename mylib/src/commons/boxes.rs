use crate::commons::ImageRef;

use jni::JNIEnv;
use jni::objects::{GlobalRef, JClass, JObject, JString};
use jni::sys::{jbyteArray, jint, jlong, jstring};

pub fn create_new_reference(env: JNIEnv, input: jbyteArray, obj: JObject) -> jlong {

    let global_ref = env.new_global_ref(obj).unwrap();
    let buffer: Vec<u8> = env.convert_byte_array(input).unwrap();
    let mut img_ref = ImageRef::new(buffer, Some(global_ref));
    img_ref.augment(env);
    Box::into_raw(Box::new(img_ref)) as jlong
}