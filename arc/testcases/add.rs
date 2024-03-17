fn add(a: i32, b: i32) -> i32 {
    let c = a + b;
    return c;
}

struct A {
    a: i32,
    b: i32,
}

struct B {
    c: f32,
    d: A
}

fn main() {
    let a = 4;
    let b = 5;
    let c: i32 = add(a, b);
    print("{}", c);
    let ch: char = ' ';
}
