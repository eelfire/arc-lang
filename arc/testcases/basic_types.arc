fx main() {
    let x = 12; // by default this is i32
    let a = 12 as u8;
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
