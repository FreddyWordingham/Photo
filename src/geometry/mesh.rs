use crate::geometry::{Triangle, AABB};

#[derive(Debug, Clone, Copy)]
struct TriangleData {
    positions_indices: [u32; 3],
    coordinate_indices: [u32; 3],
    normal_indices: [u32; 3],
}

pub struct Mesh {
    aabb: AABB,
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    coordinates: Vec<[f32; 2]>,
    faces: Vec<TriangleData>,
}

impl Mesh {
    pub fn load(path: &str) -> Self {
        let string = std::fs::read_to_string(path).unwrap();

        let (aabb, positions) = Self::init_positions(&string);
        debug_assert!(!positions.is_empty());

        let normals = Self::init_normals(&string);
        debug_assert!(!normals.is_empty());

        let coordinates = Self::init_coordinates(&string);
        debug_assert!(!coordinates.is_empty());

        let faces = Self::init_faces(&string);
        debug_assert!(!faces.is_empty());

        Mesh {
            aabb,
            positions,
            normals,
            coordinates,
            faces,
        }
    }

    fn init_positions(string: &str) -> (AABB, Vec<[f32; 3]>) {
        let mut mins = [std::f32::MAX; 3];
        let mut maxs = [-std::f32::MAX; 3];

        let mut positions = Vec::new();

        for line in string.lines() {
            if line.starts_with("v ") {
                let mut iter = line.split_whitespace().skip(1);

                let x = iter.next().unwrap().parse::<f32>().unwrap();
                let y = iter.next().unwrap().parse::<f32>().unwrap();
                let z = iter.next().unwrap().parse::<f32>().unwrap();

                positions.push([x, y, z]);

                if x < mins[0] {
                    mins[0] = x;
                }
                if x > maxs[0] {
                    maxs[0] = x;
                }

                if y < mins[1] {
                    mins[1] = y;
                }
                if y > maxs[1] {
                    maxs[1] = y;
                }

                if z < mins[2] {
                    mins[2] = z;
                }
                if z > maxs[2] {
                    maxs[2] = z;
                }
            }
        }

        (AABB::new(mins, maxs), positions)
    }

    fn init_normals(string: &str) -> Vec<[f32; 3]> {
        let mut normals = Vec::new();

        for line in string.lines() {
            if line.starts_with("vn ") {
                let mut iter = line.split_whitespace().skip(1);

                let x = iter.next().unwrap().parse::<f32>().unwrap();
                let y = iter.next().unwrap().parse::<f32>().unwrap();
                let z = iter.next().unwrap().parse::<f32>().unwrap();

                normals.push([x, y, z]);
            }
        }

        normals
    }

    fn init_coordinates(string: &str) -> Vec<[f32; 2]> {
        let mut normals = Vec::new();

        for line in string.lines() {
            if line.starts_with("vt ") {
                let mut iter = line.split_whitespace().skip(1);

                let x = iter.next().unwrap().parse::<f32>().unwrap();
                let y = iter.next().unwrap().parse::<f32>().unwrap();

                normals.push([x, y]);
            }
        }

        normals
    }

    fn init_faces(string: &str) -> Vec<TriangleData> {
        let mut faces = Vec::new();

        for line in string.lines() {
            if line.starts_with("f ") {
                let indices: Vec<_> = line
                    .split_whitespace()
                    .skip(1)
                    .map(|s| {
                        s.split('/')
                            .map(|x| x.parse::<u32>().unwrap() - 1)
                            .collect::<Vec<_>>()
                    })
                    .collect();

                faces.push(TriangleData {
                    positions_indices: [indices[0][0], indices[1][0], indices[2][0]],
                    coordinate_indices: [indices[0][1], indices[1][1], indices[2][1]],
                    normal_indices: [indices[0][2], indices[1][2], indices[2][2]],
                });
            }
        }

        faces
    }

