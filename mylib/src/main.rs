mod commons;

use std::time;
use opencv::core;

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
        let mat_src = commons::image::load_buffer(&buffer).unwrap();
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

fn get_size_rez(img_resize: commons::ImageResize) -> core::Size {
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
    let mut buffer = commons::image::load_image_from_disk("/Users/renatomoitinho/Documents/repositories/rust-lang/imgs/png/1mb.png");

    let start_1 = time::Instant::now();

    // load image from buffer
    let start_2 = time::Instant::now();
    let img_ref = ImageRef::new(buffer);
    println!("time to load image from buffer => {:?}", start_2.elapsed() );

    // target new size
    let img_resize = commons::get_target_size(get_size_ref(&img_ref), 1000, 1000);

    // resize
    let start_3 = time::Instant::now();
    let mut new_img = commons::image::resize(img_ref.mat, get_size_rez(img_resize)).unwrap();
    println!("time to resize image => {:?}", start_3.elapsed() );

    // fill
    let start_4 = time::Instant::now();
    new_img = commons::image::expand(new_img, img_resize).unwrap();
    println!("time to fill resize image => {:?}", start_4.elapsed() );

    // write in disk
    let start_5 = time::Instant::now();
    commons::image::write_on_disc(new_img, "~/Documents/repositories/rust-lang/imgs/UUU.jpeg", 80, "JPG");
   // buffer = get_mat_as_buffer(new_img).unwrap();
    println!("time to write image => {:?}", start_5.elapsed()  );
   // println!("time to write image => {:?} {:?}", start_5.elapsed() , buffer.len() );
    println!("time total => {:?}", start_1.elapsed() );

}
