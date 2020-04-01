extern crate tensorflow;

fn main() {
    let tfVersionRes = tensorflow::version();
    match tfVersionRes {
        Ok(tfVersion) => println!("Hello, tf! {}", tfVersion),
        Err(errMsg) => println!("Error while getting tensorflow version from Rust! {}", errMsg)
    }
    
}
