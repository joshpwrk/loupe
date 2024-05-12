use rand::{distributions::Uniform, Rng};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <filename> <size in MB>", args[0]);
        std::process::exit(1);
    }

    let mut path = PathBuf::from("data"); // Specify the directory
    path.push(&args[1]); // Append the filename from command line argument

    let target_size_mb: usize = args[2].parse().unwrap(); // Target size in MB
    let target_size = target_size_mb * 1024 * 1024; // Convert MB to bytes

    let mut file = BufWriter::new(File::create(&path)?);
    let mut rng = rand::thread_rng();
    let length_range = Uniform::new(100, 200); // Range is exclusive of the upper bound

    let mut current_size = 0usize;
    let start_time = Instant::now(); // Start the timer

    while current_size < target_size {
        let line_length = rng.sample(&length_range);
        let line: String = (0..line_length)
            .map(|_| rng.sample(&rand::distributions::Alphanumeric))
            .map(char::from)
            .collect();

        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
        current_size += line.len() + 1; // +1 for the newline character
    }

    file.flush()?;

    println!("Generated file {}", path.display());
    println!("Time taken: {:?}", start_time.elapsed()); // Print the duration
    Ok(())
}
