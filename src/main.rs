use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;

fn parse_markdown_file( filename: &str) {
  print_short_banner();
  println!("[ INFO ] Starting parser!");
  
  // Create a path variable from the filename
  let input_filename = Path::new( filename);

  // Try to open the file
  let file = File::open(&input_filename).expect("[ ERROR ] Failed to open file!");

  // Create a place to store all our tokens
  let mut tokens: Vec<String> = Vec::new();

  // Read the file line-by-line
  let reader = BufReader::new(file);

  let mut ptag: bool = false; // keep track of paragraph enclosures
  let mut htag: bool = false;

  for line in reader.lines() {

    let line_contents = line.unwrap();
    
    let mut first_char: Vec<char> = line_contents.chars().take(1).collect();

    // Now check the first character to for headings
    let mut s = String::new();
    let slice = &line_contents.to_string();
    match first_char.pop() {
      Some('#') => {
        if ptag {
          ptag = false;
          s.push_str("</p>\n"); // adding \n for instructional clarity
        } 
        if htag {
          htag = false;
          s.push_str("</h1>\n"); // close it if we're already open
        } else {
            htag = true;
            s.push_str("<h1>");
            s.push_str(&slice[2..]); // Get all but the first two characters
        }
      },
      
      _ => {
        if htag {
          htag = false;
          s.push_str("</h1>\n");
        }
        
        if !ptag {
          ptag = true;
          s.push_str("<p>");
        } 

        s.push_str(&slice);
        
      }
    };

    // At the very end, check if any of the tag bools are still open. If so,
    // close them.
    if htag {
      htag = false;
      s.push_str("</h1>\n");      
    }

    if ptag {
      ptag = false;
      s.push_str("</p>\n");
    }

    // Don't push blank lines
    if s != "<p></p>\n" {
      tokens.push(s);
    }

  }
  
  // Create an output file based on the input file, minus ".md"
  let output_filename = & filename[.. filename.len()-3];
  let mut output_filename = String::from(output_filename);
  output_filename.push_str(".html");

  let mut outfile = File::create(output_filename.to_string())
    .expect("[ ERROR ] Could not create output file!");
  
  for line in &tokens {
    outfile.write_all(line.as_bytes())
      .expect("[ ERROR ] Could not write to output file!");
  }
}

fn get_title() -> String {
  let mut the_title = String::from(env!("CARGO_PKG_NAME"));
  the_title.push_str(" (v");
  the_title.push_str(env!("CARGO_PKG_VERSION"));
  the_title.push_str("), ");
  the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
  return the_title;
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
  println!("Written by: {}\nHomepage: {}\nUsage: tinymd <somefile>.md\n", 
    env!("CARGO_PKG_AUTHORS"), 
    env!("CARGO_PKG_HOMEPAGE")
  );

}

fn usage() {
    print_long_banner();
    
}

fn main() {

    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
          println!("[ ERROR ] No input file specified!");
          usage();
        }
    }
}