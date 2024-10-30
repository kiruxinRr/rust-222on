
#[test]
fn test1(){
    struct Person {
        name: String,
        age: u8,
        hobby: String
    }
    fn main() {
        let age = 30;
        let p = Person {
            name: String::from("sunface"),
            age,
            hobby: String::from("coding"),
        };

        println!("Success!");
    }

}

#[test]
fn test2(){
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
    fn do_something_with_unit(u: Unit) {   }
}

#[test]
fn test3(){
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    fn main() {
        let v: Point = Point(0, 127, 255);
        check_color(v);

        println!("Success!");
    }

    fn check_color(p: Point) {
        let Point (x, _, z) = p;
        assert_eq!(x, 0);
        assert_eq!(p.1, 127);
        assert_eq!(z, 255);
    }
}

#[test]
fn test4(){
    struct Person {
        name: String,
        age: u8,
    }
    fn main() {
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
}

#[test]
fn test5(){
    // Fill the blank
    struct Person {
        name: String,
        age: u8,
    }
    fn main() {
        println!("Success!");
    }

    fn build_person(name: String, age: u8) -> Person {
        Person {
            age,
            name: name,
        }
    }
}

#[test]
fn test6(){
    // Fill the blank to make the code work
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    fn main() {
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
}

#[test]
fn test7(){
    // Fill the blank to make the code work
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    fn main() {
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
}

#[test]
fn test8(){
    // Fix errors to make it work
    #[derive(Debug)]
    struct File {
        name: String,
        data: String,
    }
    fn main() {
        let f: File = File {
            name: String::from("readme.md"),
            data: "Rust By Practice".to_string()
        };

        let _name = f.name;

        // ONLY modify this line
        println!("{}, {}",_name, f.data);
    }
}