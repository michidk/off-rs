use std::path::Path;

use off_rs::{
    geometry::color_format::ColorFormat,
    mesh::{Mesh, ParserOptions},
};

const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/cube.off");

fn main() {
    let mesh = Mesh::from_path(
        Path::new(PATH),
        ParserOptions {
            color_format: ColorFormat::RGBAFloat,
            ..Default::default()
        },
    );

    println!("{:#?}", mesh);
}
