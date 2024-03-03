// adsg
/* aksdhgffl 

*/

fx checking_precedence(a i32, b f64) ~ (i32, [f64; 4]) {
    let mut a = 2 + 3 * 4 - 3;
a += b as i32;
    return a;
}

fx flow_control() {
    if a < b {
        a = b;
    }
}

fx add(a i32, b i32) ~ i32 {
    let c = a + b;
    return c;
}

fx main() {
    let a = 4;
    let b = 5;
    let c i32 = add(a + 2, b);
    print("{}", c);
    let ch char = ' ';
}

/* Types & conversion */

// support for unicode characters
fx main() {
    let x = 12; // by default this is i32
    let a = 12 as i32;
    let b = 4.3; // by default this is f64
    let c = b as f32;
    let d = 'r'; // unicode character
    let emoji = '😀'; // also a unicode character
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    print(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        x, a, b, c, d, emoji, bv, t.0, t.1, sentence
    );
}

// use of as keyword
fx main() {
  let a i32 = 13;
  let b i32 = 7;
  let c = (a as i32) + b;
  print("{}", c);

  let t = true;
  print("{}", t as i32);
}

/* This is a multiline comment test.
The functions in this program print 
synonyms of 'BYE' */

fx Bye_m8() {
    // print("Bye.\nAdieu.\nAdieu.\nArrivederci.\nBye-bye.\nCheerio.\nGood day.\nSayonara.\nSo long."); 
    
// above line has an error and the text is not being read

    print("Bye.
Adieu.
Adieu.
Arrivederci.
Bye-bye.
Cheerio.
Good day.
Sayonara.
So long.");
}

// Done

// arrays & lists
fx main() {
  let arr [[i32; 5]; 3] = [1, 2, 3];
  let a_list <i32> = <1, 2, 3>;
  print(arr.len());
  a_list.push(4);

  // tuples
  let empty_tup () = ();
  let tup (i32, i64, f32) = ( 1-5, 67, 12.-13., b - a);


  let a = 4;
  let b = 5;
  let c bool = a < b;
}