fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = largest_num(&number_list);

    println!("最大數字為 {}", largest);
    // assert_eq!(*largest, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let largest = largest_num(&number_list);

    println!("最大數字為 {}", largest);
}

// extracting function, so we can call the function several times without add much code.
fn largest_num(list: &Vec<i32>) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
