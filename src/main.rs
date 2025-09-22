// Import necessary crates for command-line parsing, colored output, CSV handling, and file I/O
use clap::{Arg, Command};
use colored::*;
// use csv::{ReaderBuilder, WriterBuilder};
use csv::ReaderBuilder;
use std::fs::create_dir_all;
use std::io;
use std::{error::Error, fs::File, io::Write};

// Import the module for extracting version info from Cargo.toml
mod toml_extract; // extract and print the version information according to the toml file

// Display a colored ASCII banner for the program
fn show_banner() {

    let banner = r#"

     ██████╗    ██████╗        ███╗    ██████╗    ███████╗   
    ██╔════╝    ██╔══██╗      ████╗    ██╔══██╗   ██╔════╝   
    ██║  ███╗   ██████╔╝     ██╔██╗    ██║  ██║   █████╗     
    ██║   ██║   ██╔══██╗    ██╔╝██╗    ██║  ██║   ██╔══╝     
    ╚██████╔╝   ██║  ██║   ███████╗    ██████╔╝   ███████╗   
     ╚═════╝    ╚═╝  ╚═╝   ╚══════╝    ╚═════╝    ╚══════╝   
    
        ███╗    ██╗        ███████╗   ██████╗    ████████╗   
       ████╗    ██║        ██╔════╝   ██╔══██╗   ╚══██╔══╝   
      ██╔██╗    ██║        █████╗     ██████╔╝      ██║      
     ██╔╝██╗    ██║        ██╔══╝     ██╔══██╗      ██║      
    ███████╗    ███████╗   ███████╗   ██║  ██║      ██║      
    ╚══════╝    ╚══════╝   ╚══════╝   ╚═╝  ╚═╝      ╚═╝  
    
    ██████╗    ███████╗   
    ██╔══██╗   ██╔════╝   
    ██████╔╝   ███████╗   
    ██╔══██╗   ╚════██║   
    ██║  ██║   ███████║   
    ╚═╝  ╚═╝   ╚══════╝ 
    
 "#;
    println!("{}", banner);
    // Print the banner in purple color
    colour_print(&banner, "blue");


    let msg = format!("\n\t grade_alert_rs\n").bright_green().bold();
    println!("{}", msg);
}

// Print colored text to the terminal, with different color options
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

// Main entry point of the program
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

    // Get the input and pairings file paths from command-line arguments
    let input_file = matches.get_one::<String>("input").unwrap();
    let pairings_file = matches.get_one::<String>("pairings").unwrap();

    // Process the input CSV file and generate markdown files
    process_csv(input_file, output_dir)?;

    // Process the pairings file to copy files to their destinations
    process_pairings(pairings_file, output_dir);

    Ok(())
}

// Parse command-line arguments using clap
fn parse_arguments() -> clap::ArgMatches {
    Command::new("Grade Alert Utility")
        .version("0.14.1")
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
                .help("Include Pairings file to determine file copy destinations"),
        )
        .get_matches()
}

// Read the input CSV file and generate markdown files for each record
fn process_csv(input_file: &str, output_dir: &str) -> Result<(), Box<dyn Error>> {
    // Open the input CSV file
    let input = File::open(input_file)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(input);

    // Read the headers from the CSV file
    let headers = rdr.headers()?.clone();
    // println!("Headers: {:?}\n", headers); //debug print

    // Iterate over each record in the CSV file
    println!("CSV Headers:");
    for header in headers.iter() {
        println!(" {}", header);
    }

    // Process each row in the CSV file
    for result in rdr.records() {
        let record = result?;

       // Generate the filename from the first column (student name or ID)
        let filename = format!(
            "{}/{}.md",
            output_dir,
            record.get(0).unwrap().trim()
                .chars()
                .map(|c| {
                    if c.is_ascii_alphabetic() {
                        c.to_ascii_lowercase()
                    } else if c.is_ascii_digit() {
                        c
                    } else {
                        '_'
                    }
                })
                .collect::<String>()
        );


// originally
    //    // Generate the filename from the first column (student name or ID)
    //     let filename = format!(
    //         "{}/{}.md",
    //         output_dir,
    //         record.get(0).unwrap().trim().replace(" ", "")
    //     );

        // Create and write to the markdown file
        let mut file = File::create(&filename)?;
        writeln!(file, "# {}\n", record.get(0).unwrap())?; // Title from the first column

        // Write each column's data with its header
        for (header, value) in headers.iter().zip(record.iter()) {
            // writeln!(file, "**{}** : {}", header, value)?; //bold markdown for header

            //            println!("{} : {}", header, value); //debug print

            if header != "" {
                //empty header check

                // print!("WRITING ||| {} : {}\n", header, value); //debug print
                writeln!(file, "{} | {}", header, value)?;
            } else {
                // print!("EMPTY HEADER SKIPPED\n"); //debug print
                writeln!(file, " ")?;
                // continue; // Skip empty headers, line not necessary at this time
            }
        }

        // Original for_loop code below. Leave for now
        // // Write each column's data with its header
        // for (header, value) in headers.iter().zip(record.iter()) {
        //     // writeln!(file, "**{}** : {}", header, value)?; //bold markdown for header
        //     writeln!(file, "{} : {}", header, value)?;

        // }
    }

    Ok(())
}

// Read the pairings file and copy generated markdown files to their destination directories
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

    // Read the pairings file (each line: source_file,destination_dir)
    let pairings_content = std::fs::read_to_string(pairings_file).expect("Missing pairings.txt?");
    for line in pairings_content.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let source_file = format!("{}/{}", output_dir, parts[0]);
            let destination_dir = parts[1];

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

            // Copy the file to the destination directory (filename is the same as in output_dir)
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

    // Write the list of unique destination directories to dirNames.txt
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
