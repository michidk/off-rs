use std::path::Path;

use off_rs::{
    document::{OffDocument, ParserOptions},
    geometry::ColorFormat,
};

const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/cube.off");

fn main() {
    let mesh = OffDocument::from_path(
        Path::new(PATH),
        ParserOptions {
            color_format: ColorFormat::RGBAFloat,
            ..Default::default()
        },
    );

    println!("{:#?}", mesh);
}
