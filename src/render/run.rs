use indicatif::{ProgressBar, ProgressStyle};
use palette::LinSrgba;
use std::fs::create_dir_all;

use crate::{
    geometry::Ray,
    render::{Sample, Settings, Tile},
    world::{Camera, Scene},
};

pub fn render(settings: &Settings, scene: &Scene, camera_id: &str, camera: &Camera) {
    let output_directory = settings.output_directory().join(camera_id);
    create_dir_all(&output_directory).expect("Unable to create camera output directory");

    let [rows, columns] = camera.num_tiles();
    let total_num_tiles = rows * columns;

    let pb = ProgressBar::new(total_num_tiles as u64).with_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.green/red}] [{pos}/{len}] {percent}% ({eta}) {msg}")
                .expect("Failed to set progress-bar style.")
                .progress_chars("\\/"),
        );
    (0..total_num_tiles).into_iter().for_each(|n| {
        pb.inc(1);
        let row = n % rows;
        let column = n / rows;
        let tile = render_tile(scene, camera, [row, column]);
        let file_name = output_directory.join(format!("tile_{:03}_{:03}.png", row, column));
        tile.save(&file_name);
    });
    pb.finish_with_message(format!("Rendered {}", camera_id));
}

fn render_tile(scene: &Scene, camera: &Camera, tile_index: [usize; 2]) -> Tile {
    let mut tile = Tile::new(
        tile_index,
        camera.tile_resolution(),
        [0.0, 0.0, 0.0, 0.0].into(),
    );

    tile.data.par_mapv_inplace(|sample| {
        let mut sample = sample;
        for xi in 0..camera.super_samples_per_axis() {
            for yi in 0..camera.super_samples_per_axis() {
                let ray = camera.generate_ray(sample.pixel_index, [xi, yi]);
                sample += sample_stencil(scene, sample.pixel_index, ray.clone());
                sample += sample_distance(scene, sample.pixel_index, ray);
                sample += sample_normal(scene, sample.pixel_index, ray);
            }
        }
        sample /= (camera.super_samples_per_axis() * camera.super_samples_per_axis() * 3) as f32;
        sample
    });

    tile
}

fn sample_stencil(scene: &Scene, pixel_index: [usize; 2], ray: Ray) -> Sample {
    if scene.ray_intersect(&ray) {
        let r = ray.direction().x.abs() as f32;
        let g = ray.direction().y.abs() as f32;
        let b = ray.direction().z.abs() as f32;
        return Sample::new(pixel_index, LinSrgba::new(r, g, b, 1.0));
    }

    return Sample::new(pixel_index, LinSrgba::new(0.0, 0.0, 0.0, 0.0));
}

fn sample_distance(scene: &Scene, pixel_index: [usize; 2], ray: Ray) -> Sample {
    if let Some(distance) = scene.ray_intersect_distance(&ray) {
        let scale = 1.0 / 20.0;
        let r = scale * distance as f32;
        let g = scale * distance as f32;
        let b = scale * distance as f32;
        return Sample::new(pixel_index, LinSrgba::new(r, g, b, 1.0));
    }

    return Sample::new(pixel_index, LinSrgba::new(0.0, 0.0, 0.0, 0.0));
}

fn sample_normal(scene: &Scene, pixel_index: [usize; 2], ray: Ray) -> Sample {
    if let Some((_distance, normal)) = scene.ray_intersect_distance_normal(&ray) {
        let r = normal.x.abs() as f32;
        let g = normal.y.abs() as f32;
        let b = normal.z.abs() as f32;
        return Sample::new(pixel_index, LinSrgba::new(r, g, b, 1.0));
    }

    return Sample::new(pixel_index, LinSrgba::new(0.0, 0.0, 0.0, 0.0));
}
