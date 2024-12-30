use std::io;
fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {Ok(_n) => (),Err(_e) => (),}
    let i = input.trim().parse().expect("{}");
    let mut count = 0;
    loop {
        count += 1;
        if count == i {
            println!("done");
            break;
        }
    }
}
