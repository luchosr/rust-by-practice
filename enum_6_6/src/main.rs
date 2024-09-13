fn main() {
    println!("Hello, world!");
}



//exercise_5:🌟🌟 Since there is no null in Rust, we have to use enum Option<T> to deal with the cases when the value is absent.

fn exercise_5:() {
    let five:Option<i32> = Some(5);
    let six:Option<i32> = plus_one(five);
    let none:Option<i32> = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);

        println!("Success!");
    }else{
        panic!("NEVER LET THIS RUN！");
    } 
        
    
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}