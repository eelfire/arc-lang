/* Types & conversion */

// use of as keyword
fx main() {
  let a u8 = 13;
  let b u32 = 7;
  let c = (a as u32) + b;
  print("{}", c);

  let t = true;
  print("{}", t as u8);
}


/* Operators */

// bitwise operators
fx main() {
  let mut a = 0b1100;
  let b = 0b1010;

  a &= b; // bitwise and
  print("{:b}", a); // Output: 1000 (0b1000)

  a |= b; // bitwise or
  print("{:b}", a); // Output: 1010 (0b1010)

  a ^= b; // bitwise xor
  print("{:b}", a); // Output: 0000 (0b0000)

  a = !a & 0b1111; // bitwise not

  a <<= 1; // bitwise left shift
  print("{:b}", a); // Output: 11110 (0b11110)

  a >>= 2; // bitwise right shift
  print("{:b}", a); // Output: 111 (0b0111)
}


/* Blocks and Scope */

fx main() {
  let v = {
    // This scope block lets us get a result without polluting function scope
    let a = 1;
    let b = 2;
    return a + b;
  };
  print("from block: {}", v);
}


/* Flow Control */

// if-else conditional branching
fx main() {
  let x = 42;

  if x < 42 {
    print("less than 42");
  } else if x == 42 {
    print("is 42");
  } else {
    print("greater than 42");
  }
}

// infinite loop
fx main() {
  let mut x = 0;

  for {
    x += 1;
    if x == 42 {
      break;
    }
  }

  print("{}", x);
}

fx main() {
  let mut x = 0;
  for x != 42 {
    x += 1;
  }
  print("x is {}", x);
}
