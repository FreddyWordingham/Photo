use indicatif::{ProgressBar, ProgressStyle};
use nalgebra::Unit;
use palette::LinSrgba;
use std::fs::create_dir_all;

use crate::{
    assets::{Gradient, Material},
    geometry::Ray,
    render::{Hit, Sample, Settings, Tile},
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
        let tile = render_tile(settings, scene, camera, [row, column]);
        let file_name = output_directory.join(format!("tile_{:03}_{:03}.png", row, column));
        tile.save(&file_name);
    });
    pb.finish_with_message(format!("Rendered {}", camera_id));
}

fn render_tile(
    settings: &Settings,
    scene: &Scene,
    camera: &Camera,
    tile_index: [usize; 2],
) -> Tile {
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
                sample += sample_full(settings, scene, sample.pixel_index, ray.clone());
            }
        }
        sample /= (camera.super_samples_per_axis() * camera.super_samples_per_axis()) as f32;
        sample
    });

    tile
}

fn _sample_stencil(scene: &Scene, pixel_index: [usize; 2], ray: Ray) -> Sample {
    if scene.ray_intersect(&ray) {
        let r = ray.direction().x.abs() as f32;
        let g = ray.direction().y.abs() as f32;
        let b = ray.direction().z.abs() as f32;
        return Sample::new(pixel_index, LinSrgba::new(r, g, b, 1.0));
    }

    return Sample::new(pixel_index, LinSrgba::new(0.0, 0.0, 0.0, 0.0));
}

fn _sample_distance(scene: &Scene, pixel_index: [usize; 2], ray: Ray) -> Sample {
    if let Some(distance) = scene.ray_intersect_distance(&ray) {
        let scale = 1.0 / 20.0;
        let r = scale * distance as f32;
        let g = scale * distance as f32;
        let b = scale * distance as f32;
        return Sample::new(pixel_index, LinSrgba::new(r, g, b, 1.0));
    }

    return Sample::new(pixel_index, LinSrgba::new(0.0, 0.0, 0.0, 0.0));
}

fn _sample_side(scene: &Scene, pixel_index: [usize; 2], ray: Ray) -> Sample {
    if let Some(hit) = scene.ray_intersect_hit(&ray) {
        let r = if hit.is_inside { 1.0 } else { 0.0 };
        let g = 0.0;
        let b = if hit.is_inside { 0.0 } else { 1.0 };
        return Sample::new(pixel_index, LinSrgba::new(r, g, b, 1.0));
    }

    return Sample::new(pixel_index, LinSrgba::new(0.0, 0.0, 0.0, 0.0));
}

fn _sample_normal(scene: &Scene, pixel_index: [usize; 2], ray: Ray) -> Sample {
    if let Some(hit) = scene.ray_intersect_hit(&ray) {
        let r = hit.normal.x.abs() as f32;
        let g = hit.normal.y.abs() as f32;
        let b = hit.normal.z.abs() as f32;
        return Sample::new(pixel_index, LinSrgba::new(r, g, b, 1.0));
    }

    return Sample::new(pixel_index, LinSrgba::new(0.0, 0.0, 0.0, 0.0));
}

fn _sample_material(
    settings: &Settings,
    scene: &Scene,
    pixel_index: [usize; 2],
    ray: Ray,
) -> Sample {
    if let Some(hit) = scene.ray_intersect_hit(&ray) {
        let hit_position = ray.origin() + ray.direction().as_ref() * hit.distance;
        let sun_direction = Unit::new_normalize(settings.sun_position() - hit_position);
        let lightness = (hit.smooth_normal.dot(&sun_direction) as f32).max(0.0);
        match hit.material {
            Material::Diffuse { colour } => {
                let c = colour.sample(lightness);
                return Sample::new(pixel_index, c);
            }
            Material::Reflective { colour, .. } => {
                let c = colour.sample(lightness);
                return Sample::new(pixel_index, c);
            }
            Material::Refractive { colour, .. } => {
                let c = colour.sample(lightness);
                return Sample::new(pixel_index, c);
            }
        }
    }

    return Sample::new(pixel_index, LinSrgba::new(0.0, 0.0, 0.0, 0.0));
}

