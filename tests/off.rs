use off_rs::{
    document::{OffDocument, ParserOptions},
    geometry::{ColorFormat, Face, Position, Vertex},
};

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
    let off = content.parse::<OffDocument>().unwrap();
    println!("{:#?}", off);
}

#[test]
fn wiki_example() {
    let content = r#"
OFF
# cube.off
# A cube

8 6 12
 1.0  0.0 1.4142
 0.0  1.0 1.4142
-1.0  0.0 1.4142
 0.0 -1.0 1.4142
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
        color_format: ColorFormat::RGBAInteger,
        ..Default::default()
    };
    let off = OffDocument::parse(content, options);
    println!("{:#?}", off);
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

    let off = content.parse::<OffDocument>();
    assert_eq!(
        off.unwrap(),
        OffDocument {
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
