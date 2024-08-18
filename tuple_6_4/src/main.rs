fn main() {
    println!("Hello, world!");
}
fn exercise_1() {
    let _t0: (u8,i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}

fn exercise_2() {
    let t: (&str,&str, &str) = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");

    println!("Success!");
}

fn exercise_3(){
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}

fn exercise_4(){
    let tup:(i32,f64, &str) = (1, 6.4, "hello");

    // Fill the blank to make the code work
    let (x,z,y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}
