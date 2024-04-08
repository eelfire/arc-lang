// arrays & lists
fx main() {
  let arr [i32; 3] = [1, 2, 3];
  let a_list <i32> = <1, 2, 3>;
  print(arr.len());
  a_list.push(4);

  // tuples
  let tup (i32, i64, f32) = (-5, 67, 13.);
  let (x, y, z) = tup;

  let a = 4;
  let b = 5;
  let c bool = a < b;

  let arr_access = arr[0];
  let arr_slice = arr[0:2];
  
  let list_access = a_list[0];
  let list_slice = a_list[0:2];

  let tup_access = tup.0;
}