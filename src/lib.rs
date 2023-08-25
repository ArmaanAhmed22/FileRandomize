pub use clap::Parser;
use std::path::{Path, PathBuf};
use rand::{distributions::Alphanumeric, Rng};
use colored::Colorize;
use filetime::{set_file_mtime, FileTime};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub directory: String,

    #[arg(short, long, default_value_t = 20, value_parser=length_correct)]
    pub length: i32,

    #[arg(short, long)]
    pub yes: bool,
}

pub fn length_correct(length: &str) -> Result<i32, String> {
    let length = length.parse::<i32>().unwrap_or(-1);
    if length > 0 {
        Ok(length)
    } else {
        Err("Length invalid".to_string())
    }
}

fn get_log_number_to_reach_percent(percent: f64, length: i32) -> f64 {
    0.5 *f64::log10(2_f64) + (length as f64) * 0.5 * f64::log10(36_f64)+ 0.5 * f64::log10(f64::log(1_f64 / (1_f64-percent), std::f64::consts::E))
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

pub fn run_parsing(cli: &mut Cli) -> Result<(), Box<dyn std::error::Error>> {
    let mut response_to_continue = cli.yes;

    while !response_to_continue {
        println!("For a 1% chance of a collision, there needs to be 10^{:.2} objects. Change length ({}) or enter yes to continue: ",get_log_number_to_reach_percent(0.01, cli.length),cli.length);
        let mut response = String::new();
        std::io::stdin().read_line(&mut response)?;
        let response = response.trim();
        match response.to_lowercase().as_str() {
            "yes" | "y" => {
                response_to_continue = true;
                continue;
            },
            _ => {}
        }
        //Convert response to integer
        cli.length = match length_correct(response.trim()) {
            Ok(response) => response,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

    }
    Ok(())
}

pub fn run_randomize(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
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