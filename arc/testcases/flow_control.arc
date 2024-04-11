fx flow_control() {
    let z = 1;
    let a = match z {
        1 => {
            let mut d = 0;
            d += 1;
            return d;
        },
        2 => {2},
        _ => {3},
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
        let d = 1;
        c += d;
    } else {
        c = b;
    }

    while c > 0 {
        c -= 1;
    }

    for i in 0:10 {
        let mut c = 0;
        e += i;
        let i = 1;
        c += i;
    }
    c = i; // i is not defined
}
