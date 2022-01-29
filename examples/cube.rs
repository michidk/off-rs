use std::path::Path;

use off_rs::parser::{color_format::ColorFormat, options::Options};

const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/cube.off");

fn main() {
    let mesh = off_rs::from_path(
        Path::new(PATH),
        Options {
            color_format: ColorFormat::RGBAFloat,
            ..Default::default()
        },
    );

    println!("{:#?}", mesh);
}
