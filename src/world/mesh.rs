use nalgebra::{Point3, Unit, Vector2, Vector3};
use std::{fs::read_to_string, path::Path};

use crate::geometry::{Aabb, Ray, Triangle};

/// Triangular mesh.
pub struct Mesh {
    /// List of vertex positions.
    vertex_positions: Vec<Point3<f64>>,
    /// List of vertex normals.
    vertex_normals: Vec<Unit<Vector3<f64>>>,
    /// List of vertex texture coordinates.
    vertex_texture_coordinates: Vec<Vector2<f64>>,
    /// List face indices.
    face_indices: Vec<[[usize; 3]; 3]>,
    /// Axis-aligned bounding box.
    aabb: Aabb,
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
        let mut vertex_texture_coordinates = vec![];
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
                    vertex_texture_coordinates.push(Vector2::new(u, v));
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
            vertex_texture_coordinates,
            face_indices,
            aabb: Aabb::new(mins, maxs),
        }
    }

    /// Get the list of vertex positions.
    pub fn vertex_positions(&self) -> &[Point3<f64>] {
        &self.vertex_positions
    }

    /// Get the list of vertex normals.
    pub fn vertex_normals(&self) -> &[Unit<Vector3<f64>>] {
        &self.vertex_normals
    }

    /// Get the list of vertex texture coordinates.
    pub fn vertex_texture_coordinates(&self) -> &[Vector2<f64>] {
        &self.vertex_texture_coordinates
    }

    /// Get the list of face indices.
    pub fn face_indices(&self) -> &[[[usize; 3]; 3]] {
        &self.face_indices
    }

    /// Iterate over the triangles of the mesh.
    pub fn triangles(&self) -> impl Iterator<Item = Triangle> + '_ {
        self.face_indices.iter().map(move |indices| Triangle {
            vertex_positions: [
                self.vertex_positions[indices[0][0]],
                self.vertex_positions[indices[1][0]],
                self.vertex_positions[indices[2][0]],
            ],
            vertex_normals: [
                self.vertex_normals[indices[0][2]],
                self.vertex_normals[indices[1][2]],
                self.vertex_normals[indices[2][2]],
            ],
            vertex_texture_coordinates: [
                self.vertex_texture_coordinates[indices[0][1]],
                self.vertex_texture_coordinates[indices[1][1]],
                self.vertex_texture_coordinates[indices[2][1]],
            ],
        })
    }

    /// Get the axis-aligned bounding box.
    pub fn aabb(&self) -> &Aabb {
        &self.aabb
    }

    /// Check if the mesh intersects an AABB.
    pub fn intersects_aabb(&self, aabb: &Aabb) -> bool {
        if !self.aabb.overlaps_aabb(aabb) {
            return false;
        }

        for triangle in self.triangles() {
            if triangle.overlaps_aabb(aabb) {
                return true;
            }
        }

        false
    }

    /// Test for an intersection with a ray.
    pub fn intersect_ray(&self, ray: &Ray) -> bool {
        if !self.aabb.intersect_ray(ray) {
            return false;
        }

        for triangle in self.triangles() {
            if triangle.intersect_ray(ray) {
                return true;
            }
        }

        false
    }

    /// Test for an intersection distance with a ray.
    pub fn intersect_ray_distance(&self, ray: &Ray) -> Option<f64> {
        if !self.aabb.intersect_ray(ray) {
            return None;
        }

        let mut closest_distance: Option<f64> = None;

        for triangle in self.triangles() {
            if let Some(distance) = triangle.intersect_ray_distance(ray) {
                if closest_distance.is_none() || distance < closest_distance.unwrap() {
                    closest_distance = Some(distance);
                }
            }
        }

        closest_distance
    }

    /// Test for an intersection point with a ray.
    pub fn intersect_ray_point(&self, ray: &Ray) -> Option<Point3<f64>> {
        if !self.aabb.intersect_ray(ray) {
            return None;
        }

        let mut closest_intersection: Option<f64> = None;

        for triangle in self.triangles() {
            if let Some(intersection_point) = triangle.intersect_ray_point(ray) {
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

    /// Test for an intersection point with a ray.
    pub fn intersect_ray_distance_normal(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        if !self.aabb.intersect_ray(ray) {
            return None;
        }

        let mut closest_normal: Option<(f64, Unit<Vector3<f64>>)> = None;

        for triangle in self.triangles() {
            if let Some((distance, normal)) = triangle.intersect_ray_distance_normal(ray) {
                if closest_normal.is_none() || distance < closest_normal.unwrap().0 {
                    closest_normal = Some((distance, normal));
                }
            }
        }

        closest_normal
    }
}
