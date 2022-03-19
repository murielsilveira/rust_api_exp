#![allow(dead_code)]

use rand::Rng;
use std::cmp::Ordering;
use std::fmt;
use std::io;

fn main() {
  // guessing_game();
  // print_some_fibonacci_numbers();
  // convert_temperatures();
  // play_with_string_slices();
  printing_structs();
}

fn guessing_game() {
  println!("This is a guessing game, you need to guess a number from 1 to 10!");

  let secret = rand::thread_rng().gen_range(1..11);
  println!("The secret number is {} 🤫", secret);

  'game: loop {
    println!("Type a natural number and press enter:");

    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
      Ok(number) => number,
      Err(_) => continue 'game,
    };

    match guess.cmp(&secret) {
      Ordering::Less => println!("To small"),
      Ordering::Greater => println!("Too big"),
      Ordering::Equal => {
        println!("Nice guess! 🎊");
        break 'game;
      },
    }
  }
}

fn print_some_fibonacci_numbers() {
  for n in 0..9 {
    print!("{} ", nth_fibonacci(n));
  }
  println!();
}

fn nth_fibonacci(n: u32) -> u32 {
  if n == 0 {
    return 0;
  }
  if n <= 2 {
    return 1;
  }
  return nth_fibonacci(n - 1) + nth_fibonacci(n - 2);
}

fn convert_temperatures() {
  let celsius = 0.0;
  let fahrenheit = 32.0;
  println!(
    "{:.1}°C = {:.1}°F",
    celsius,
    convert_celsius_to_fahrenheit(celsius)
  );
  println!(
    "{:.1}°F = {:.1}°C",
    fahrenheit,
    convert_fahrenheit_to_celsius(fahrenheit)
  );
}

fn convert_celsius_to_fahrenheit(celsius: f32) -> f32 {
  celsius * 1.8 + 32.0
}

fn convert_fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
  (fahrenheit - 32.0) / 1.8
}

fn play_with_string_slices() {
  let a = String::from("abc🤮 efg");

  println!("{}", first_word(&a));

  for (i, c) in a.as_bytes().iter().enumerate() {
    println!("{} - {}", i, c);
  }

  // Taking a string slice in the middle of a utf-8
  // character compiles but fails at runtime.
  // let b = &a[..5];
  // println!("{}", b);
}

// also valid, but less flexible and idiomatic
// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
  for (i, &item) in s.as_bytes().iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}

fn printing_structs() {
  let t = Temperature {
    value: 10.0,
    scale: 'F',
  };
  let r1 = Rectangle {
    width: 50,
    height: 40,
  };
  let r2 = Rectangle::square(50);

  println!("Debug print: {:?}", t);
  println!("Display print: {}", t);
  println!("Pretty debug print: {:#?}", t);

  dbg!(&r1);
  let _ = dbg!(r1.area());

  println!("{} can hold {}: {}", r1, r1, r1.can_hold(&r1));
  println!("{} can hold {}: {}", r1, r2, r1.can_hold(&r2));
}

#[derive(Debug)]
struct Temperature {
  value: f32,
  scale: char,
}

// Implement the fmt::Display trait, so we can use `{}` marker on print functions.
impl fmt::Display for Temperature {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}°{}", self.value, self.scale)
  }
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl fmt::Display for Rectangle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Rect({}, {})", self.width, self.height)
  }
}

impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }

  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, r: &Rectangle) -> bool {
    self.width >= r.width && self.height >= r.height
  }
}
