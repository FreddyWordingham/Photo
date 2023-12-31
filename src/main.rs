use std::{env::args, error::Error, fs::create_dir_all, io, path::Path, process::exit};

use indicatif::{ProgressBar, ProgressStyle};

use photo::{
    input::Parameters,
    render::{run::render_tiles, Settings},
    world::{Camera, Scene},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Load parameters from file.
    let parameters = load_parameters()?;
    parameters.validate()?;

    // Build world components.
    let settings = parameters.build_settings();
    let spectra = parameters.build_spectra()?;
    let materials = parameters.build_materials(&spectra)?;
    let meshes =
        parameters.build_meshes(settings.mesh_bvh_max_children, settings.mesh_bvh_max_depth)?;
    let entities = parameters.build_entities(&materials, &meshes)?;
    let lights = parameters.build_lights();
    let cameras = parameters.build_cameras();
    drop(parameters);

    // Build scene.
    let scene = Scene::new(
        lights,
        entities,
        settings.scene_bvh_max_children,
        settings.scene_bvh_max_depth,
    );

    // Render [`Camera`] images.
    for (camera_name, camera) in cameras.iter() {
        render_camera_photo(&settings, &scene, camera, camera_name)?;
    }

    Ok(())
}

/// Read in the command line arguments, and return the described [`Parameters`].
///
/// # Errors
///
/// Returns a [`Box<dyn Error>`] if [`Parameters`] object cannot be constructed.
fn load_parameters() -> Result<Parameters, Box<dyn Error>> {
    // Read command line arguments.
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Usage: photo <path/to/parameters.yaml>");
        exit(1);
    }
    let parameters_path = Path::new(&args[1]);

    // Check if parameters file exists.
    if !parameters_path.exists() {
        eprintln!(
            "Error: parameters file `{}` does not exist.",
            parameters_path.display()
        );
        exit(1);
    }

    // Load parameters from file.
    Parameters::load(parameters_path)
}

/// Render a photograph use multiple threads.
///
/// # Errors
///
/// Returns a [`Box<dyn Error>`] if the output directory cannot be created,
/// or if an error occurs while rendering.
///
/// # Panics
///
/// Panics a [`Tile`] cannot be saved.
#[inline]
#[allow(clippy::expect_used, clippy::integer_division)]
pub fn render_camera_photo(
    settings: &Settings,
    scene: &Scene,
    camera: &Camera,
    image_name: &str,
) -> Result<(), io::Error> {
    // Create output directory.
    let output_directory = settings.output_directory.join(image_name);
    create_dir_all(&output_directory)?;

    let pb = create_progress_bar(camera.total_num_tiles() as u64);
    for tile in render_tiles(settings, scene, camera) {
        pb.inc(1);
        tile.save(&output_directory).expect("Failed to save tile.");
    }
    pb.finish();

    Ok(())
}

/// Create a progress bar for rendering a photograph.
#[must_use]
#[inline]
fn create_progress_bar(ticks: u64) -> ProgressBar {
    ProgressBar::new(ticks as u64).with_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.green/red}] [{pos}/{len}] {percent}% ({eta}) {msg}")
            .expect("Failed to set progress-bar style.")
            .progress_chars("\\/"),
    )
}
