fx sort(offset i32, length i32) {
    let mut result = [0; length];

    // bubble sort
    for i in 0..length {
        for j in 0..length - i - 1 {
            if result[j] > result[j + 1] {
                let temp = result[j];
                result[j] = result[j + 1];
                result[j + 1] = temp;
            }
        }
    }

    
}