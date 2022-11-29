use std::io::stdin;
use regex::Regex;

#[derive(PartialEq)]
enum Temperatures {
  Celsius,
  Fahrenheit,
}

fn get_temperature_type(str: &str) -> Result<Temperatures, &str> {
  if str.to_lowercase() == "c" {
    return Ok(Temperatures::Celsius);
  }

  if str.to_lowercase() == "f" {
    return Ok(Temperatures::Fahrenheit);
  }

  Err("Not Found")
}

fn convert_temperatures(temperature: i32, from: Temperatures, to: Temperatures) -> Result<i32, String> {
  if from == to {
    return Ok(temperature);
  }

  if from == Temperatures::Fahrenheit && to == Temperatures::Celsius {
    return Ok((temperature - 32) * 5/9);
  }

  if from == Temperatures::Celsius && to == Temperatures::Fahrenheit {
    return Ok((temperature * 9/5) + 32);
  }

  Err("Not found".to_string())
}

fn main() {
  println!("Temperatures Convertor CLI");
  println!("Please type temperatures to convert. Example: 10C to F");

  let mut user_input = String::new();
  stdin().read_line(&mut user_input).expect("Input failed");

  let re = Regex::new(r"(\d+)([cCfF])\s+to\s+([cCfF])").unwrap();

  let (parsed_temp, parsed_from, parsed_to) = match re.captures(&*user_input) {
    None => (0, "", ""),
    Some(capt) => {
      let temperature = match capt.get(1) {
        None => 0,
        Some(m) => m.as_str().parse().expect("Not a number")
      };

      let from = match capt.get(2) {
        None => "",
        Some(m) => m.as_str()
      };

      let to = match capt.get(3) {
        None => "",
        Some(m) => m.as_str()
      };

      (temperature, from, to)
    }
  };

  let from = match get_temperature_type(parsed_from) {
    Ok(from) => from,
    Err(_) => {panic!("From type invalid")}
  };

  let to = match get_temperature_type(parsed_to) {
    Ok(to) => to,
    Err(_) => {panic!("To type invalid")}
  };


  match convert_temperatures(parsed_temp, from, to) {
    Ok(res) => println!(
      "Result: {}{} => {}{}",
      parsed_temp,
      parsed_from.to_uppercase(),
      res,
      parsed_to.to_uppercase(),
    ),
    Err(message) => println!("{}", message),
  }
}

// [Shadowing]

// fn main() {
//   let mut x = 5;
//   println!("{}", x);
//   x += 1;
//
//   let x = x - 1;
//
//   x += 1;
// }

// [Guessing Game]

// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;
// use std::num::ParseIntError;
// use futures::executor::block_on;
//
// async fn get_user_guess() -> Result<u32, ParseIntError> {
//   let mut guess = String::new();
//
//   io::stdin()
//     .read_line(&mut guess)
//     .expect("Failed to read line");
//
//   return guess.trim().parse();
// }
//
// fn main() {
//   let secret_number = rand::thread_rng().gen_range(1..101);
//   let mut attempts: u32 = 10;
//   println!("Guess the number!");
//
//   while attempts > 0 {
//     println!("Attempts left: {}", attempts);
//     println!("Please input your guess.");
//     let guess: u32 = match block_on(get_user_guess()) {
//       Ok(num) => num,
//       Err(_) => continue,
//     };
//     println!("You guessed: {}", guess);
//
//     match guess.cmp(&secret_number) {
//       Ordering::Less => {
//         println!("Too small!");
//         attempts -= 1;
//       },
//       Ordering::Equal => {
//         println!("You win!");
//         break;
//       },
//       Ordering::Greater => {
//         println!("Too big!");
//         attempts -= 1;
//       },
//     }
//
//     if attempts == 0 {
//       println!("You ran out of attempts, looser!");
//     }
//   }
// }
