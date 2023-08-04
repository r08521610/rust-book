fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is: {}", x);

    let spaces = "   ";
    println!("spaces len is: {}", spaces_len(spaces));

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn spaces_len(spaces: &str) -> usize {
    let spaces = spaces.len();
    return spaces;
}
