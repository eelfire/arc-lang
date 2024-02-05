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
