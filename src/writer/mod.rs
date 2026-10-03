//! Writes a [`Mesh`] to the `off` format.

pub mod error;
pub mod options;

use std::io::Write;

use crate::geometry::mesh::Mesh;

use self::{
    error::{Element, Error},
    options::Options,
};

pub type Result<T = ()> = std::result::Result<T, Error>;

/// Writes a [`Mesh`] in the `off` format to the given [`Write`]r.
///
/// The edge count in the header is written as `0`, as the `off` format does not require it.
///
/// # Errors
///
/// Will return [`Error`] if the mesh can not be represented in the `off` format
/// or if an error occurs while writing.
pub fn write<W: Write>(mesh: &Mesh, mut writer: W, options: Options) -> Result {
    validate(mesh)?;

    writeln!(writer, "OFF")?;
    writeln!(writer, "{} {} 0", mesh.vertex_count(), mesh.face_count())?;

    for (index, vertex) in mesh.vertices.iter().enumerate() {
        let position = vertex.position;
        write!(writer, "{} {} {}", position.x, position.y, position.z)?;

        if let Some(color) = vertex.color {
            let color =
                options
                    .color_format
                    .format(color)
                    .map_err(|source| Error::InvalidColor {
                        element: Element::Vertex(index),
                        source,
                    })?;
            write!(writer, " {color}")?;
        }

        writeln!(writer)?;
    }

    for (index, face) in mesh.faces.iter().enumerate() {
        write!(writer, "{}", face.vertices.len())?;

        for vertex in &face.vertices {
            write!(writer, " {vertex}")?;
        }

        if let Some(color) = face.color {
            let color =
                options
                    .color_format
                    .format(color)
                    .map_err(|source| Error::InvalidColor {
                        element: Element::Face(index),
                        source,
                    })?;
            write!(writer, " {color}")?;
        }

        writeln!(writer)?;
    }

    writer.flush()?;

    Ok(())
}

/// Checks that the faces of the mesh can be parsed again.
fn validate(mesh: &Mesh) -> Result {
    for (face_index, face) in mesh.faces.iter().enumerate() {
        if face.vertices.len() < 3 {
            return Err(Error::InvalidFace {
                face: face_index,
                vertex_count: face.vertices.len(),
            });
        }

        if let Some(&index) = face.vertices.iter().find(|&&i| i >= mesh.vertex_count()) {
            return Err(Error::InvalidFaceIndex {
                face: face_index,
                index,
                vertex_count: mesh.vertex_count(),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{
        color::Color,
        mesh::{Face, Vertex},
        position::Position,
    };
    use crate::parser::color_format::ColorFormat;

    fn triangle() -> Mesh {
        Mesh {
            vertices: vec![
                Vertex::new(Position::new(1.0, 0.0, 0.0), None),
                Vertex::new(Position::new(0.0, 1.0, 0.5), None),
                Vertex::new(Position::new(0.0, 0.0, -1.0), None),
            ],
            faces: vec![Face::new(vec![0, 1, 2], None)],
        }
    }

    fn to_string(mesh: &Mesh, options: Options) -> Result<String> {
        let mut buffer = Vec::new();
        write(mesh, &mut buffer, options)?;
        Ok(String::from_utf8(buffer).unwrap())
    }

    #[test]
    fn write_without_colors() {
        assert_eq!(
            to_string(&triangle(), Options::default()).unwrap(),
            "OFF\n3 1 0\n1 0 0\n0 1 0.5\n0 0 -1\n3 0 1 2\n"
        );
    }

    #[test]
    fn write_colors() {
        let mut mesh = triangle();
        mesh.vertices[1].color = Some(Color::new(1.0, 0.0, 0.0, 0.5).unwrap());
        mesh.faces[0].color = Some(Color::new(0.0, 1.0, 0.0, 1.0).unwrap());

        let options = Options {
            color_format: ColorFormat::RGBAInteger,
        };
        assert_eq!(
            to_string(&mesh, options).unwrap(),
            "OFF\n3 1 0\n1 0 0\n0 1 0.5 255 0 0 128\n0 0 -1\n3 0 1 2 0 255 0 255\n"
        );
    }

    #[test]
    fn write_empty_mesh() {
        assert_eq!(
            to_string(&Mesh::default(), Options::default()).unwrap(),
            "OFF\n0 0 0\n"
        );
    }

    #[test]
    fn invalid_face_index() {
        let mut mesh = triangle();
        mesh.faces.push(Face::new(vec![0, 1, 3], None));

        assert!(matches!(
            to_string(&mesh, Options::default()),
            Err(Error::InvalidFaceIndex {
                face: 1,
                index: 3,
                vertex_count: 3
            })
        ));
    }

    #[test]
    fn invalid_face() {
        let mut mesh = triangle();
        mesh.faces[0].vertices.pop();

        assert!(matches!(
            to_string(&mesh, Options::default()),
            Err(Error::InvalidFace {
                face: 0,
                vertex_count: 2
            })
        ));
    }

    #[test]
    fn invalid_color() {
        let mut mesh = triangle();
        mesh.vertices[2].color = Some(Color {
            red: 2.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        });

        assert!(matches!(
            to_string(&mesh, Options::default()),
            Err(Error::InvalidColor {
                element: Element::Vertex(2),
                ..
            })
        ));
    }
}
