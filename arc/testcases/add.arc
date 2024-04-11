pub fx add(a i32, b i32) ~ i32 {
    let c = a + b;
    return c;
}

fx main() {
    let mut a = 4;
    a = 3;
    let b = 5;
    let c i32 = add(a, b);
    print("{}", c);
    let ch char = ' ';
}