use clap::{Arg, Command};
use colored::*;
// use csv::{ReaderBuilder, WriterBuilder};
use csv::ReaderBuilder;
use std::fs::create_dir_all;
use std::io;
use std::{error::Error, fs::File, io::Write};

mod toml_extract; // extract and print the version information according to the toml file

fn show_banner() {
    // banner ref: https://manytools.org/hacker-tools/ascii-banner/

    //logo design: "ticks", use "█" to replace "/\" chars, "_" replaced with space
    //     let banner = String::from(
    //         "
    // grade_alert_rs
    //         "
    //     );
    //     colour_print(&banner, "purple")

    let msg = format!("\n\t grade_alert_rs\n").bright_green().bold();
    println!("{}", msg);
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
    // Show the banner
    show_banner();

    // Display version information from the TOML file
    toml_extract::main();

    // Parse command-line arguments
    let matches = parse_arguments();

    // Create the output directory
    let output_dir = "0_out";
    create_dir_all(output_dir)?;

    // Get the input and pairings file paths
    let input_file = matches.get_one::<String>("input").unwrap();
    let pairings_file = matches.get_one::<String>("pairings").unwrap();

    // Process the input CSV file
    process_csv(input_file, output_dir)?;

    // Process the pairings file
    process_pairings(pairings_file, output_dir);

    Ok(())
}

fn parse_arguments() -> clap::ArgMatches {
    Command::new("Grade Alert Utility")
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
                .default_value("pairings.txt")
                .required(false)
                .help("Pairings file to determine file copy destinations"),
        )
        .get_matches()
}

fn process_csv(input_file: &str, output_dir: &str) -> Result<(), Box<dyn Error>> {
    // Open the input CSV file
    let input = File::open(input_file)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(input);

    // Read the headers
    let headers = rdr.headers()?.clone();
    println!("Headers: {:?}\n", headers);

    // Process each row in the CSV file
    for result in rdr.records() {
        let record = result?;

        // Generate the filename from the first column
        let filename = format!(
            "{}/{}.md",
            output_dir,
            record.get(0).unwrap().trim().replace(" ", "")
        );

        // Create and write to the markdown file
        let mut file = File::create(&filename)?;
        writeln!(file, "# {}\n", record.get(0).unwrap())?; // Title from the first column

        // Write each column's data with its header
        for (header, value) in headers.iter().zip(record.iter()) {
            writeln!(file, "**{}**: {}", header, value)?;
        }
    }

    Ok(())
}

fn process_pairings(pairings_file: &str, output_dir: &str) {
    let mut dir_names = std::collections::HashSet::new(); // Use a HashSet to avoid duplicates

    // Check if the pairings file exists
    if !std::path::Path::new(pairings_file).exists() {
        let msg = format!("\t Pairings file does not exist: {}", pairings_file)
            .bright_red()
            .bold();
        eprintln!("{}", msg);
        return;
    }

    // Read the pairings file
    let pairings_content = std::fs::read_to_string(pairings_file).expect("Missing pairings.txt?");
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


            // Add the destination directory to the set
            dir_names.insert(destination_dir.to_string());

            // Check if the destination directory exists
            if !std::path::Path::new(destination_dir).exists() {
                let msg = format!(
                    "\t Directory does not exist: {}.\n\t File {} was not copied.",
                    destination_dir, source_file
                )
                .bright_red()
                .bold();
                eprintln!("{}", msg);
                continue;
            }

            // Copy the file to the destination directory
            let destination_file = format!("{}{}", destination_dir, parts[0]);
            if let Err(e) = std::fs::copy(&source_file, &destination_file) {
                let msg = format!(
                    "\t Failed to copy file {} to {}: {}",
                    source_file, destination_file, e
                )
                .bright_red()
                .bold();
                eprintln!("{}", msg);
            } else {
                let msg = format!("\t Copied file {} to {}", source_file, destination_file)
                    .bright_cyan()
                    .bold();
                eprintln!("{}", msg);
            }
        }
    }

    // Write the list of directories to dirNames.txt
    let dir_names_file = format!("{}/dirNames.txt", output_dir);
    let mut file = File::create(&dir_names_file).expect("Failed to create dirNames.txt");
    for dir in dir_names {
        writeln!(file, "{}", dir).expect("Failed to write to dirNames.txt");
    }

    let msg = format!("\t Saved directory names to {}", dir_names_file)
        .bright_green()
        .bold();
    println!("{}", msg);
}
