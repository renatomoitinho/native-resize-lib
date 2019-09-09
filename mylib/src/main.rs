mod commons;

use std::fs;
use std::time;
use opencv::core;
use opencv::imgcodecs;
use commons::*;
use opencv::imgproc;

use opencv::types::{VectorOfint, VectorOfuchar};
use opencv::prelude::Vector;


const WHITE_COLOR : f64 = 255 as f64;

#[derive(Debug)]
struct ImageResize {
    width: i32,
    height: i32,
    vertical_border: i32,
    horizontal_border: i32
}

impl ImageResize {
    pub fn new (img_resize: ImageResize) -> ImageResize {
        ImageResize {
            width: img_resize.width,
            height: img_resize.height,
            vertical_border: img_resize.vertical_border,
            horizontal_border: img_resize.horizontal_border
        }
    }
}

#[derive(Debug)]
struct ImageRef {
    mat: core::Mat,
    size: i64,
  //  hash: String,
    width: i32,
    height: i32,
}
impl ImageRef {
    pub fn new (buffer: Vec<u8>) -> ImageRef {

        let img_size = buffer.len() as i64;
        //let hash_md5 = get_hash(buffer.as_slice());
        let mat_src = load_image_open_cv(&buffer).unwrap();
        let (real_height, real_width ) = (mat_src.rows().unwrap(), mat_src.cols().unwrap());

        ImageRef {
            mat: mat_src,
            size: img_size,
           // hash: hash_md5,
            width: real_width,
            height: real_height,
        }
    }
}

fn default_scalar() -> core::Scalar {
    core::Scalar::new(WHITE_COLOR, WHITE_COLOR, WHITE_COLOR, WHITE_COLOR)
}

fn get_target_size(img_ref: core::Size, width: i32 , height: i32 ) -> ImageResize {

    let radio: f32 = min( width as f32/ img_ref.width as f32, height as f32/ img_ref.height as f32);
    let mut new_width : i32 = (img_ref.width as f32 * radio) as i32;
    let mut new_height : i32 = (img_ref.height as f32 * radio) as i32;
    let v_border;
    let h_border;

    if new_height > new_width {
        let border: f32 = (new_height - new_width) as f32 / 2.0;
        h_border = border as i32;
        v_border = 0;
        new_width += (border as i32 % 1) * 2;
    } else if new_width > new_height {
        let border: f32 = (new_height - new_width) as f32 / 2.0;
        v_border = {border as i32};
        h_border = 0;
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

fn load_image_from_disk(url: String) -> Vec<u8> {
    fs::read(url).unwrap()
}

fn load_image_open_cv(buffer: &[u8]) -> Result<core::Mat,opencv::Error> {
    let mat_buf = core::Mat::from_slice(&buffer)?;
    let src_mat = imgcodecs::imdecode(&mat_buf, -1)?;

    Ok(src_mat)
}

fn resize(img_ref: core::Mat, size: core::Size) -> Result<core::Mat, opencv::Error> {
    let mut result = core::Mat::default()?;;
    let img: &core::Mat = &img_ref;

    imgproc::resize(
        img,
        &mut result,
        size,
        0f64,
        0f64,
        imgproc::INTER_LINEAR,
    )?;

    Ok(result)
}

fn write_on_disc(src: core::Mat) {
    let mut quality = VectorOfint::with_capacity(2);
    quality.push(80);
    quality.push(imgcodecs::IMWRITE_JPEG_QUALITY);
    imgcodecs::imwrite("/Users/renatomoitinho/Documents/repositories/rust-lang/imgs/1mb_1.jpeg", &src, &quality).unwrap();
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

fn get_mat_as_buffer(src: core::Mat) -> Result<Vec<u8>, opencv::Error> {
    let mut rs_buf = VectorOfuchar::new();
    let mut quality = VectorOfint::with_capacity(2);
    quality.push(80);
    quality.push(imgcodecs::IMWRITE_JPEG_QUALITY);

    imgcodecs::imencode(".jpg", &src, &mut rs_buf, &quality).expect("create a buffer image failed");

    Ok(rs_buf.to_vec())
}

fn fill(src: core::Mat, resize: ImageResize) -> Result<core::Mat, opencv::Error> {
    let mut result = core::Mat::default()?;
    core::copy_make_border(&src, &mut result,
                           resize.vertical_border, resize.vertical_border,
                           resize.horizontal_border, resize.horizontal_border,
                           core::BORDER_CONSTANT, default_scalar()).expect("not load buffer");
    Ok(result)
}

//fn read_image_direct_mat(image: &str) -> Result<core::Mat, opencv::Error> {
//    let mat = imgcodecs::imread(image, 0i32).unwrap();
//    Ok(mat)
//}

fn get_size_rez(img_resize: &ImageResize) -> core::Size {
    core::Size {
        width: img_resize.width,
        height: img_resize.height
    }
}

fn get_size_ref(img_ref: &ImageRef) -> core::Size {
    core::Size {
        width: img_ref.width,
        height: img_ref.height
    }
}

fn main() {

    // read image from disk
    let mut buffer = load_image_from_disk("/Users/renatomoitinho/Documents/repositories/rust-lang/imgs/png/1mb.png".parse().unwrap());

    let start_1 = time::Instant::now();

    // load image from buffer
    let start_2 = time::Instant::now();
    let img_ref = ImageRef::new(buffer);
    println!("time to load image from buffer => {:?}", start_2.elapsed() );

    // target new size
    let img_resize = get_target_size(get_size_ref(&img_ref), 1000, 1000);

    // resize
    let start_3 = time::Instant::now();
    let mut new_img = resize(img_ref.mat, get_size_rez(&img_resize)).unwrap();
    println!("time to resize image => {:?}", start_3.elapsed() );

    // fill
    let start_4 = time::Instant::now();
    new_img = fill(new_img, ImageResize::new(img_resize)).unwrap();
    println!("time to fill resize image => {:?}", start_4.elapsed() );

    // write in disk
    let start_5 = time::Instant::now();
    write_on_disc(new_img);
   // buffer = get_mat_as_buffer(new_img).unwrap();
    println!("time to write image => {:?}", start_5.elapsed()  );
   // println!("time to write image => {:?} {:?}", start_5.elapsed() , buffer.len() );
    println!("time total => {:?}", start_1.elapsed() );

}
