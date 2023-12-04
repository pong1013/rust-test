#[derive(Debug)]
struct Rectangle {
    name: char,
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            name: 'a',
            width: 10,
            height: 1,
        },
        Rectangle {
            name: 'b',
            width: 3,
            height: 5,
        },
        Rectangle {
            name: 'c',
            width: 7,
            height: 12,
        },
    ];

    let mut operations = 0;
    list.sort_by_key(|r| {
        operations += 1;
        r.width
    });
    println!("Sort by width: {:#?}, operations: {}", list, operations);

    let mut operation_times = 0;
    list.sort_by_key(|r| r.height);
    println!("Sort by height: {:#?}", list);
}
