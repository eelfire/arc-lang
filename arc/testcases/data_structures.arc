// arrays & lists
fx main() {
  let arr [i32; 3] = [1, 2, 3];
  let a_list <i32> = <1, 2, 3>;
  print(arr.len());
  a_list.push(4);

  // tuples
  let tup (i32, i64, f32) = (-5, 67, 13.);

  let a = 4;
  let b = 5;
  let c bool = a < b;
}