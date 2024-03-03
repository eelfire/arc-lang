fx main(){
    let multiply = fx(x, y) {
        let result = x * y;
        result
    };

    let factor = 10;
    let multiplier = fx(n) { n * factor };
    let result = multiplier(5); // result is 50

    let double = fx(n) { n * 2 };
    let negate = fx(n) { -n };
    let result = negate(double(3)); // result is -6
}
