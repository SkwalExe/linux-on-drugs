// Author : SkwalExe
// Github : https://github.com/SkwalExe

use rand::Rng;
use std::process;

const RED: &str = "\x1b[91m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const BLUE: &str = "\x1b[94m";
const MAGENTA: &str = "\x1b[95m";
const CYAN: &str = "\x1b[96m";
const WHITE: &str = "\x1b[97m";
const RESET: &str = "\x1b[0m";
const BG_RED: &str = "\x1b[41m";
const BG_MAGENTA: &str = "\x1b[45m";

// this function is used to print a text with random color for each char
fn drug_print(text: &str) {
   // random color for each letter
   let mut color_text = String::new();
   for c in text.chars() {
      let color = match rand::thread_rng().gen_range(0..6) {
         0 => RED,
         1 => GREEN,
         2 => YELLOW,
         3 => BLUE,
         4 => MAGENTA,
         5 => CYAN,
         _ => RESET,
      };
      color_text.push_str(color);
      color_text.push(c);
      color_text.push_str(RESET);
   }
   println!("{}", color_text);
}

fn main() {
   let mut block_content = String::from(" "); // the character inside each block

   let mut block_size: usize = 2; // the length of each block, the block_content will be repeated this number of times
   let mut command = "drugs"; // command to execute (drugs, help, version)

   let mut args: Vec<String> = std::env::args().collect(); // command line arguments
   args.remove(0); // remove the name of the program

   while args.len() > 0 {
      match args[0].as_str() {
         "-c" | "--content" => {
            if args.len() > 1 {
               // if no argument is provided after the flag
               // print an error message and exit
               println!(
                  "{}[ x ] : Error: Argument needed after -c/--content{}",
                  RED, RESET
               );
               process::exit(1);
            }

            args.remove(0);
            let temp = args.remove(0);

            if temp.len() == 1 {
               // if the argument is 1 char
               block_content = temp;
            } else {
               // if the argument is more than 1 char
               // print an error message and exit
               println!(
                  "{}[ x ] : Error: you must specify only one character after -c/--content{}",
                  RED, RESET
               );
               process::exit(1);
            }
         }
         "-b" | "--block-size" => {
            if args.len() > 1 {
               println!(
                  "{}[ x ] : Error: Argument needed after -b/--block-size{}",
                  RED, RESET
               );
               process::exit(1);
            }

            // parse number from argument
            block_size = match args[1].parse::<usize>() {
               Ok(n) => n,
               Err(_) => {
                  println!(
                     "{}[ x ] : Error: Argument after -b/--block-size must be a number{}",
                     RED, RESET
                  );
                  process::exit(1);
               }
            };

            if block_size < 1 || block_size > 2000 {
               // block size must be between 1 and 2000
               println!(
                  "{}[ x ] : Error: Argument after -b/--block-size must be between 1 and 2000{}",
                  RED, RESET
               );
               process::exit(1);
            }

            args.remove(0);
            args.remove(0);
         }
         "--version" | "-v" => {
            command = "version";
            args.remove(0);
         }
         "--help" | "-h" => {
            command = "help";
            args.remove(0);
         }
         _ => {
            println!(
               "{}Invalid argument: {}{} {} {}",
               RED, WHITE, BG_RED, args[0], RESET
            );
            process::exit(1);
         }
      }
   }
   match command {
      "version" => println!(
         "{}Linux on drugs, by Skwal => {}{}{}",
         MAGENTA,
         RED,
         env!("CARGO_PKG_VERSION"),
         RESET
      ),
      "help" => {
         println!("{}{} Linux on drugs {}", BG_MAGENTA, WHITE, RESET);
         println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
         println!("Author: {}SkwalExe{}", MAGENTA, RESET);
         println!("Github: {}https://github.com/SkwalExe{}", MAGENTA, RESET);
         println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
         drug_print("Gives drugs to your computer.");
         println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
         println!("Options : ");
         println!(
            "\t{}--version, -v: {}Prints the version of the program{}",
            MAGENTA, YELLOW, RESET
         );
         println!(
            "\t{}--help, -h: {}Prints this help message{}",
            MAGENTA, YELLOW, RESET
         );
         println!(
            "\t{}--block-size, -b: {}Sets the size of each color block{}",
            MAGENTA, YELLOW, RESET
         );
         println!(
            "\t{}--content, -c: {}Sets the content of each color block{}",
            MAGENTA, YELLOW, RESET
         );
         println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
      }

      "drugs" => {
         // block is equal to block_content repeated block_size times
         let block = block_content.repeat(block_size);
         loop {
            // print the block with a random color in the background
            print!(
               "\x1b[48;5;{}m{}\x1b[0m",
               rand::thread_rng().gen_range(0..255),
               block
            );
         }
      }

      _ => {}
   }
}
