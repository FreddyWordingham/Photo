use indicatif::{ProgressBar, ProgressStyle};
use nalgebra::{Unit, Vector3};
use palette::LinSrgba;
use std::fs::create_dir_all;

use crate::{
    assets::Material,
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

fn _sample_smooth_normal(scene: &Scene, pixel_index: [usize; 2], ray: Ray) -> Sample {
    if let Some(hit) = scene.ray_intersect_hit(&ray) {
        let r = hit.smooth_normal.x.abs() as f32;
        let g = hit.smooth_normal.y.abs() as f32;
        let b = hit.smooth_normal.z.abs() as f32;
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
    let smoothing_length = settings.smoothing_length();

    if let Some(hit) = scene.ray_intersect_hit(&ray) {
        let hit_position = ray.origin() + ray.direction().as_ref() * hit.distance;
        let shadow_ray_position = hit_position
            + (hit.smooth_normal.as_ref()
                * smoothing_length
                * if hit.is_inside { -1.0 } else { 1.0 });
        let sun_direction = Unit::new_normalize(settings.sun_position() - shadow_ray_position);
        let mut shadow_ray = Ray::new(shadow_ray_position, sun_direction);
        let lightness = (hit.smooth_normal.dot(&sun_direction) as f32).max(0.0);

        let mut shadow = 0.0;
        while shadow < 1.0 {
            if let Some(hit) = scene.ray_intersect_hit(&shadow_ray) {
                shadow += hit.material.opacity();
                shadow_ray.travel(hit.distance + smoothing_length);
            } else {
                break;
            }
        }
        let shadow = 1.0 - shadow.min(1.0) as f32;

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

fn sample_full(settings: &Settings, scene: &Scene, pixel_index: [usize; 2], ray: Ray) -> Sample {
    let depth = 0;
    let current_refractive_index = vec![1.0];
    let colour = sample_full_inner(settings, scene, ray, depth, current_refractive_index).unwrap();
    Sample::new(pixel_index, colour)
}

fn sample_full_inner(
    settings: &Settings,
    scene: &Scene,
    ray: Ray,
    depth: u16,
    mut current_refractive_index: Vec<f64>,
) -> Option<LinSrgba> {
    let smoothing_length = settings.smoothing_length();

    if depth > settings.max_loops() {
        return None;
    }

    if let Some(hit) = scene.ray_intersect_hit(&ray) {
        let hit_position = ray.origin() + ray.direction().as_ref() * hit.distance;
        let shadow_ray_position = hit_position
            + (hit.smooth_normal.as_ref()
                * smoothing_length
                * if hit.is_inside { -1.0 } else { 1.0 });
        let sun_direction = Unit::new_normalize(settings.sun_position() - shadow_ray_position);
        let mut shadow_ray = Ray::new(shadow_ray_position, sun_direction);
        let lightness = (hit.smooth_normal.dot(&sun_direction) as f32).max(0.0);

        let mut shadow = 0.0;
        while shadow < 1.0 {
            if let Some(hit) = scene.ray_intersect_hit(&shadow_ray) {
                shadow += hit.material.opacity();
                shadow_ray.travel(hit.distance + smoothing_length);
            } else {
                break;
            }
        }
        let shadow = 1.0 - shadow.min(1.0) as f32;

        match hit.material {
            Material::Diffuse { colour } => {
                let mut c = colour.sample(lightness);
                c.red *= shadow;
                c.green *= shadow;
                c.blue *= shadow;
                return Some(c);
            }
            Material::Reflective { colour, absorption } => {
                let mut c = colour.sample(lightness);
                c.red *= shadow;
                c.green *= shadow;
                c.blue *= shadow;

                let reflected_direction = Unit::new_normalize(
                    ray.direction().as_ref()
                        - 2.0
                            * ray.direction().dot(&hit.smooth_normal)
                            * if hit.is_inside { -1.0 } else { 1.0 }
                            * hit.smooth_normal.as_ref(),
                );
                let mut new_ray = Ray::new(hit_position, reflected_direction);
                new_ray.travel(smoothing_length);
                if let Some(oc) = sample_full_inner(
                    settings,
                    scene,
                    new_ray,
                    depth + 1,
                    current_refractive_index,
                ) {
                    return Some((c * *absorption as f32) + (oc * (1.0 - *absorption as f32)));
                } else {
                    // ??? Mixing with nothing
                    return Some(c);
                }
            }
            Material::Refractive {
                colour,
                absorption,
                refractive_index,
            } => {
                let mut c = colour.sample(lightness);
                c.red *= shadow;
                c.green *= shadow;
                c.blue *= shadow;

                let (curr_ref_index, next_ref_index) = if hit.is_inside {
                    (*refractive_index, current_refractive_index.pop().unwrap())
                } else {
                    let prev_ref_index = *current_refractive_index.last().unwrap();
                    current_refractive_index.push(*refractive_index);
                    (prev_ref_index, *refractive_index)
                };

                let reflected_direction = Unit::new_normalize(
                    ray.direction().as_ref()
                        - 2.0
                            * ray.direction().dot(&hit.smooth_normal)
                            * if hit.is_inside { -1.0 } else { 1.0 }
                            * hit.smooth_normal.as_ref(),
                );
                let mut reflected_ray = Ray::new(hit_position, reflected_direction);
                reflected_ray.travel(smoothing_length);

                if let Some(transmittance) = fresnel_transmittance(
                    ray.direction().as_ref(),
                    &(hit.smooth_normal.as_ref() * if hit.is_inside { -1.0 } else { 1.0 }),
                    curr_ref_index,
                    *refractive_index,
                ) {
                    if let Some(refracted_direction) = refract(
                        ray.direction().as_ref(),
                        &(hit.smooth_normal.as_ref() * if hit.is_inside { -1.0 } else { 1.0 }),
                        curr_ref_index / next_ref_index,
                    ) {
                        let mut refracted_ray = Ray::new(hit_position, refracted_direction);
                        refracted_ray.travel(smoothing_length);

                        let reflected_colour = sample_full_inner(
                            settings,
                            scene,
                            refracted_ray,
                            depth + 1,
                            current_refractive_index.clone(),
                        );
                        let refracted_colour = sample_full_inner(
                            settings,
                            scene,
                            refracted_ray,
                            depth + 1,
                            current_refractive_index,
                        );

                        if reflected_colour.is_none() && refracted_colour.is_none() {
                            // ??? Mixing with nothing
                            return Some(c);
                        }
                        if let Some(oc) = reflected_colour {
                            // ??? Mixing with nothing
                            return Some(
                                (c * *absorption as f32) + (oc * (1.0 - *absorption as f32)),
                            );
                        }
                        if let Some(oc) = refracted_colour {
                            // ??? Mixing with nothing
                            return Some(
                                (c * *absorption as f32) + (oc * (1.0 - *absorption as f32)),
                            );
                        }
                        return Some(
                            (c * *absorption as f32)
                                + (reflected_colour.unwrap()
                                    * (1.0 - *absorption as f32)
                                    * transmittance as f32)
                                + (refracted_colour.unwrap()
                                    * (1.0 - *absorption as f32)
                                    * (1.0 - transmittance as f32)),
                        );
                    } else {
                        if let Some(oc) = sample_full_inner(
                            settings,
                            scene,
                            reflected_ray,
                            depth + 1,
                            current_refractive_index,
                        ) {
                            return Some(
                                (c * *absorption as f32) + (oc * (1.0 - *absorption as f32)),
                            );
                        } else {
                            // ??? Mixing with nothing
                            return Some(c);
                        }
                    }
                } else {
                    if let Some(oc) = sample_full_inner(
                        settings,
                        scene,
                        reflected_ray,
                        depth + 1,
                        current_refractive_index,
                    ) {
                        return Some((c * *absorption as f32) + (oc * (1.0 - *absorption as f32)));
                    } else {
                        // ??? Mixing with nothing
                        return Some(c);
                    }
                }
            }
        }
    }

    return Some(LinSrgba::new(0.0, 0.0, 0.0, 1.0));
}

fn refract(v: &Vector3<f64>, n: &Vector3<f64>, eta: f64) -> Option<Unit<Vector3<f64>>> {
    let cos_theta = -v.dot(n);
    let k = 1.0 - eta * eta * (1.0 - cos_theta * cos_theta);
    if k < 0.0 {
        None
    } else {
        Some(Unit::new_normalize(
            eta * v + (eta * cos_theta - k.sqrt()) * n,
        ))
    }
}

fn fresnel_transmittance(
    incident: &Vector3<f64>,
    normal: &Vector3<f64>,
    n1: f64,
    n2: f64,
) -> Option<f64> {
    let cos_theta_i = -incident.dot(normal).max(0.0).min(1.0);
    let sin_theta_i = (1.0 - cos_theta_i * cos_theta_i).sqrt();
    let sin_theta_t = n1 / n2 * sin_theta_i;

    // Total internal reflection
    if sin_theta_t > 1.0 {
        return None;
    }

    let cos_theta_t = (1.0 - sin_theta_t * sin_theta_t).sqrt();
    let rs = (n1 * cos_theta_i - n2 * cos_theta_t) / (n1 * cos_theta_i + n2 * cos_theta_t);
    let rp = (n2 * cos_theta_i - n1 * cos_theta_t) / (n2 * cos_theta_i + n1 * cos_theta_t);

    Some(1.0 - (0.5 * (rs * rs + rp * rp)))
}
