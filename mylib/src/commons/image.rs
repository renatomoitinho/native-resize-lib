use crate::commons::ImageResize;
use std::fs;
use opencv::core;
use opencv::imgproc;
use opencv::imgcodecs;
use opencv::types::{VectorOfint, VectorOfuchar};
use opencv::prelude::Vector;

const WHITE_COLOR: f64 = 255 as f64;

pub fn get_size(_width: i32, _height: i32) -> core::Size {
    core::Size {
        width: _width,
        height: _height,
    }
}

pub fn default_white_scalar() -> core::Scalar {
    core::Scalar::new(WHITE_COLOR, WHITE_COLOR, WHITE_COLOR, WHITE_COLOR)
}

pub fn expand(src: &core::Mat, resize: ImageResize) -> Result<core::Mat, opencv::Error> {
    let mut result = core::Mat::default()?;

    core::copy_make_border(src, &mut result, resize.vertical_border, resize.vertical_border,
                           resize.horizontal_border, resize.horizontal_border, core::BORDER_CONSTANT,
                           default_white_scalar()).expect("not load buffer");
    Ok(result)
}

pub fn resize(img_ref: &core::Mat, size: core::Size) -> Result<core::Mat, opencv::Error> {
    let mut result = core::Mat::default()?;
    let img: &core::Mat = img_ref;

    imgproc::resize(
        img,
        &mut result,
        size,
        0f64,
        0f64,
        imgproc::INTER_LINEAR,
    ).expect("");

    Ok(result)
}

pub fn load_buffer(buffer: &[u8]) -> Result<core::Mat, opencv::Error> {
    let mat_buf = core::Mat::from_slice(&buffer)?;
    let src_mat = imgcodecs::imdecode(&mat_buf, imgcodecs::IMREAD_UNCHANGED)
        .expect("");

    Ok(src_mat)
}

pub fn load_image_from_disk(url: &str) -> Vec<u8> {
    fs::read(url).unwrap()
}

pub fn read_image_direct_mat(image: &str) -> Result<core::Mat, opencv::Error> {
    let mat = imgcodecs::imread(image, 0i32).unwrap();
    Ok(mat)
}

pub fn get_buffer(src: &core::Mat, q: i32, format: &str) -> Result<Vec<u8>, opencv::Error> {
    let quality = get_format(format, q);
    let mut rs_buf = VectorOfuchar::new();

    imgcodecs::imencode(format!(".{}", format.to_lowercase()).as_str(), src, &mut rs_buf, &quality)
        .expect("create a buffer image failed");

    Ok(rs_buf.to_vec())
}

pub fn write_on_disc(src: core::Mat, path: &str, q: i32, format: &str) {
    let quality = get_format(format, q);
    imgcodecs::imwrite(path, &src, &quality)
        .expect("");
}

pub fn get_format(format: &str, q: i32) -> VectorOfint {
    let mut quality = VectorOfint::with_capacity(2);
    match format {
        "JPG" | "JPEG" => {
            quality.push(q);
            quality.push(imgcodecs::IMWRITE_JPEG_QUALITY);
        }
        "PNG" => {
            quality.push(q);
            quality.push(imgcodecs::IMWRITE_PNG_COMPRESSION);
        }
        _ => {
            panic!("not support format {:?}", format);
        }
    }
    quality
}
