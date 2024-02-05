let safe_divide = fx(a, b) ~ result<f64, string> {
    if b == 0 {
        return err("Division by zero");
    } else {
        return ok(a / b);
    }
};

let result = safe_divide(10, 0);
if let ok(value) = result {
    print("Result: {}", value);
} else {
    print("Error: {}", result.err());
}
