fn main() {
    let mut input = String::new();

    println!("How big is daa triii: ");

    std::io::stdin().read_line(&mut input).unwrap();

    println!("Hello, {}", input);

    let my_int: i32 = input.trim().parse().unwrap();

    tree(my_int)
}

fn tree(length: i32) {
    let stem_width = length / 4;
    let stem_height = length / 4;

    for i in 0..length {
        for _ in 0..length - i {
            print!(" ");
        }
        for _ in 0..i * 2 - 1 {
            print!("*");
        }
        println!();
    }

    //print stem

    for _ in 0..stem_height {
        for _ in 0..length - stem_width / 2 {
            print!(" ");
        }
        for _ in 0..stem_width {
            print!("*");
        }
        println!();
    }
}
