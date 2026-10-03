# off-rs - A simple .off file parser

[![CI](https://img.shields.io/github/actions/workflow/status/michidk/off-rs/ci.yaml?branch=main)](https://github.com/michidk/off-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/off-rs)](https://crates.io/crates/off-rs)
[![docs.rs](https://img.shields.io/docsrs/off-rs)](https://docs.rs/off-rs)
[![License](https://img.shields.io/crates/l/off-rs)](#license)

Parses `.off` ([Object File Format](<https://en.wikipedia.org/wiki/OFF_(file_format)>)) files.
This implementation follows [this spec](https://people.sc.fsu.edu/~jburkardt/data/off/off.html) from the Princeton Shape Benchmark.

Sample `.off` file:

```off
# this file header has to be the first instruction
OFF
# cube.off
# A cube

# 8 vertices, 6 faces, 12 edges
8 6 12

# vetex coordinates: x, y, z
  1.632993   0.000000   1.154701
  0.000000   1.632993   1.154701
 -1.632993   0.000000   1.154701
  0.000000  -1.632993   1.154701
  1.632993   0.000000  -1.154701
  0.000000   1.632993  -1.154701
 -1.632993   0.000000  -1.154701
  0.000000  -1.632993  -1.154701

# face indicies & RGBA color data: n, v1, v2, v3, v4, r, g, b, a
  4  0 1 2 3  1.000 0.000 0.000 0.75
  4  7 4 0 3  0.300 0.400 0.000 0.75
  4  4 5 1 0  0.200 0.500 0.100 0.75
  4  5 6 2 1  0.100 0.600 0.200 0.75
  4  3 2 6 7  0.000 0.700 0.300 0.75
  4  6 5 4 7  0.000 1.000 0.000 0.75
```

This [cube.off](examples/cube.off) file is parsed using `off-rs` [in this example](examples/cube.rs).

## Usage

```rust
let off_string = r#"
OFF
3 1
1.0 0.0 0.0
0.0 1.0 0.0
0.0 0.0 1.0
3  0 1 2  255 0 0 # red
"#;

let mesh = off_rs::parse(
    off_string,
    Default::default() // optional ParserOptions
);

println!("{:#?}", mesh);
```

Will return a structure like this:

```jsonc
Mesh {
    vertices: [
        Vertex {
            position: Position {
                x: 1.0,
                y: 0.0,
                z: 0.0,
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
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 1.0,
                },
            ),
        },
        ...
```

## License

Licensed under either of [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or [MIT license](http://opensource.org/licenses/MIT) at your option.
