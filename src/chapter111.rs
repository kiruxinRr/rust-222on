#[test]
fn test1(){

    // FILL in the blanks and FIX errors
    // 1. Don't use `to_string()`
    // 2. Don't add/remove any code line
    fn main() {
        let mut s: String = String::from("hello, ");
        s.push_str("world");
        s.push('!');

        move_ownership(s.clone());

        assert_eq!(s, "hello, world!");

        println!("Success!")
    }

    fn move_ownership(s: String) {
        println!("ownership of \"{}\" is moved here!", s)
    }
}

#[test]
fn test2(){
    // FILL in the blanks
    fn main() {
        // get a slice of String with reference: String -> &str
        let mut s = String::from("hello, world");

        let slice1: &str = &s; // in two ways
        assert_eq!(slice1, "hello, world");

        let slice2 = &s[0..5];
        assert_eq!(slice2, "hello");

        //Note! The type here cant be `&mut str` due to `push` is ONLY defined on String type and its mut reference: `&mut String` !
        // So you can't use `s.as_mut_str()`
        let slice3: &mut String = &mut s;
        slice3.push('!');
        assert_eq!(slice3, "hello, world!");

        println!("Success!")
    }
}

#[test]
fn test3(){

    // Question: how many heap allocations are happening here?
    // Your answer:
    fn main() {
        let mut s = String::from("hello, world");

        let slice1: &str = s.as_str();
        assert_eq!(slice1, "hello, world");

        let slice2 = &s[0..5];
        assert_eq!(slice2, "hello");

        //Note! The type here cant be `&mut str` due to `push` is ONLY defined on String type and its mut reference: `&mut String` !
        // So you can't use `s.as_mut_str()`
        let slice3: &mut String = &mut s;
        slice3.push('!');
        assert_eq!(slice3, "hello, world!");

        println!("Success!")
    }
}

#[test]
fn test4(){

    // FILL in the blank and FIX errors
    fn main() {
        let s = String::from("hello, 世界");
        let slice1 = &s[0..1]; //modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
        assert_eq!(slice1, "h");

        let slice2 = &s[7..10];//modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
        assert_eq!(slice2, "世");

        for (i, c) in s.chars().enumerate() {
            if i == 7 {
                assert_eq!(c, '世')
            }
        }

        println!("Success!")
    }
}

#[test]
fn test5(){

    // FILL in the blanks
    fn main() {
        let mut s = String::new();
        s.push_str("hello");

        // some bytes, in a vector
        let v = vec![104, 101, 108, 108, 111];

        // Turn a bytes vector into a String
        let s1 = String::from_utf8(v).unwrap();


        assert_eq!(s, s1);

        println!("Success!")
    }
}

#[test]
fn test6(){

    // Modify the code below to print out:
    // 25
    // 25
    // 25
    // Here, there’s no need to allocate more memory inside the loop.
    fn main() {
        let mut s = String::with_capacity(25);

        println!("{}", s.capacity());

        for _ in 0..2 {
            s.push_str("hello");
            println!("{}", s.capacity());
        }

        println!("Success!")
    }
}

#[test]
fn test7(){

    // FILL in the blanks
    use std::mem;

    fn main() {
        let story = String::from("Rust By Practice");

        // Prevent automatically dropping the String's data
        let mut story = mem::ManuallyDrop::new(story);

        let ptr = story.as_mut_ptr();
        let len = story.len();
        let capacity = story.capacity();

        // story has nineteen bytes
        assert_eq!(16, len);

        // We can re-build a String out of ptr, len, and capacity. This is all
        // unsafe because we are responsible for making sure the components are
        // valid:
        let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

        assert_eq!(*story, s);

        println!("Success!")
    }
}
