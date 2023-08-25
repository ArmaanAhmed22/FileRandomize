use file_randomize::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli = Cli::parse();
    run_parsing(&mut cli)?;
    run_randomize(&cli)?;
    Ok(())
}
