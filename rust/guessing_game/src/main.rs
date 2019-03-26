// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

// const MAX_POINTS: u32 = 100_000;

// fn main() {
//   println!("Guess the number");

//   let secret_number = rand::thread_rng().gen_range(1, 101);

//   println!("The secret number is {}", secret_number);
  
//   println!("The constant is {}", MAX_POINTS);
//   loop {
//     println!("Input your guess.");

//     let mut guess = String::new();

//     io::stdin().read_line(&mut guess)
//       .expect("Failed to read line");

//     let guess: u32 = match guess.trim().parse() {
//       Ok(num) => num,
//       Err(_) => {
//         println!("");
//         println!("Please only enter numbers");
//         continue; 
//       }
//     };

//     println!("You guessed: {}", guess);


//     match guess.cmp(&secret_number) {
//       Ordering::Less => println!("Too small!"),
//       Ordering::Greater => println!("Too big!"),
//       Ordering::Equal => {
//         println!("You win!");
//         break;
//       }
//     }
//   }
// }
//  signed numbers stored with twos complement

fn main() {
  // let mut x: u8 = 280;
  let guess: u32 = "42".parse().expect("Not a number!");
  println!("The variable x is {}", guess);

  let tup = (500, 6.4, 1);

  let (x, y, z) = tup;

  println!("The value of y is {}", y);
}

//arrays in rust have fixed length and all elements must be same type
//tuples fixed length and can have different element types
