//5-5.1//

#[test]
fn test1() {
    let x = String::from("Hello world");
    let y = &x;
    println!("{}, {}", x, y);
}

#[test]
fn test2() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
#[test]
fn test3() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    s
}

#[test]
fn test4() {
    let s = String::from("hello, world");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}

#[test]
fn test5() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}





#[test]
fn test6() {
    let s = String::from("hello, ");

    // modify this line only !
    let mut s1 = s;

    s1.push_str("world")
}




#[test]
fn test7() {
    let x = Box::new(5);

    let mut y = Box::new(3);       // implement this line, dont change other lines!

    *y = 4;

    assert_eq!(*x, 5);
}






#[test]
fn test8() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;

    // modify this line only, don't use `_s`
    println!("{:?}", t.1);
}

#[test]
fn test9() {
    let t = (String::from("hello"), String::from("world"));

    // fill the blanks
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}


//5-5.2//

#[test]
fn test10() {
    let x = 5;
    let p = &x;
    println!("the memory address of x is {:p}", p);
}

#[test]
fn test11() {
    let x = 5;
    let y = &x;
    assert_eq!(5, *y);
}

#[test]
fn test12() {
    let mut s = String::from("hello, ");
    borrow_object_immutable(&s);
}

fn borrow_object_immutable(s: &String) {}

#[test]
fn test13() {
    let mut s = String::from("hello, ");
    push_str(&mut s);
}

fn push_str(s: &mut String) {
    s.push_str("world");
}

#[test]
fn test14() {
    let mut s = String::from("hello, ");
    let p = &mut s;
    p.push_str("world");
}

#[test]
fn test15() {
    let c = '中';
    let r1 = &c;
    let ref r2 = c;
    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1), get_addr(r2));
}

fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

#[test]
fn test16() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
}

#[test]
fn test17() {
    let mut s = String::from("hello, ");
    borrow_object_mutable(&mut s);
}

fn borrow_object_mutable(s: &mut String) {}

#[test]
fn test18() {
    let mut s = String::from("hello, ");
    borrow_object_immutable(&s);
    let mut s = s;
    s.push_str("world");
}

fn borrow_object_immutable_once(s: &String) {}

#[test]
fn test19() {
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    r1.push_str("world");
}

#[test]
fn test20() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");


    let r2 = &mut s;
    r2.push_str("!");
}

