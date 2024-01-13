fn main() {
    print_hollow_world();
}

fn print_hollow_world() {
    let width = 15;
    let height = 7;

    for i in 0..height {
        for j in 0..width {
            if i == 0 || i == height - 1 || j == 0 || j == width - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
