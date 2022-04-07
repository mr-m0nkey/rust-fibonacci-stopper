use std::io;

fn main() {
    println!("Enter a terminating number");
    let mut terminating_number = String::new();
    io::stdin().read_line(&mut terminating_number).expect("An error occurred");

    let terminating_number = terminating_number.trim().parse::<i32>().expect("Invalid number");

    let mut last_number = 0;
    let mut current_number = 1;

    loop {
        if current_number >= terminating_number {
            break;
        }

        print!("{} ", current_number);
        let temp = current_number;
        current_number = current_number + last_number;
        last_number = temp;
    }
}
