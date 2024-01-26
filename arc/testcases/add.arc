fx add(a i32, b i32) ~ i32 {
    let c = a + b;
    return c;
}

fx main() {
    let a = 4;
    let b = 5;
    let c = add(a, b);
    print("{}", c);
}