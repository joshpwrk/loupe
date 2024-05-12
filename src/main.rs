use rand::{distributions::Uniform, Rng};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    //////////////////////////
    // Create 1GB Text File //
    //////////////////////////

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
    let mut start_time = Instant::now(); // Start the timer

    let mut tot_lines = 0;

    while current_size < target_size {
        let line_length = rng.sample(&length_range);
        let line: String = (0..line_length)
            .map(|_| rng.sample(&rand::distributions::Alphanumeric))
            .map(char::from)
            .collect();

        file.write_all(format!("{}: ", tot_lines).as_bytes())?;
        file.write_all(line.as_bytes())?;

        file.write_all(b"\n")?;
        current_size += line.len() + 1; // +1 for the newline character
        tot_lines += 1;
    }

    file.flush()?;

    println!("Generated file {} with {} lines", path.display(), tot_lines);
    println!("Time to write file: {:?}", start_time.elapsed()); // Print the duration

    ///////////////////////////////////////////////////////
    // Read nth line from the file using Lines -> > 1 sec//
    ///////////////////////////////////////////////////////

    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let nth_line = 10000000;

    start_time = Instant::now();
    let line = reader
        .lines()
        .nth(nth_line - 1) // nth is zero-based, so subtract 1
        .transpose()? // Turn Option<Result<String>> into Result<Option<String>>
        .unwrap_or_else(|| "Line not found".to_string());

    println!("The 10 millionth line is: {}", line);
    println!("Time to read 10 millionth line: {:?}", start_time.elapsed()); // Print the duration

    //////////////////////////////////////////////////////
    // Seek directly to nth byte and read line -> < 1ms //
    //////////////////////////////////////////////////////

    let file_with_seek = File::open(&path)?;
    let mut reader_with_seek = BufReader::new(file_with_seek);

    // Seek to the 2 billionth byte of the file
    let start_time = Instant::now();
    reader_with_seek.seek(SeekFrom::Start(2_000_000_000))?;

    // Read the next 250 characters
    let mut buffer = vec![0; 250]; // Buffer to hold bytes
    reader_with_seek.read_exact(&mut buffer)?;

    // Convert buffer to string
    let result_string =
        String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    println!("The ~10 millionth line via seek: {}", result_string);

    println!(
        "Time taken to seek to ~10 millionth line via seek: {:?}",
        start_time.elapsed()
    );

    Ok(())
}
