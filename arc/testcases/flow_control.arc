fx flow_control() {
    let a = match a {
        1 => 1,
        2 => 2,
        _ => 3,
    };

    let b = if a == 1 {
        1
    } else if a == 2 {
        2
    } else {
        3
    };

    let mut c = 0;
    if a > b {
        c = a;
    } else {
        c = b;
    }
}
