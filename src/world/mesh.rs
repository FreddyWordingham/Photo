use nalgebra::{Point3, Unit, Vector2, Vector3};
use std::{fs::read_to_string, path::Path};

use crate::geometry::{Aabb, Ray, Triangle};

/// Triangular mesh.
pub struct Mesh {
    /// List of vertex positions.
    pub vertex_positions: Vec<Point3<f64>>,
    /// List of vertex normals.
    pub vertex_normals: Vec<Unit<Vector3<f64>>>,
    /// List of vertex textures.
    pub vertex_textures: Vec<Vector2<f64>>,
    /// List face indices.
    pub face_indices: Vec<[[usize; 3]; 3]>,
    /// Axis-aligned bounding box.
    pub aabb: Aabb,
}

impl Mesh {
    /// Load a mesh from a wavefront obj file.
    pub fn load(file_path: &Path) -> Self {
        let file_string = read_to_string(file_path).unwrap();
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
        let mut vertex_textures = vec![];
        let mut face_indices = vec![];

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
                    vertex_textures.push(Vector2::new(u, v));
                }
                "f" => {
                    let mut face_indices_inner = [[0; 3]; 3];
                    for (i, token) in tokens.iter().skip(1).enumerate() {
                        let indices: Vec<_> = token
                            .split('/')
                            .map(|index| index.parse::<usize>().unwrap() - 1)
                            .collect();
                        face_indices_inner[i] = [indices[0], indices[1], indices[2]];
                    }
                    face_indices.push(face_indices_inner);
                }
                _ => {}
            }
        }

        Self {
            vertex_positions,
            vertex_normals,
            vertex_textures,
            face_indices,
            aabb: Aabb::new(mins, maxs),
        }
    }

    /// Check if the mesh intersects an AABB.
    pub fn intersects_aabb(&self, aabb: &Aabb) -> bool {
        if !self.aabb.intersects_aabb(aabb) {
            return false;
        }

        for face in &self.face_indices {
            let vertex_positions = [
                self.vertex_positions[face[0][0]],
                self.vertex_positions[face[1][0]],
                self.vertex_positions[face[2][0]],
            ];
            let triangle = Triangle::new(vertex_positions);

            if triangle.intersects_aabb(aabb) {
                return true;
            }
        }

        false
    }

    /// Test for an intersection distance with a ray.
    pub fn intersect_ray(&self, ray: &Ray) -> Option<Point3<f64>> {
        // First check if the ray intersects the mesh's AABB
        if self.aabb.intersect_ray(ray).is_none() {
            return None;
        }

        let mut closest_intersection: Option<f64> = None;

        for face in &self.face_indices {
            let vertices = [
                self.vertex_positions[face[0][0]],
                self.vertex_positions[face[1][0]],
                self.vertex_positions[face[2][0]],
            ];
            let triangle = Triangle::new(vertices);

            if let Some(intersection_point) = triangle.intersect_ray(ray) {
                let intersection_distance = nalgebra::distance(&intersection_point, &ray.origin);

                if closest_intersection.is_none()
                    || intersection_distance < closest_intersection.unwrap()
                {
                    closest_intersection = Some(intersection_distance);
                }
            }
        }

        closest_intersection.map(|distance| ray.origin + distance * ray.direction.as_ref())
    }
}
