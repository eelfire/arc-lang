fx main(){
    let multiply = fx(x, y) {
        let output = x * y;
        output
    };

    let factor = 10;
    let multiplier = fx(n) { n * factor };
    let output = multiplier(5); // output is 50

    let double = fx(n) { n * 2 };
    let negate = fx(n) { -n };
    let output = negate(double(3)); // output is -6
}
