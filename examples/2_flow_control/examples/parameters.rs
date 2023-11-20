fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    let x = plus_one(5);
    println!("x+1= {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x+1
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
} 