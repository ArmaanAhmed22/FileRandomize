use clap::Parser;
use std::path::{Path, PathBuf};
use rand::{distributions::Alphanumeric, Rng};
use colored::Colorize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    directory: String,

    #[arg(long, default_value_t = 20)]
    length: i32
}

fn main() {
    let cli = Cli::parse();

    let directory = get_directory(&cli.directory).expect("Directory not found");

    for entry in directory.read_dir().expect("Error reading directory") {
        if let Ok(entry) = entry {
            let random_alphanumeric_string = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(cli.length.try_into().unwrap())
                .map(char::from)
                .collect::<String>();
            change_file_name(entry.path(), random_alphanumeric_string.as_str());
        }
    }

    //change_file_name(directory, "lol");




}

fn get_directory<'a>(directory_path: &'a str) -> Result<PathBuf,&'a str> {
    let path: PathBuf = PathBuf::from(directory_path);
    let is_dir = path.is_dir();
    if is_dir {
        Ok(path)
    } else {
        Err("Directory not found")
    }
}

fn change_file_name(path: impl AsRef<Path>, name: &str) {

    

    let path = path.as_ref();
    let mut result = path.to_owned();
    result.set_file_name(name);
    println!("{} {} {}", path.display(), "→".bold().red() ,path.display());
    if let Some(ext) = path.extension() {
        result.set_extension(ext);
    }
    std::fs::rename(path, result).expect("Error renaming file");
}
