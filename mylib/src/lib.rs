pub mod commons;

// This is the interface to the JVM that we'll
// call the majority of our methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native function.
// They carry extra lifetime information to prevent them escaping this context
// and getting used after being GC'd.
use jni::objects::{JClass, JObject, JString};

// This is just a pointer. We'll be returning it from our function.
// We can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jbyteArray, jint, jlong};

//JNIEXPORT jlong JNICALL Java_image_HelloWorld_createImageReference
//(JNIEnv *, jclass, jbyteArray, jobject);
#[no_mangle]
pub unsafe extern "system" fn Java_image_HelloWorld_createImageReference(
    _env: JNIEnv,
    _class: JClass,
    input: jbyteArray,
    callback: JObject) -> jlong {
    commons::boxes::create_new_reference(&_env, input, callback)
}

#[no_mangle]
pub unsafe extern "system" fn Java_image_HelloWorld_destroyReference(
    _env: JNIEnv,
    _class: JClass,
    reference_id: jlong,
) {
    commons::boxes::destroy_reference(reference_id)
}

#[no_mangle]
pub unsafe extern "system" fn Java_image_HelloWorld_resize(
    _env: JNIEnv,
    _class: JClass,
    reference_id: jlong,
    width: jint,
    height: jint,
    quality: jint,
    format: JString,
) -> jbyteArray {
    let input: String = _env
        .get_string(format)
        .expect("Couldn't get java string!")
        .into();

    let buffer = commons::boxes::box_resize(&_env, reference_id, width as i32, height as i32, quality as i32, &input).unwrap();
    let output = _env.byte_array_from_slice(&buffer).unwrap();

    output
}

#[no_mangle]
pub unsafe extern "system" fn Java_image_HelloWorld_scale(
    env: JNIEnv,
    _class: JClass,
    reference_id: jlong,
    width: jint,
    height: jint,
    quality: jint,
    format: JString,
) -> jbyteArray {
    let input: String = env
        .get_string(format)
        .expect("Couldn't get java string!")
        .into();

    let buffer = commons::boxes::box_scale(&env, reference_id, width as i32, height as i32, quality as i32, &input).unwrap();

    env.byte_array_from_slice(&buffer).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}