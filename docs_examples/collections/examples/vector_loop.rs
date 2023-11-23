fn main() {
    let v_1 = vec![100, 32, 57];

    for i in &v_1 {
        println!("{i}");
    }

    let mut v_2 = vec![100, 32, 57];
    for i in &mut v_2 {
        *i += 50;
        println!("{i}");
    }
}
