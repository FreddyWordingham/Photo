use indicatif::ProgressBar;
use std::{fs::create_dir_all, path::Path};

use crate::{
    input::Settings,
    render::Sample,
    render::Tile,
    utility::terminal,
    world::{Camera, Scene},
};

/// Render the all the cameras.
pub fn render_all_cameras(settings: &Settings, scene: &Scene, output_directory: &Path) {
    debug_assert!(settings.is_valid());

    for (camera_name, camera_settings) in settings.cameras() {
        println!("{}", terminal::subheading(camera_name));
        let camera = camera_settings.build();
        println!("{}", camera);

        // Create output directory for camera.
        let camera_output_directory = output_directory.join(camera_name);
        if !camera_output_directory.exists() {
            create_dir_all(&camera_output_directory).unwrap();
        }

        render_camera(settings, scene, &camera, &camera_output_directory);
    }
}

/// Render a camera's image.
pub fn render_camera(
    settings: &Settings,
    scene: &Scene,
    camera: &Camera,
    camera_output_directory: &Path,
) {
    let [num_tile_rows, num_tile_columns] = camera.image_resolution();
    let total_num_tiles = num_tile_rows * num_tile_columns;

    let pb = ProgressBar::new((total_num_tiles) as u64);
    pb.inc(0);
    (0..total_num_tiles).into_iter().for_each(|n| {
        let row = n % num_tile_rows;
        let column = n / num_tile_rows;
        let tile = render_tile(settings, scene, camera, [row, column]);
        tile.save(camera_output_directory);
        pb.inc(1);
    });
    pb.finish();
}

/// Render a single tile.
fn render_tile(
    settings: &Settings,
    scene: &Scene,
    camera: &Camera,
    tile_index: [usize; 2],
) -> Tile {
    let mut tile = Tile::new(tile_index, camera.tile_resolution());
    tile.data
        .par_mapv_inplace(|sample| run_sample(settings, scene, camera, tile_index, sample));

    if settings.print_tiles_to_terminal() {
        println!("{}", tile);
    }

    tile
}

/// Run a single pixel sample.
fn run_sample(
    settings: &Settings,
    _scene: &Scene,
    camera: &Camera,
    tile_index: [usize; 2],
    mut sample: Sample,
) -> Sample {
    debug_assert!(settings.is_valid());

    let row = sample.sample_index[0] + (tile_index[0] * camera.tile_resolution()[0]);
    let column = sample.sample_index[1] + (tile_index[1] * camera.tile_resolution()[1]);

    let d_phi = (camera.field_of_view() / camera.aspect_ratio())
        / (camera.image_resolution()[0] * camera.tile_resolution()[0]) as f64;
    let d_theta = camera.field_of_view()
        / (camera.image_resolution()[1] * camera.tile_resolution()[1]) as f64;

    let phi = (row as f64 * d_phi) - (camera.field_of_view() / camera.aspect_ratio() * 0.5);
    let theta = (column as f64 * d_theta) - (camera.field_of_view() * 0.5);

    let forwards = camera.forwards();
    let right = camera.right();
    let up = camera.up();

    let vertical_rotation = nalgebra::Rotation3::from_axis_angle(&up, phi);
    let lateral_rotation = nalgebra::Rotation3::from_axis_angle(&right, theta);

    let direction = lateral_rotation * vertical_rotation * forwards;

    sample.colour.red = direction.x.abs() as f32;
    sample.colour.green = direction.y.abs() as f32;
    sample.colour.blue = direction.z.abs() as f32;

    sample
}
