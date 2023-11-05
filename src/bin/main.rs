use photo::{
    input::Settings,
    run,
    utility::{setup, terminal},
    Scene,
};

fn main() {
    println!("{}", terminal::title("PHOTO!"));

    let settings_filepath = setup::read_command_line_arguments();
    let settings = Settings::load(&settings_filepath);
    let output_directory = setup::create_output_directory(&settings);
    println!("{}\n{}", terminal::heading("Settings"), settings);

    let scene = Scene::new();
    println!("{}\n{}", terminal::heading("Scene"), scene);

    run::all(&scene, &settings, &output_directory);
    println!("{}", terminal::heading("Done!"));
}
