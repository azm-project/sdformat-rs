use sdformat_rs::*;

pub fn main() {
    let sdf = read_file("examples/simple_arm/model.sdf").unwrap();
    println!("{:#?}", sdf);
}
