use std::path::Path;

use off_rs::{
    geometry::mesh::Mesh,
    parser::{color_format::ColorFormat, options::Options},
    FromPath,
};

const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/cube.off");

fn main() {
    let mesh = Mesh::from_path(
        Path::new(PATH),
        Options {
            color_format: ColorFormat::RGBAFloat,
            ..Default::default()
        },
    );

    println!("{:#?}", mesh);
}
