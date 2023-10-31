pub struct Mesh {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    coordinates: Vec<[f32; 2]>,
    faces: Vec<Face>,
}

struct Face {
    positions_indices: [u32; 3],
    coordinate_indices: [u32; 3],
    normal_indices: [u32; 3],
}

impl Mesh {
    pub fn load(path: &str) -> Self {
        println!("Reading mesh...: {}", path);

        let string = std::fs::read_to_string(path).unwrap();

        let positions = Self::init_positions(&string);
        let normals = Self::init_normals(&string);
        let coordinates = Self::init_coordinates(&string);
        let faces = Self::init_faces(&string);

        Mesh {
            positions,
            normals,
            coordinates,
            faces,
        }
    }

    fn init_positions(string: &str) -> Vec<[f32; 3]> {
        let mut positions = Vec::new();

        for line in string.lines() {
            if line.starts_with("v ") {
                let mut iter = line.split_whitespace().skip(1);

                let x = iter.next().unwrap().parse::<f32>().unwrap();
                let y = iter.next().unwrap().parse::<f32>().unwrap();
                let z = iter.next().unwrap().parse::<f32>().unwrap();

                positions.push([x, y, z]);
            }
        }

        positions
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

    fn init_faces(string: &str) -> Vec<Face> {
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

                faces.push(Face {
                    positions_indices: [indices[0][0], indices[1][0], indices[2][0]],
                    coordinate_indices: [indices[0][1], indices[1][1], indices[2][1]],
                    normal_indices: [indices[0][2], indices[1][2], indices[2][2]],
                });
            }
        }

        faces
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

    pub fn positions_data(&self) -> Vec<f32> {
        debug_assert!(self.is_valid());

        self.positions
            .iter()
            .map(|[px, py, pz]| [*px, *py, *pz, 0.0])
            .flatten()
            .collect()
    }

    pub fn position_indices_data(&self) -> Vec<u32> {
        debug_assert!(self.is_valid());

        self.faces
            .iter()
            .map(|f| f.positions_indices)
            .map(|[px, py, pz]| [px, py, pz, 0])
            .flatten()
            .collect()
    }

    pub fn normals_data(&self) -> Vec<f32> {
        debug_assert!(self.is_valid());

        self.normals
            .iter()
            .map(|[nx, ny, nz]| [*nx, *ny, *nz, 0.0])
            .flatten()
            .collect()
    }

    pub fn normal_indices_data(&self) -> Vec<u32> {
        debug_assert!(self.is_valid());

        self.faces
            .iter()
            .map(|f| f.normal_indices)
            .map(|[nx, ny, nz]| [nx, ny, nz, 0])
            .flatten()
            .collect()
    }
}
