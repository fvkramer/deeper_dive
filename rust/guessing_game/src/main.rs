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

// fn another_function(x: i32) {
//   println!("this is another {} function", x);
// }

// fn five() -> i32 {
//   5
// }

// fn main() {
  // let mut x: u8 = 280;
  // let guess: u32 = "42".parse().expect("Not a number!");
  // println!("The variable x is {}", guess);

  // let tup = (500, 6.4, 1);

  // let (x, y, z) = tup;

  // println!("The value of x, y, z is {}, {}, {}", x, y, z);

  // another_function(5);
  // let x = (5 + 5);
//   let x = five();

//   println!("The value of x is: {}", x);
// }

//arrays in rust have fixed length and all elements must be same type
//tuples fixed length and can have different element types

// fn main() {
//   for number in (1..4).rev() {
//     println!("{}!", number);
//   }

//   println!("LIFTOFF!");
// }

// fn main() {
//   let mut s = String::from("hello");
//   // let mut s = "hello";

//   s.push_str(", world");

//   println!("{}", s);
// }

fn main() {
  let s1 = String::from("hello");

  let len = calculate_length(&s1);
  // let len = &s1.len();

  println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}