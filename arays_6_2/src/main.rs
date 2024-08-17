fn main() {
    exercise_5();
}


fn exercise_1(){


        // Fill the blank with proper array type
        // let arr: [i32;5] = [1, 2, 3, 4, 5];

        // Modify the code below to make it work
        // assert!(arr.len() == 5);
        // println!("Success!");

        let arr: [i32;5] = [1, 2, 3, 4, 5];
        assert!(arr.len() == 5);
    
        println!("exercise_1 Success!");
}


fn exercise_2(){
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    
    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);

    println!("exercise_2 Success!");
}


fn exercise_3() {

    //All elements in an array can be initialized to the same value at once.
    // Fill the blank
    let list: [i32; 100] = [1;100] ;

    assert!(list[0] == 1);
    assert!(list.len() == 100);


    println!("exercise_3 Success!");
}

fn exercise_4() {
    // Fix the error
    let _arr = [1, 2, 3];

    println!("excercise_4 Success!");
}

fn exercise_5() {
    let arr = ['a', 'b', 'c'];
    
    let ele = arr[0]; // Only modify this line to make the code work!

    assert!(ele == 'a');

    println!("excercise_5 Success!");
}


// Fix the error
fn excercise_6() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // But indexing is not safe
    let _name1 = &names[1];

    println!("Success!");
}