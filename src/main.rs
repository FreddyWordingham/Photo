use photo::{run, setup, Scene};

fn main() {
    let settings_filepath = setup::read_command_line_arguments();
    let settings = setup::load_settings(&settings_filepath);
    let output_directory = setup::create_output_directory(&settings);
    println!("-- Settings --------------------------\n{}", settings);

    let scene = Scene::new();
    println!("-- Scene -----------------------------\n{}", scene);

    run::render_image_in_tiles(&scene, &settings, &output_directory);
    println!("-- Complete --------------------------");
}
