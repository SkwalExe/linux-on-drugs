// Author : SkwalExe
// Github : https://github.com/SkwalExe

use rand::Rng;

fn main() {
   loop {
      print!("\x1b[48;5;{}m  \x1b[0m", rand::thread_rng().gen_range(0..255));
   }
}
