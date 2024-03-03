fx main(){
    let safe_divide = fx(a, b) ~ result<f64, string> {
        if b == 0 {
            return err("Division by zero");
        } else {
            return ok(a / b);
        }
    };

    let res = safe_divide(10, 0);
    if let ok(value) = res {
        print("Result: {}", value);
    } else {
        print("Error: {}", res.err());
    }
}
