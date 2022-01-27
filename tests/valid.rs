use off_rs::geometry::{
    color::Color,
    color_format::ColorFormat,
    mesh::{Face, Vertex},
    position::Position,
};
#[allow(unused_imports)]
use off_rs::{geometry::*, mesh::*};

#[test]
fn spec_example() {
    let content = r#"
OFF
#
#  cube.off
#  A cube.
#  There is extra RGBA color information specified for the faces.
#
8 6 12
  1.632993   0.000000   1.154701
  0.000000   1.632993   1.154701
 -1.632993   0.000000   1.154701
  0.000000  -1.632993   1.154701
  1.632993   0.000000  -1.154701
  0.000000   1.632993  -1.154701
 -1.632993   0.000000  -1.154701
  0.000000  -1.632993  -1.154701
  4  0 1 2 3  1.000 0.000 0.000 0.75
  4  7 4 0 3  0.300 0.400 0.000 0.75
  4  4 5 1 0  0.200 0.500 0.100 0.75
  4  5 6 2 1  0.100 0.600 0.200 0.75
  4  3 2 6 7  0.000 0.700 0.300 0.75
  4  6 5 4 7  0.000 1.000 0.000 0.75
"#;

    let options = ParserOptions {
        color_format: ColorFormat::RGBAFloat,
        ..Default::default()
    };
    let off = Mesh::parse(content, options).unwrap();

    assert_eq!(
        off,
        Mesh {
            vertices: vec![
                Vertex {
                    position: Position {
                        x: 1.632993,
                        y: 0.0,
                        z: 1.154701,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: 0.0,
                        y: 1.632993,
                        z: 1.154701,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: -1.632993,
                        y: 0.0,
                        z: 1.154701,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: 0.0,
                        y: -1.632993,
                        z: 1.154701,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: 1.632993,
                        y: 0.0,
                        z: -1.154701,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: 0.0,
                        y: 1.632993,
                        z: -1.154701,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: -1.632993,
                        y: 0.0,
                        z: -1.154701,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: 0.0,
                        y: -1.632993,
                        z: -1.154701,
                    },
                    color: None,
                },
            ],
            faces: vec![
                Face {
                    vertices: vec![0, 1, 2, 3,],
                    color: Some(Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.75,
                    },),
                },
                Face {
                    vertices: vec![7, 4, 0, 3,],
                    color: Some(Color {
                        r: 0.3,
                        g: 0.4,
                        b: 0.0,
                        a: 0.75,
                    },),
                },
                Face {
                    vertices: vec![4, 5, 1, 0,],
                    color: Some(Color {
                        r: 0.2,
                        g: 0.5,
                        b: 0.1,
                        a: 0.75,
                    },),
                },
                Face {
                    vertices: vec![5, 6, 2, 1,],
                    color: Some(Color {
                        r: 0.1,
                        g: 0.6,
                        b: 0.2,
                        a: 0.75,
                    },),
                },
                Face {
                    vertices: vec![3, 2, 6, 7,],
                    color: Some(Color {
                        r: 0.0,
                        g: 0.7,
                        b: 0.3,
                        a: 0.75,
                    },),
                },
                Face {
                    vertices: vec![6, 5, 4, 7,],
                    color: Some(Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                        a: 0.75,
                    },),
                },
            ],
        }
    );
}

#[test]
fn wiki_example() {
    let content = r#"
OFF
# cube.off
# A cube

8 6 12
 1.0  0.0 1.5142
 0.0  1.0 1.5142
-1.0  0.0 1.5142
 0.0 -1.0 1.5142
 1.0  0.0 0.0
 0.0  1.0 0.0
-1.0  0.0 0.0
 0.0 -1.0 0.0
4  0 1 2 3  255 0 0 #red
4  7 4 0 3  0 255 0 #green
4  4 5 1 0  0 0 255 #blue
4  5 6 2 1  0 255 0
4  3 2 6 7  0 0 255
4  6 5 4 7  255 0 0
"#;

    let options = ParserOptions {
        color_format: ColorFormat::RGBInteger,
        ..Default::default()
    };
    let off = Mesh::parse(content, options).unwrap();

    assert_eq!(
        off,
        Mesh {
            vertices: vec![
                Vertex {
                    position: Position {
                        x: 1.0,
                        y: 0.0,
                        z: 1.5142,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: 0.0,
                        y: 1.0,
                        z: 1.5142,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: -1.0,
                        y: 0.0,
                        z: 1.5142,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: 0.0,
                        y: -1.0,
                        z: 1.5142,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: -1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: 0.0,
                        y: -1.0,
                        z: 0.0,
                    },
                    color: None,
                },
            ],
            faces: vec![
                Face {
                    vertices: vec![0, 1, 2, 3,],
                    color: Some(Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },),
                },
                Face {
                    vertices: vec![7, 4, 0, 3,],
                    color: Some(Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                        a: 1.0,
                    },),
                },
                Face {
                    vertices: vec![4, 5, 1, 0,],
                    color: Some(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 1.0,
                        a: 1.0,
                    },),
                },
                Face {
                    vertices: vec![5, 6, 2, 1,],
                    color: Some(Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                        a: 1.0,
                    },),
                },
                Face {
                    vertices: vec![3, 2, 6, 7,],
                    color: Some(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 1.0,
                        a: 1.0,
                    },),
                },
                Face {
                    vertices: vec![6, 5, 4, 7,],
                    color: Some(Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },),
                },
            ],
        }
    );
}

