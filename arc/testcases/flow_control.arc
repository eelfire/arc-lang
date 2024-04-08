fx flow_control() {
    let z = 1;
    let a = match z {
        1 => 1,
        2 => 2,
        _ => 3,
    };

    // this feature is now not supported
    // let b = if a == 1 {
    //     1
    // } else if a == 2 {
    //     2
    // } else {
    //     3
    // };

    let mut c = 0;
    let b = 2;
    if a > b {
        c = a;
    } else {
        c = b;
    }

    while c > 0 {
        c -= 1;
    }
}
