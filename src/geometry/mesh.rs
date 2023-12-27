//! Triangle mesh structure.

use std::{error::Error, fs::read_to_string, path::Path};

use nalgebra::{Point3, Unit, Vector3};

use crate::{error::ParseError, geometry::Triangle};

/// Triangular face.
struct Face {
    /// Vertex position indices.
    position_indices: [usize; 3],
    /// Vertex normal indices.
    normal_indices: [usize; 3],
}

/// Triangle mesh.
pub struct Mesh {
    /// Vertex positions.
    vertex_positions: Vec<Point3<f64>>,
    /// Vertex normals.
    vertex_normals: Vec<Unit<Vector3<f64>>>,
    /// List of faces.
    faces: Vec<Face>,
    // /// Bounding Volume Hierarchy.
    // bvh: Bvh,
}

impl Mesh {
    /// Load a [`Mesh`] from a wavefront (.obj) file.
    ///
    /// # Errors
    ///
    /// Returns a [`ParseError`] if the file cannot be read,
    /// or if the file is not a valid wavefront (.obj) file,
    /// or if the values in the file can not be parsed.
    #[inline]
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file_string = read_to_string(path)?;

        let mut vertex_positions = Vec::new();
        let mut vertex_normals = Vec::new();
        let mut faces = Vec::new();

        let mut mins = Point3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut maxs = Point3::new(f64::MIN, f64::MIN, f64::MIN);

        for line in file_string.lines() {
            let tokens: Vec<&str> = line.split_whitespace().collect();

            if tokens.is_empty() {
                continue;
            }

            match *tokens
                .first()
                .ok_or_else(|| ParseError::new("Mesh file must specify identifying token!"))?
            {
                "v" => {
                    let [vertex, min, max] = Self::parse_vertex_position(&tokens[1..])?;
                    vertex_positions.push(vertex);
                    mins = mins.inf(&min);
                    maxs = maxs.sup(&max);
                }
                "vn" => {
                    let normal = Self::parse_vertex_normal(&tokens[1..])?;
                    vertex_normals.push(normal);
                }
                "f" => {
                    let face = Self::parse_face(&tokens[1..])?;
                    faces.push(face);
                }
                _ => {}
            }
        }

        Ok(Self {
            vertex_positions,
            vertex_normals,
            faces,
        })
    }

    /// Parse a vertex position from an .obj file string.
    #[inline]
    #[allow(clippy::missing_asserts_for_indexing, clippy::panic_in_result_fn)]
    fn parse_vertex_position(coords: &[&str]) -> Result<[Point3<f64>; 3], Box<dyn Error>> {
        assert!(
            coords.len() == 3,
            "Vertex position must have exactly 3 coordinates!"
        );

        let x = coords[0].parse::<f64>()?;
        let y = coords[1].parse::<f64>()?;
        let z = coords[2].parse::<f64>()?;
        let vertex = Point3::new(x, y, z);
        let min = Point3::new(x, y, z);
        let max = Point3::new(x, y, z);

        Ok([vertex, min, max])
    }

    /// Parse a vertex normal from an .obj file string.
    #[inline]
    #[allow(clippy::missing_asserts_for_indexing, clippy::panic_in_result_fn)]
    fn parse_vertex_normal(coords: &[&str]) -> Result<Unit<Vector3<f64>>, Box<dyn Error>> {
        assert!(
            coords.len() == 3,
            "Vertex normal must have exactly 3 coordinates!"
        );

        let xn = coords[0].parse::<f64>()?;
        let yn = coords[1].parse::<f64>()?;
        let zn = coords[2].parse::<f64>()?;
        let normal = Unit::new_normalize(Vector3::new(xn, yn, zn));

        Ok(normal)
    }

    /// Parse a face from an .obj file string.
    #[inline]
    fn parse_face(tokens: &[&str]) -> Result<Face, Box<dyn Error>> {
        let mut position_indices = [0; 3];
        let mut normal_indices = [0; 3];

        for (i, token) in tokens.iter().enumerate() {
            let indices: Vec<usize> = token
                .split('/')
                .map(|index_string| index_string.parse::<usize>().map(|index| index - 1))
                .collect::<Result<Vec<_>, _>>()?;
            position_indices[i] = indices[0];
            normal_indices[i] = indices[2];
        }

        Ok(Face {
            position_indices,
            normal_indices,
        })
    }

    /// Iterate over the [`Triangle`]s of the [`Mesh`].
    #[inline]
    pub fn triangles(&self) -> impl Iterator<Item = Triangle> + '_ {
        self.faces.iter().map(|face| {
            Triangle::new(
                [
                    self.vertex_positions[face.position_indices[0]],
                    self.vertex_positions[face.position_indices[1]],
                    self.vertex_positions[face.position_indices[2]],
                ],
                [
                    self.vertex_normals[face.normal_indices[0]],
                    self.vertex_normals[face.normal_indices[1]],
                    self.vertex_normals[face.normal_indices[2]],
                ],
            )
        })
    }
}
