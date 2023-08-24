use clap::Parser;
use std::path::{Path, PathBuf};
use rand::{distributions::Alphanumeric, Rng};
use colored::Colorize;
use filetime::{set_file_mtime, FileTime};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    directory: String,

    #[arg(long, default_value_t = 20)]
    length: i32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let directory = get_directory(&cli.directory)?;

    for entry in directory.read_dir()? {
        if let Ok(entry) = entry {
            let random_alphanumeric_string = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(cli.length.try_into().unwrap())
                .map(char::from)
                .collect::<String>();
            change_file_name(entry.path(), random_alphanumeric_string.as_str())?;
        }
    }

    Ok(())




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

fn change_file_name(path: impl AsRef<Path>, name: &str) -> Result<(), &str> {

    //Get original path name
    

    let metadata = std::fs::metadata(path.as_ref()).expect("Unable to read metadata");

    let modification_time = FileTime::from_last_modification_time(&metadata);

    let path = path.as_ref();
    let original_path_name = path.to_str().unwrap();
    let mut result = path.to_owned();
    result.set_file_name(name);

    //Get filename from result

    if let Some(ext) = path.extension() {
        result.set_extension(ext);
    }
    std::fs::rename(path, result.clone()).expect("Error renaming file");
    
    set_file_mtime(result.clone(), modification_time).expect("Error maintaining modification time");
    println!("{} {} {}", original_path_name, "→".bold().red() ,result.to_str().unwrap());

    Ok(())
}
