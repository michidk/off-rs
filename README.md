# off-rs - A simple .off file parser

[![Apache 2.0 License](https://img.shields.io/badge/License-Apache%202.0-blue?style=for-the-badge)](http://www.apache.org/licenses/LICENSE-2.0)
[![MIT License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](http://opensource.org/licenses/MIT)
[![Continuous integration](https://img.shields.io/github/workflow/status/michidk/off-rs/Continuous%20Integration?style=for-the-badge)](https://github.com/michidk/off-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/off-rs?style=for-the-badge)](https://crates.io/crates/off-rs)
[![docs.rs](https://img.shields.io/docsrs/off-rs?style=for-the-badge)](https://docs.rs/off-rs)

Parses `.off` ([Object File Format](<https://en.wikipedia.org/wiki/OFF_(file_format)>)) files.
This implementation follows [this spec](https://people.sc.fsu.edu/~jburkardt/data/off/off.html) from the Princeton Shape Benchmark.

Sample `.off` file:

```
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
let mesh = Mesh::parse(
    content,
    ..Default::default() // optional ParserOptions
);

println!("{:#?}", mesh);
```

Will return a structure like this:

```
Mesh {
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
