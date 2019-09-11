mod commons;

use std::time;
use std::env;
use opencv::core;

fn get_size_rez(img_resize: commons::ImageResize) -> core::Size {
    core::Size {
        width: img_resize.width,
        height: img_resize.height
    }
}

fn get_size_ref(img_ref: &commons::ImageRef) -> core::Size {
    core::Size {
        width: img_ref.width,
        height: img_ref.height
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut path: Option<&str> = Some("/Users/renatomoitinho/Documents/repositories/rust-lang/imgs/1mb.jpg");

    match args.len() as i32 {
        2..=5 => {
            path = Some(&args[1]);
        }
        _ => {}
    }

    println!("{:?}", path.unwrap());

    // read image from disk
    let mut buffer = commons::image::load_image_from_disk(path.unwrap());

    let start_1 = time::Instant::now();

    // load image from buffer
    let start_2 = time::Instant::now();
    let img_ref = commons::ImageRef::new(buffer, None);
    println!("time to load image from buffer => {:?}", start_2.elapsed());

    // target new size
    let img_resize = commons::get_target_size(get_size_ref(&img_ref), 1024, 1024);

    // resize
    let start_3 = time::Instant::now();
    let mut new_img = commons::image::resize(img_ref.mat, get_size_rez(img_resize)).unwrap();
    println!("time to resize image => {:?}", start_3.elapsed());

    // fill
    let start_4 = time::Instant::now();
    new_img = commons::image::expand(new_img, img_resize).unwrap();
    println!("time to fill resize image => {:?}", start_4.elapsed());

    // write in disk
    let start_5 = time::Instant::now();

    commons::image::write_on_disc(new_img, "/tmp/media_test.jpeg", 100, "JPG");
    // buffer = get_mat_as_buffer(new_img).unwrap();
    println!("time to write image => {:?}", start_5.elapsed());
    // println!("time to write image => {:?} {:?}", start_5.elapsed() , buffer.len() );
    println!("time total => {:?}", start_1.elapsed());
}
