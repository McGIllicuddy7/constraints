pub mod constraints;
mod utils;
pub mod images;
use std::time::SystemTime;
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
    let (mut handle,mut thread) = raylib::prelude::init().size(1000, 1000).title("hello window").log_level(raylib::prelude::TraceLogLevel::LOG_DEBUG).build();
    let img = images::ByteImage::new_from_file("image.png").expect("i know you exist");
    let blurred = img.blur_shader(&thread, &mut handle, 100, 500.0).expect("msg");
    blurred.export("blurred.png");
}

#[allow(unused)]
fn diff_fast(){
    let (mut handle,mut thread) = raylib::prelude::init().size(1000, 1000).title("hello window").log_level(raylib::prelude::TraceLogLevel::LOG_DEBUG).build();
    let img = images::ByteImage::new_from_file("image.png").expect("i know you exist");
    let blurred = img.guass_diff_shader_explicit(&thread, &mut handle, 10, 10.0, 20, 20.0,true).expect("msg");
    blurred.export("blurred.png");
}
fn main() {
    let now = SystemTime::now();
    diff_fast();
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
