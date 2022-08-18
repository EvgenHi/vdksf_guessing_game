//! To start using this library add this code under `[dependencies]`:
//! ```
//! vdksf_guessing_game = "0.1.*"
//! ```
//! After this use this code to start game:
//! ```
//! use vdksf_guessing_game::start_game;
//! 
//! fn main() {
//!     start_game();
//! }
//! ```

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

pub fn start_game() {
    println!("\nIt's a guessing game! Write your guess until it matches the generated random number in the range 1..100");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut input = String::new();

    loop {
        print!("\nPlease input your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number. Try again.\n"); 
                continue
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("\n! ! ! CONGRATULATIONS ! ! ! YOU GUESSED THE NUMBER ! ! !");
                print!("\nWant to play again? (Yes/Sure) ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                if input.trim().to_lowercase() == "donut".to_string() {
                    donut()
                }    
            }
        }
    }
}

// THIS IS NOT MY CODE. This donut does not belong to me. I don't know where it came from. It's just an easter egg.
fn donut() {
    let mut a = 0.;
	let mut b = a;
	print!("\x1b[2J");
	loop {
		let (mut r, mut z, mut j, sc) = ([' '; 1760], [0.; 1760], 0., f32::sin_cos);
		while j < 6.28 {
			let mut i = 0.;
			while i < 6.28 {
				let ((c, l), (f, d), (e, g), (n, m)) = (sc(i), sc(j), sc(a), sc(b));
				let h = d + 2.;
				let (p, t) = (1. / (c * h * e + f * g + 5.), c * h * g - f * e);
				let (x, y) = ((40. + 30. * p * (l * h * m - t * n)) as usize, (12. + 15. * p * (l * h * n + t * m)) as usize);
				let o = x + 80 * y;
				let q = (8. * ((f * e - c * d * g) * m - c * d * e - f * g - l * d * n)) as usize;
				if 22 > y && y > 0 && x > 0 && 80 > x && p > z[o] {
					z[o] = p;
					r[o] = b".,-~:;=!*#$@"[q.max(0)] as char;
				}
				i += 0.02;
			}
			j += 0.07;
		}
		print!("\x1b[H");
		for k in 0..1761 {
			print!("{}", if k % 80 != 0 {r[k]} else {'\n'});
			a += 4e-5;
			b += 2e-5;
		}
		std::thread::sleep(std::time::Duration::from_millis(21));
	}
}
