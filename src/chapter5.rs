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

