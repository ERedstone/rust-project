use std::io;
fn main() {
    let mut input = String::new();
    match io::stdin()
            .read_line(&mut input) {
        Ok(..) => (),
        Err(err) => println!("The following error occurred: {:?}", err),
    }

    let i = input
        .trim()
        .parse()
        .expect("{}");

        
    let mut count = 0;
    loop {
        count += 1;
        println!("{}", count);
        if count == i {
            println!("{}", count);
            break;
        }
    }
}