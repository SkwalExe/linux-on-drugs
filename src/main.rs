// Author : SkwalExe
// Github : https://github.com/SkwalExe

#![allow(dead_code)]

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
const BG_GREEN: &str = "\x1b[42m";
const BG_YELLOW: &str = "\x1b[43m";
const BG_BLUE: &str = "\x1b[44m";
const BG_MAGENTA: &str = "\x1b[45m";
const BG_CYAN: &str = "\x1b[46m";

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
   let block_content = String::from(" "); // I will add the possibility to change this later

   let mut block_size: usize = 2;
   let mut command = "drugs";

   let mut args: Vec<String> = std::env::args().collect();
   args.remove(0);

   while args.len() > 0 {
      match args[0].as_str() {
         "-b" | "--block-size" => {
            if args.len() < 2 {
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
         println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
      }

      "drugs" => {
         let block = block_content.repeat(block_size);
         loop {
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
