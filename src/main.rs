pub mod constraints;
mod utils;
pub mod images;
pub mod tile_set;
#[allow(unused)]
use std::{thread::sleep, time::{Duration, SystemTime}};
#[allow(unused)]
use raylib::{color::Color, prelude::RaylibDraw, RaylibHandle, RaylibThread};
#[allow(unused)]
fn collapse_sim(){
    constraints::test_collapse();
}

#[allow(unused)]
fn blur_sim(){
    let img = images::ByteImage::new_from_file("image.png").expect("i know you exist");
    let blurred = img.blur(15,5000.0);
    blurred.export("blurred.png");
}

#[allow(unused)]
fn blur_fast(){
    let (mut handle,mut thread) = raylib::prelude::init().size(1000, 1000).title("hello window").log_level(raylib::prelude::TraceLogLevel::LOG_ALL).build();
    let img = images::ByteImage::new_from_file("image.png").expect("i know you exist");
    let blurred = img.blur_shader(&thread, &mut handle, 100, 50.0).expect("msg");
    blurred.export("blurred.png");
}




#[allow(unused)]
fn cell_fast(){
    let (mut handle,mut thread) = raylib::prelude::init().size(1000, 1000).title("hello window").log_level(raylib::prelude::TraceLogLevel::LOG_DEBUG).build();
    let img = images::ByteImage::new_from_file("image.png").expect("i know you exist");
    let blurred = img.cell_shader(&thread, &mut handle, 6, 6.0).expect("msg");
    blurred.export("cell.png");
}

#[allow(unused)]
fn diff_fast(){
    let (mut handle,mut thread) = raylib::prelude::init().size(1000, 1000).title("hello window").log_level(raylib::prelude::TraceLogLevel::LOG_ERROR).build();
    let img = images::ByteImage::new_from_file("image.png").expect("i know you exist");
    let blurred = img.guass_diff_shader(&thread, &mut handle, 6, 6.0, 10, 10.0,false).expect("msg");
    blurred.export("blurred.png");
}

#[allow(unused)]
fn diff(){
    let img = images::ByteImage::new_from_file("image.png").expect("i know you exist");
    let img2 = img.shift_horizontal(1000);
    println!("diff(img,img) ={}", images::byte_image_comparision(&img, &img2));
}

fn main() {
    let now = SystemTime::now();
    diff();
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("took {} ms", elapsed.as_millis());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
}
