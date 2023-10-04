use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let args: Vec<_> = args[1..].to_vec();

    let mut sum = 0;
    for arg in args {
        match arg.parse::<i32>() {
            Ok(num) => sum += num,
            Err(_) => continue,
        }
    }

    println!("{}", sum);
}
