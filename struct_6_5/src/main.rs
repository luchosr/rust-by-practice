fn main() {
    println!("Hello, world!");
}


//exercise_1:🌟 We must specify concrete values for each of the fields in struct.

// Fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn exercise_1() {
    let age:u8 = 30;
    let p: Person = Person {
        name: String::from("sunface"),
        age,
        hobby:String::from("rust coding")
    };

    println!("Success!");
} 

// exercise_2:🌟 Unit struct don't have any fields. It can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

// *************WIP*************

struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

// We don't care about what fields  are  in the Unit, but we care about its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit {  }
fn main() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
} 

// Fill the blank to make the code work
fn do_something_with_unit(u: __) {   }

//exercise_3:🌟🌟🌟 Tuple struct looks similar to tuples, it has added meaning the struct name provides but has no named fields. It's useful when you want to give the whole tuple a name, but don't care about the fields's names.

// Fix the error and fill the blanks
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn exercise_3() {
    let v: Point = Point(0, 127, 255);
    check_color(v);

    println!("Success!");
}   

fn check_color(p: Point) {
    let Point (x,_,z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
 }

 //exercise_4:🌟 You can make a whole struct mutable when instantiating it, but Rust doesn't allow us to mark only certain fields as mutable.

// Fill the blank and fix the error without adding/removing new line
struct Person {
    name: String,
    age: u8,
}
fn exercise_4() {
    let age: u8 = 18;
    let mut p: Person = Person {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18? 
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success!");
}

//exercise_5:🌟 Using field init shorthand syntax to reduce repetitions.
// Fill the blank
struct Person {
    name: String,
    age: u8,
}
fn exercise_5() {
    println!("Success!");
} 

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name,
    }
}

//exercise_6:🌟 You can create instance from other instance with struct update syntax


// Fill the blank to make the code work
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn exercise_6() {
    let u1: User = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2: User = set_email(u1);

    println!("Success!");
} 

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}

//exercise_7:🌟🌟 We can use #[derive(Debug)] to make a struct printable.
// Fill the blanks to make the code work
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn exercise_7() {
    let scale: u32 = 2;
    let rect1: Rectangle = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("{:?}", rect1); // Print debug info to stdout
}

//exercise_8: 
// Fix errors to make it work
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn exercise_8() {
    let f: File = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name: String = f.name;

    // ONLY modify this line
    println!("{}, {}",_name, f.data);
} 🌟🌟