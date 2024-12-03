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
fn main() {
    let now = SystemTime::now();
    blur_sim();
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