fn _sample_shadow(settings: &Settings, scene: &Scene, pixel_index: [usize; 2], ray: Ray) -> Sample {
    if let Some(hit) = scene.ray_intersect_hit(&ray) {
        let mut hit_position = ray.origin() + ray.direction().as_ref() * hit.distance;
        hit_position += hit.normal.as_ref() * 0.0001;
        let sun_direction = Unit::new_normalize(settings.sun_position() - hit_position);

        let shadow_ray = Ray::new(hit_position, sun_direction);
        let shadow = if let Some(distance) = scene.ray_intersect_distance(&shadow_ray) {
            (distance as f32 / 20.0).min(1.0)
        } else {
            1.0
        };
        let lightness = (hit.smooth_normal.dot(&sun_direction) as f32).max(0.0);

        match hit.material {
            Material::Diffuse { colour } => {
                let mut c = colour.sample(lightness);
                c.red *= shadow;
                c.green *= shadow;
                c.blue *= shadow;
                return Sample::new(pixel_index, c);
            }
            Material::Reflective { colour, .. } => {
                let mut c = colour.sample(lightness);
                c.red *= shadow;
                c.green *= shadow;
                c.blue *= shadow;
                return Sample::new(pixel_index, c);
            }
            Material::Refractive { colour, .. } => {
                let mut c = colour.sample(lightness);
                c.red *= shadow;
                c.green *= shadow;
                c.blue *= shadow;
                return Sample::new(pixel_index, c);
            }
        }
    }

    return Sample::new(pixel_index, LinSrgba::new(0.0, 0.0, 0.0, 0.0));
}

fn sample_full(
    settings: &Settings,
    scene: &Scene,
    pixel_index: [usize; 2],
    mut ray: Ray,
) -> Sample {
    let mut current_weight = 1.0;
    let mut current_colour = LinSrgba::new(0.0, 0.0, 0.0, 1.0);
    let mut loops = 0;
    while let Some(hit) = scene.ray_intersect_hit(&ray) {
        let (weight, colour, new_ray) = match hit.material {
            Material::Diffuse { colour } => run_diffuse(settings, scene, ray, hit, colour),
            Material::Reflective {
                colour,
                reflectivity,
            } => run_reflective(settings, scene, ray, hit, colour, *reflectivity),
            Material::Refractive {
                colour,
                refractive_index,
            } => run_refractive(settings, scene, ray, hit, colour, *refractive_index),
        };

        // current_colour += colour * (current_weight * (1.0 - weight)) as f32;
        let new_colour = colour * (current_weight * (1.0 - weight)) as f32;
        current_colour.red += new_colour.red;
        current_colour.green += new_colour.green;
        current_colour.blue += new_colour.blue;
        current_weight *= weight;

        if current_weight < settings.min_weight() {
            break;
        }
        if loops > settings.max_loops() {
            break;
        }

        ray = new_ray;
        loops += 1;
    }

    return Sample::new(pixel_index, current_colour);
}

fn run_diffuse(
    settings: &Settings,
    scene: &Scene,
    ray: Ray,
    hit: Hit,
    colour: &Gradient,
) -> (f64, LinSrgba, Ray) {
    let mut hit_position = ray.origin() + ray.direction().as_ref() * hit.distance;
    hit_position += hit.normal.as_ref() * 0.0001;
    let sun_direction = Unit::new_normalize(settings.sun_position() - hit_position);
    let shadow_ray = Ray::new(hit_position, sun_direction);

    let lightness = (hit.smooth_normal.dot(&sun_direction) as f32).max(0.0);
    let shadow = if scene.ray_intersect(&shadow_ray) {
        0.0
    } else {
        1.0
    };

    let mut c = colour.sample(lightness);
    c.red *= shadow;
    c.green *= shadow;
    c.blue *= shadow;

    (0.0, c, ray)
}

fn run_reflective(
    settings: &Settings,
    scene: &Scene,
    ray: Ray,
    hit: Hit,
    colour: &Gradient,
    reflectivity: f64,
) -> (f64, LinSrgba, Ray) {
    let mut hit_position = ray.origin() + ray.direction().as_ref() * hit.distance;
    hit_position += (hit.normal.as_ref() * 0.01) * (if hit.is_inside { -1.0 } else { 1.0 });
    let sun_direction = Unit::new_normalize(settings.sun_position() - hit_position);
    let shadow_ray = Ray::new(hit_position, sun_direction);

    let lightness = (hit.smooth_normal.dot(&sun_direction) as f32).max(0.0);
    let shadow = if scene.ray_intersect(&shadow_ray) {
        0.0
    } else {
        1.0
    };

    let mut c = colour.sample(lightness);
    c.red *= shadow;
    c.green *= shadow;
    c.blue *= shadow;

    let reflect_direction = Unit::new_normalize(
        ray.direction().as_ref() - 2.0 * ray.direction().dot(&hit.normal) * hit.normal.as_ref(),
    );

    (reflectivity, c, Ray::new(hit_position, reflect_direction))
}

fn run_refractive(
    _settings: &Settings,
    _scene: &Scene,
    ray: Ray,
    _hit: Hit,
    _colour: &Gradient,
    _refractive_index: f64,
) -> (f64, LinSrgba, Ray) {
    (0.0, [1.0, 0.0, 1.0, 1.0].into(), ray)
}
