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

  while true {
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
