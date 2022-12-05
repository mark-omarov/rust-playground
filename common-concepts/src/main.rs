const HARDCODED_CONSTANT: u32 = 100;

fn main() {
  /* Variables */
  let mut x = 5;
  println!("The value of x is: {x}");
  x = 6;
  println!("The value of x is: {x}");

  // this was unexpected to me, the value will actually be 1000,
  // even though it might seem like it should be 100.
  // I suppose because Rust resolves constants at compile time,
  // but still allows shadowing - this is the result we get.
  println!("The value of HARDCODED_CONSTANT is: {HARDCODED_CONSTANT}");
  const HARDCODED_CONSTANT: u32 = 1_000;
  println!("The value of HARDCODED_CONSTANT is: {HARDCODED_CONSTANT}");

  // This is also interesting, the value of the "main" scope will be "1000",
  // but this inner scope has value of the shadowed constant set to 1000000.
  // I believe if it be printed in the "global" (idk if it's called
  // this in Rust) the value will be 100. Not sure how to feel about it yet.
  // Does this also means that shadowing allocates memory for each one of them?
  // Otherwise, we would modify the original value...
  {
    const HARDCODED_CONSTANT: u32 = 1_000_000;
    println!("The value of HARDCODED_CONSTANT is: {HARDCODED_CONSTANT}");
  }
  // It seems so, because as soon as we exit the previous scope we're back
  // to having the value pointed to 1000.
  println!("The value of HARDCODED_CONSTANT is: {HARDCODED_CONSTANT}");
  // It was actually explained in the book, if only I finished reading before playing. :D

  /* Data Types */
  let tup: (u8, i32, f64) = (1, 100, 100.1); // Bounds all values to the tuple named tup
  // values can be accessed with destructure, or dot (.) notation using value's index
  let (x, y, z) = tup;
  println!("Destructured values are {x}, {y}, {z}");
  // println!("Dot-notation values are {tup.0} {tup.1} {tup.2}"); // this didn't work
  // not sure why, probably this format is now allowed, maybe more info on it later in the book
  let a = tup.0;
  let b = tup.1;
  let c = tup.2;
  println!("Dot-notation values are {a} {b} {c}");

  // Difference between tuples and arrays is that every element of an array must be of the same type.
  // Arrays are size-fixed, unlike JS, vectors aren't though.
  // Array's type annotation: [i32, 5], where i32 is the type of elements, and 5 is the size.
  // For the same values repeated over n times: let a = [3,5] -> [3, 3, 3, 3, 3]
}