    pub fn aabb(&self) -> AABB {
        debug_assert!(self.is_valid());

        self.aabb
    }

    pub fn create_triangle(&self, index: usize) -> Triangle {
        debug_assert!(self.is_valid());
        debug_assert!(index < self.faces.len());

        let face = &self.faces[index];

        let positions = [
            self.positions[face.positions_indices[0] as usize],
            self.positions[face.positions_indices[1] as usize],
            self.positions[face.positions_indices[2] as usize],
        ];
        let normals = [
            self.normals[face.normal_indices[0] as usize],
            self.normals[face.normal_indices[1] as usize],
            self.normals[face.normal_indices[2] as usize],
        ];
        let coordinates = [
            self.coordinates[face.coordinate_indices[0] as usize],
            self.coordinates[face.coordinate_indices[1] as usize],
            self.coordinates[face.coordinate_indices[2] as usize],
        ];

        Triangle::new(positions, coordinates, normals)
    }

    pub fn create_triangles(&self) -> Vec<Triangle> {
        debug_assert!(self.is_valid());

        let mut triangles = Vec::with_capacity(self.faces.len());
        for face in &self.faces {
            let positions = [
                self.positions[face.positions_indices[0] as usize],
                self.positions[face.positions_indices[1] as usize],
                self.positions[face.positions_indices[2] as usize],
            ];
            let normals = [
                self.normals[face.normal_indices[0] as usize],
                self.normals[face.normal_indices[1] as usize],
                self.normals[face.normal_indices[2] as usize],
            ];
            let coordinates = [
                self.coordinates[face.coordinate_indices[0] as usize],
                self.coordinates[face.coordinate_indices[1] as usize],
                self.coordinates[face.coordinate_indices[2] as usize],
            ];

            triangles.push(Triangle::new(positions, coordinates, normals));
        }

        triangles
    }

    pub fn is_valid(&self) -> bool {
        let max_position_index = self
            .faces
            .iter()
            .map(|face| face.positions_indices.iter().max().unwrap())
            .max()
            .unwrap();

        let max_coordinate_index = self
            .faces
            .iter()
            .map(|face| face.coordinate_indices.iter().max().unwrap())
            .max()
            .unwrap();

        let max_normal_index = self
            .faces
            .iter()
            .map(|face| face.normal_indices.iter().max().unwrap())
            .max()
            .unwrap();

        !self.positions.is_empty()
            && !self.normals.is_empty()
            && !self.coordinates.is_empty()
            && !self.faces.is_empty()
            && *max_position_index == (self.positions.len() as u32 - 1)
            && *max_coordinate_index == (self.coordinates.len() as u32 - 1)
            && *max_normal_index == (self.normals.len() as u32 - 1)
    }

    pub fn positions_buffer(&self, tag: f32) -> Vec<f32> {
        debug_assert!(self.is_valid());

        self.positions
            .iter()
            .map(|[px, py, pz]| [*px, *py, *pz, tag])
            .flatten()
            .collect()
    }

    pub fn position_indices_buffer(&self, tag: u32, offset: u32) -> Vec<u32> {
        debug_assert!(self.is_valid());

        self.faces
            .iter()
            .map(|f| f.positions_indices)
            .map(|[px, py, pz]| [offset + px, offset + py, offset + pz, tag])
            .flatten()
            .collect()
    }

    pub fn normals_buffer(&self, tag: f32) -> Vec<f32> {
        debug_assert!(self.is_valid());

        self.normals
            .iter()
            .map(|[nx, ny, nz]| [*nx, *ny, *nz, tag])
            .flatten()
            .collect()
    }

    pub fn normal_indices_buffer(&self, tag: u32, offset: u32) -> Vec<u32> {
        debug_assert!(self.is_valid());

        self.faces
            .iter()
            .map(|f| f.normal_indices)
            .map(|[nx, ny, nz]| [offset + nx, offset + ny, offset + nz, tag])
            .flatten()
            .collect()
    }
}
