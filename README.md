# off-rs - A simple .off file parser

[![MIT License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](https://choosealicense.com/licenses/mit/) [![Continuous integration](https://img.shields.io/github/workflow/status/michidk/off-rs/Continuous%20Integration?style=for-the-badge)](https://github.com/michidk/off-rs/actions) [![Crates.io](https://img.shields.io/crates/v/off-rs?style=for-the-badge)](https://crates.io/crates/spirv-layout)

Parses `.off` ([Object File Format](https://en.wikipedia.org/wiki/OFF_(file_format))) files.
This implementation follows [this spec](https://people.sc.fsu.edu/~jburkardt/data/off/off.html) from the Princeton Shape Benchmark.

Sample `.off` file:
![sample cube file](.github/images/off.svg)
This [cube.off](examples/cube.off) file is parsed [here](examples/cube.rs).

## Usage

```rust
let mesh = OffDocument::parse(
    content,
    ..Default::default() // optional ParserOptions
);

println!("{:#?}", mesh);
```

Will return a structure like this:
```json
OffDocument {
    vertices: [
        Vertex {
            position: Position {
                x: 1.632993,
                y: 0.0,
                z: 1.154701,
            },
            color: None,
        },
        ...
    faces: [
        Face {
            vertices: [
                0,
                1,
                2,
                3,
            ],
            color: Some(
                Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.75,
                },
            ),
        },
        ...
```
