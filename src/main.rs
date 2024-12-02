pub mod constraints;
mod utils;
use std::time::SystemTime;
fn main() {
    let now = SystemTime::now();
    constraints::test_collapse();
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
