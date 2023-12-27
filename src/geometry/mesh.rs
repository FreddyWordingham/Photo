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
        let line_tokens: Vec<Vec<_>> = file_string
            .lines()
            .map(|line| line.split_whitespace().collect())
            .collect();

        let mut vertex_positions = Vec::with_capacity(
            line_tokens
                .iter()
                .filter(|tokens| !tokens.is_empty() && tokens[0] == "v")
                .count(),
        );
        let mut vertex_normals = vec![];
        let mut faces = vec![];

        let mut mins = Point3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut maxs = Point3::new(f64::MIN, f64::MIN, f64::MIN);

        for tokens in line_tokens {
            if tokens.is_empty() {
                continue;
            }

            match *tokens
                .first()
                .ok_or_else(|| ParseError::new("Mesh file must specify identifying token!"))?
            {
                "v" => {
                    let x = tokens[1].parse::<f64>()?;
                    let y = tokens[2].parse::<f64>()?;
                    let z = tokens[3].parse::<f64>()?;
                    vertex_positions.push(Point3::new(x, y, z));

                    if x < mins.x {
                        mins.x = x;
                    }
                    if y < mins.y {
                        mins.y = y;
                    }
                    if z < mins.z {
                        mins.z = z;
                    }

                    if x > maxs.x {
                        maxs.x = x;
                    }
                    if y > maxs.y {
                        maxs.y = y;
                    }
                    if z > maxs.z {
                        maxs.z = z;
                    }
                }
                "vn" => {
                    let xn = tokens[1].parse::<f64>()?;
                    let yn = tokens[2].parse::<f64>()?;
                    let zn = tokens[3].parse::<f64>()?;
                    vertex_normals.push(Unit::new_normalize(Vector3::new(xn, yn, zn)));
                }
                "f" => {
                    let mut face = [[0; 3]; 3];
                    for (i, token) in tokens.iter().skip(1).enumerate() {
                        let indices: Vec<usize> = token
                            .split('/')
                            .map(|index_string| {
                                index_string.parse::<usize>().map(|index| index - 1)
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        face[i] = [indices[0], indices[1], indices[2]];
                    }
                    faces.push(Face {
                        position_indices: [face[0][0], face[1][0], face[2][0]],
                        normal_indices: [face[0][2], face[1][2], face[2][2]],
                    });
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
