use clap::{Arg, Command};
use colored::*;
// use csv::{ReaderBuilder, WriterBuilder};
use csv::{ReaderBuilder};
use std::io;
use std::{error::Error, fs::File, io::Write};
use std::fs::{create_dir_all};

mod toml_extract; // extract and print the version information according to the toml file

fn show_banner() {
    // banner ref: https://manytools.org/hacker-tools/ascii-banner/

    //logo design: "ticks", use "█" to replace "/\" chars, "_" replaced with space
    let banner = String::from(
        "
grade_alert_rs    
        ",
    );

    colour_print(&banner, "purple")
}

fn colour_print(text: &str, colour: &str) {
    match colour {
        "flush_green" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            io::stdout().flush().unwrap();
            print!(" {}", text.bright_green().bold());
            io::stdout().flush().unwrap();
        }
        "green" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_green().bold());
        }
        "red" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_red().bold());
        }
        "cyan" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_cyan().bold());
        }
        "purple" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_purple().bold());
        }
        "blue" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_blue().bold());
        }
        "yellow" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_yellow().bold());
        }
        _ => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_yellow().bold());
        }
    }
} // end of colour_print()

fn main() -> Result<(), Box<dyn Error>> {
    // show the banner
    show_banner();

    toml_extract::main(); // display version information.

    // Set up command-line argument parsing with clap v4.x

    let matches = Command::new("Grade Alert Utility")
        .version("1.0")
        .author("Oliver Bonham-Carter <obonhamcarter@allegheny.edu>")
        .about("Grade alert utility for teachers")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_parser(clap::value_parser!(String))
                .required(true)
                .help("Input CSV file"),
        )
        .arg(
            Arg::new("pairings")
                .short('p')
                .long("pairings")
                .value_parser(clap::value_parser!(String))
                .required(false)
                .default_value("pairings.txt")
                .help("Pairings file to determine file copy destinations"),
        )
        .get_matches();

    let output_dir = "0_out";
    create_dir_all(output_dir)?;

    // Get the input, output, and column arguments
    let input_file = matches.get_one::<String>("input").unwrap();
    let pairings_file = matches.get_one::<String>("pairings").unwrap();

    // Open the input CSV file
    let input = File::open(input_file)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(input);


    // // Read the headers
    let headers = rdr.headers()?.clone();

    println!("Headers: {:?}\n", headers);

    // Process each row in the CSV file
    for result in rdr.records() {
        let record = result?;

        // Get the value of the first column to use as the filename
        let filename = format!("{}/{}.md", output_dir, record.get(0).unwrap().trim().replace(" ",""));
        
        // Leave this other filename treatment here for now ...
        // let filename = format!("{}/{}.md", output_dir, record.get(0).unwrap());

        // Create and write to the markdown file
        let mut file = File::create(&filename)?;
        writeln!(file, "# {}\n", record.get(0).unwrap())?; // Title from the first column

        // Write each column's data with its header
        for (header, value) in headers.iter().zip(record.iter()) {
            writeln!(file, "**{}**: {}", header, value)?;
        }
    }

    // Read the pairings file
    println!("Pairings file: {}", pairings_file);
    let pairings_content = std::fs::read_to_string(pairings_file)?;
    for line in pairings_content.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let source_file = format!("{}/{}", output_dir, parts[0]);
            let destination_dir = parts[1];

            // // Create the destination directories if they do not exist :: is this necessary?
            // if let Err(e) = std::fs::create_dir_all(destination_dir) {
            //     eprintln!("Failed to create directory {}: {}", destination_dir, e);
            //     continue;
            // } else {
            //     println!("Created directory: {}", destination_dir);
            // }

            // Check if the destination directory exists
            if !std::path::Path::new(destination_dir).exists() {
                eprintln!("Directory does not exist: {}. File {} was not copied.", destination_dir, source_file);
                continue;
            }

            // Copy the file to the destination directory
            let destination_file = format!("{}/{}", destination_dir, parts[0]);
            if let Err(e) = std::fs::copy(&source_file, &destination_file) {
                eprintln!("Failed to copy file {} to {}: {}", source_file, destination_file, e);
            } else {
                println!("Copied file {} to {}", source_file, destination_file);
            }
        }
    }

    Ok(())
}
