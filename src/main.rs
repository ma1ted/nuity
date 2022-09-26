use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};
use directories::{BaseDirs, ProjectDirs};
use std::io::Write;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
    editor_path: String,
    project_path: String,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Unity Project
    New { name: String},
}

fn expand_path(path_raw: &str) -> PathBuf {
    let mut path = PathBuf::from(path_raw);

    if path_raw.starts_with("~") {
        let mut path_chars = path_raw.chars();
        path_chars.next();

        if let Some(base_dirs) = BaseDirs::new() {
            path = Path::new(base_dirs.home_dir().clone()).join(path);
        }
    }

    path
}

fn main() {

    // dbg!(BaseDirs::new().unwrap().home_dir().display());

    // Parse config file
    let config_file_name = "config.toml";

    if let Some(proj_dirs) = ProjectDirs::from("com", "Malted", "Nuity") {
        let config_dir = proj_dirs.config_dir();
        let config_file = config_dir.join(config_file_name);

        let config_file_contents = fs::read_to_string(&config_file).unwrap();

        let config: Config = toml::from_str(&config_file_contents)
            .expect(format!("Error while parsing {}. Make sure there are no TOML syntax errors then try again", &config_file.display()).as_str());
        
        // Validate the paths in the config file
        if !Path::new(&config.editor_path).exists() {
            panic!("The editor path \"{}\" does not exist. Edit {} to fix this", config.editor_path, &config_file.display());
        }
        

        // if !config_file.exists() {
        //     fs::create_dir_all(config_dir)
        //         .expect(format!("Failed to create {}", config_dir.display()).as_str());

        //     let mut config_file_ref = fs::File::create(config_file)
        //         .expect(format!("Failed to create {} at {}", config_file_name, config_dir.display()).as_str());

        //     config_file_ref.write_all("Helloo World\n".as_bytes())
        //         .expect(format!("Failed to write into {} at {}", config_file_name, config_dir.display()).as_str());
        
        //     println!("It doesn't look like you haven't used Nuity before. Please edit {} to include the Unity editors' install location and your Unity projects' location.", config_file)
        // }
    }

    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::New { name } => {
            let projects_dir = fs::read_dir("./").unwrap();
            for file in projects_dir {
                if name == file.unwrap().file_name().to_str().unwrap() {
                    panic!("A project with this name already exists.")
                }
            }

            // Command::new("Unity")
            //         .args(["-createProject", name])
            //         .unwrap();
            // -createProject <pathname>
        }
    }
}
