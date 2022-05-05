fn five() -> i32 {
    let x = 5;
    return x;
}

fn main() {
    let y = {
        let h = 9;
        h
    };
    println!("The value of y is:{}", y);

    let x = five();

    println!("The value of x is: {}", x);
}
