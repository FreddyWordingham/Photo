use nalgebra::{Point3, Unit, Vector2, Vector3};
use std::{
    cmp::Ordering,
    {fs::read_to_string, path::Path},
};

use crate::{
    assets::MeshBvh,
    geometry::{Ray, Triangle},
};

struct Face {
    position_indices: [usize; 3],
    normal_indices: [usize; 3],
    texture_coordinate_indices: [usize; 3],
}

pub struct Mesh {
    vertex_positions: Vec<Point3<f64>>,
    vertex_normals: Vec<Unit<Vector3<f64>>>,
    vertex_texture_coordinates: Vec<Vector2<f64>>,
    faces: Vec<Face>,
    bvh: MeshBvh,
}

impl Mesh {
    /// Load a mesh from a file.
    pub fn load(path: &Path) -> Self {
        let file_string = read_to_string(path).unwrap();
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
        let mut vertex_texture_coordinates = vec![];
        let mut faces = vec![];

        let mut mins = Point3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut maxs = Point3::new(f64::MIN, f64::MIN, f64::MIN);

        for tokens in line_tokens {
            if tokens.is_empty() {
                continue;
            }

            match tokens[0] {
                "v" => {
                    let x = tokens[1].parse::<f64>().unwrap();
                    let y = tokens[2].parse::<f64>().unwrap();
                    let z = tokens[3].parse::<f64>().unwrap();
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
                    let xn = tokens[1].parse::<f64>().unwrap();
                    let yn = tokens[2].parse::<f64>().unwrap();
                    let zn = tokens[3].parse::<f64>().unwrap();
                    vertex_normals.push(Unit::new_normalize(Vector3::new(xn, yn, zn)));
                }
                "vt" => {
                    let u = tokens[1].parse::<f64>().unwrap();
                    let v = tokens[2].parse::<f64>().unwrap();
                    vertex_texture_coordinates.push(Vector2::new(u, v));
                }
                "f" => {
                    let mut f = [[0; 3]; 3];
                    for (i, token) in tokens.iter().skip(1).enumerate() {
                        let indices: Vec<_> = token
                            .split('/')
                            .map(|index| index.parse::<usize>().unwrap() - 1)
                            .collect();
                        f[i] = [indices[0], indices[1], indices[2]];
                    }
                    faces.push(Face {
                        position_indices: [f[0][0], f[1][0], f[2][0]],
                        normal_indices: [f[0][2], f[1][2], f[2][2]],
                        texture_coordinate_indices: [f[0][1], f[1][1], f[2][1]],
                    });
                }
                _ => {}
            }
        }

        let triangles = faces
            .iter()
            .map(|f| {
                Triangle::new(
                    [
                        vertex_positions[f.position_indices[0]],
                        vertex_positions[f.position_indices[1]],
                        vertex_positions[f.position_indices[2]],
                    ],
                    [
                        vertex_normals[f.normal_indices[0]],
                        vertex_normals[f.normal_indices[1]],
                        vertex_normals[f.normal_indices[2]],
                    ],
                    [
                        vertex_texture_coordinates[f.texture_coordinate_indices[0]],
                        vertex_texture_coordinates[f.texture_coordinate_indices[1]],
                        vertex_texture_coordinates[f.texture_coordinate_indices[2]],
                    ],
                )
            })
            .collect::<Vec<_>>();

        Self {
            vertex_positions,
            vertex_normals,
            vertex_texture_coordinates,
            faces,
            bvh: MeshBvh::new(&triangles),
        }
    }

    /// Get a single triangle.
    pub fn triangle(&self, index: usize) -> Triangle {
        let face = &self.faces[index];
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
            [
                self.vertex_texture_coordinates[face.texture_coordinate_indices[0]],
                self.vertex_texture_coordinates[face.texture_coordinate_indices[1]],
                self.vertex_texture_coordinates[face.texture_coordinate_indices[2]],
            ],
        )
    }

    /// Iterate over the triangles of the mesh.
    pub fn triangles(&self) -> impl Iterator<Item = Triangle> + '_ {
        self.faces.iter().map(|f| {
            Triangle::new(
                [
                    self.vertex_positions[f.position_indices[0]],
                    self.vertex_positions[f.position_indices[1]],
                    self.vertex_positions[f.position_indices[2]],
                ],
                [
                    self.vertex_normals[f.normal_indices[0]],
                    self.vertex_normals[f.normal_indices[1]],
                    self.vertex_normals[f.normal_indices[2]],
                ],
                [
                    self.vertex_texture_coordinates[f.texture_coordinate_indices[0]],
                    self.vertex_texture_coordinates[f.texture_coordinate_indices[1]],
                    self.vertex_texture_coordinates[f.texture_coordinate_indices[2]],
                ],
            )
        })
    }

    pub fn ray_intersect(&self, ray: &Ray) -> bool {
        self.bvh
            .ray_intersections(ray, self)
            .into_iter()
            .any(|(n, _dist)| self.triangle(n).ray_intersect(ray))
    }

    pub fn ray_intersect_distance(&self, ray: &Ray) -> Option<f64> {
        self.bvh
            .ray_intersections(ray, self)
            .into_iter()
            .filter_map(|(n, _)| self.triangle(n).ray_intersect_distance(ray))
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
    }

    pub fn ray_intersect_distance_normals(
        &self,
        ray: &Ray,
    ) -> Option<(f64, Unit<Vector3<f64>>, Unit<Vector3<f64>>)> {
        self.bvh
            .ray_intersections(ray, self)
            .into_iter()
            .filter_map(|(n, _)| {
                self.triangle(n)
                    .ray_intersect_distance_normals(ray)
                    .map(|result| (n, result))
            })
            .min_by(|(_, (a_distance, _, _)), (_, (b_distance, _, _))| {
                a_distance
                    .partial_cmp(b_distance)
                    .unwrap_or(Ordering::Equal)
            })
            .map(|(_, result)| result)
    }
}