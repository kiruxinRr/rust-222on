//6-6.1//
#[test]
fn test1() {
    let s: &str = "hello, world";
}

#[test]
fn test2() {
    let s: Box<str> = "hello, world".into();
    greetings_str(&s)
}

fn greetings_str(s: &str) {
    println!("{}", s)
}


#[test]
fn test3() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
}

#[test]
fn test4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s)
}

#[test]
fn test5() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats")
}

#[test]
fn test6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}

#[test]
fn test7() {
    let s = "hello, world".to_string();
    greetings_string(s)
}

fn greetings_string(s: String) {
    println!("{}", s)
}

#[test]
fn test8() {
    let s = "hello, world".to_string();
    let s1: &str = &s;
}

#[test]
fn test9() {
    // You can use escapes to write bytes by their hexadecimal values
    // fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

#[test]
fn test10() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    // modify above line to make it work
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let  delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fill the blank
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"")
}
#[test]
fn test11() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1];
    assert_eq!(h, "h");

    let h1 = &s1[3..6];
    assert_eq!(h1, "中");
}

#[test]
fn test12() {
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}




//6-6.2//
#[test]
fn test13_1() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    assert!(arr.len() == 5);
}

#[test]
fn test13_2() {
    // we can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    // Arrays are stack allocated, `std::mem::size_of_val` return the bytes which array occupies
    // A char takes 4 byte in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);
}

#[test]
fn test13_3() {
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);
    assert!(list.len() == 100);
}

#[test]
fn test13_4() {
    // fix the error
    let _arr = [1, 2, 3];
}

#[test]
fn test13_5() {
    let arr = ['a', 'b', 'c'];

    let ele = arr[0];

    assert!(ele == 'a');
}

#[test]
fn test13_6() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];

    // `get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // but indexing is not safe
    let _name1 = &names[1];
}

//6-6.3//

#[test]
fn test14_1() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world";
}

#[test]
fn test14_2() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // TIPS: slice( reference ) IS NOT an array, because if it is, then `assert!` will passed: each of the two UTF-8 chars '中' and '国' occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);
}

#[test]
fn test14_3() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);
}

#[test]
fn test14_4() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);
}

#[test]
fn test14_5() {
    let s = "你好，世界";
    let slice = &s[0..3];

    assert!(slice == "你");
}

#[test]
fn test14_6() {
    let mut s = String::from("hello world");

    // here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // it works because `&String` can be implicitly converted to `&str`, If you want to know more, this is called `Deref`
    let letter = first_letter(&s);

    println!("the first letter is: {}", letter);

    s.clear();
}

fn first_letter(s: &str) -> &str {
    &s[..1]
}

//6-6.4//
#[test]
fn test15_1() {
    let _t0: (u8, i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
}

#[test]
fn test15_2() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");
}

#[test]
fn test15_3() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}

#[test]
fn test15_4() {
    let tup = (1, 6.4, "hello");

    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);
}

#[test]
fn test15_5() {
    let (x, y, z);

    // fill the blank
    (y, z, x) = (1, 2, 3);

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);
}

#[test]
fn test15_6() {
    let (x, y) = sum_multiply((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}


//6-6.5//
#[test]
fn test16_1() {
    struct Person {
        name: String,
        age: u8,
        hobby: String,
    }
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: "coding".to_string(),
    };
}

struct Unit; // Оголошуємо структуру глобально

trait SomeTrait {

}


impl SomeTrait for Unit {}

#[test]
fn test16_2() {
    let u = Unit;
    do_something_with_unit(u);
}

fn do_something_with_unit(u: Unit) {}


#[derive(Debug)]
struct Color(i32, i32, i32); // Оголошуємо структуру глобально
#[derive(Debug)]
struct Point(i32, i32, i32); // Оголошуємо структуру глобально

#[test]
fn test16_3() {
    let v = Point(0, 127, 255);
    check_color(v);
}

fn check_color(p: Point) {
    let Point(x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
}


#[test]
fn test16_4() {
    struct Person {
        name: String,
        age: u8,
    }

    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };

    // how can you believe sunface is only 18?
    p.age = 30;
    p.name = String::from("sunfei");
}

#[test]
fn test16_5() {
    struct Person {
        name: String,
        age: u8,
    }

    fn build_person(name: String, age: u8) -> Person {
        Person { age, name }
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[test]
fn test16_6() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}


#[test]
fn test16_7() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // print debug info to stderr

    println!("{:?}", rect1); // print debug info to stdout
}

#[test]
fn test16_8() {
    #[derive(Debug)]
    struct File {
        name: String,
        data: String,
    }

    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name = f.name;

    println!("{}", f.data);
}


//6-6.3//