#[test]
fn cube() {
    let content = r#"
OFF
8 6 0
-0.500000 -0.500000 0.500000
0.500000 -0.500000 0.500000
-0.500000 0.500000 0.500000
0.500000 0.500000 0.500000
-0.500000 0.500000 -0.500000
0.500000 0.500000 -0.500000
-0.500000 -0.500000 -0.500000
0.500000 -0.500000 -0.500000
4 0 1 3 2
4 2 3 5 4
4 4 5 7 6
4 6 7 1 0
4 1 7 5 3
4 6 0 2 4
"#;

    let off = content.parse::<Mesh>().unwrap();
    assert_eq!(
        off,
        Mesh {
            vertices: vec![
                Vertex {
                    position: Position {
                        x: -0.5,
                        y: -0.5,
                        z: 0.5,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: 0.5,
                        y: -0.5,
                        z: 0.5,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: -0.5,
                        y: 0.5,
                        z: 0.5,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: 0.5,
                        y: 0.5,
                        z: 0.5,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: -0.5,
                        y: 0.5,
                        z: -0.5,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: 0.5,
                        y: 0.5,
                        z: -0.5,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: -0.5,
                        y: -0.5,
                        z: -0.5,
                    },
                    color: None,
                },
                Vertex {
                    position: Position {
                        x: 0.5,
                        y: -0.5,
                        z: -0.5,
                    },
                    color: None,
                },
            ],
            faces: vec![
                Face {
                    vertices: vec![0, 1, 3, 2],
                    color: None,
                },
                Face {
                    vertices: vec![2, 3, 5, 4],
                    color: None,
                },
                Face {
                    vertices: vec![4, 5, 7, 6],
                    color: None,
                },
                Face {
                    vertices: vec![6, 7, 1, 0],
                    color: None,
                },
                Face {
                    vertices: vec![1, 7, 5, 3],
                    color: None,
                },
                Face {
                    vertices: vec![6, 0, 2, 4],
                    color: None,
                },
            ],
        }
    );
}

#[test]
fn vertex_colors() {
    let content = r#"
OFF
3 1 0
-0.500000 -0.500000 0.500000 12 122 210
0.500000 -0.500000 0.500000 34 23 112
-0.500000 0.500000 0.500000 123 12 44
3 0 1 2
"#;

    let off = Mesh::parse(
        content,
        ParserOptions {
            color_format: ColorFormat::RGBInteger,
            ..Default::default()
        },
    )
    .unwrap();

    println!("{:#?}", off);
    assert_eq!(
        off,
        Mesh {
            vertices: vec![
                Vertex {
                    position: Position {
                        x: -0.5,
                        y: -0.5,
                        z: 0.5,
                    },
                    color: Some(Color {
                        r: 0.047058824,
                        g: 0.47843137,
                        b: 0.8235294,
                        a: 1.0,
                    },),
                },
                Vertex {
                    position: Position {
                        x: 0.5,
                        y: -0.5,
                        z: 0.5,
                    },
                    color: Some(Color {
                        r: 0.13333334,
                        g: 0.09019608,
                        b: 0.4392157,
                        a: 1.0,
                    },),
                },
                Vertex {
                    position: Position {
                        x: -0.5,
                        y: 0.5,
                        z: 0.5,
                    },
                    color: Some(Color {
                        r: 0.48235294,
                        g: 0.047058824,
                        b: 0.17254902,
                        a: 1.0,
                    },),
                },
            ],
            faces: vec![Face {
                vertices: vec![0, 1, 2,],
                color: None,
            },],
        }
    )
}
