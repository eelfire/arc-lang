// arrays & lists
fx main() {
  let arr [u32; 3] = [1, 2, 3];
  let a_list <u32> = <1, 2, 3>;
  print(arr.len());
  a_list.push(4);

  // tuples
  let tup (i32, u64, u32) = (-5, 67, 13);

  let a = 4;
  let b = 5;
  let c bool = a < b;
}
