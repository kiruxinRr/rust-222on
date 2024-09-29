#[test]
fn test1() {
    let x: i32 = 5;
    let _y: i32;

    assert_eq!(x, 5);
    println!("Success!");
}

#[test]
fn test2() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

#[test]
fn test3() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
        println!("The value of x is {} and value of y is {}", x, y);
    }
}
#[test]
fn test4() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> &'static str {
    let x = "hello";
    x
}

#[test]
fn test5() {
    let x: i32 = 5;
    {
        let inner_x = 12;
        assert_eq!(inner_x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x);
}


#[test]
fn test6() {
    let _x: i32 = 1;
    let mut x: i32 = 7;
    x += 3;

    println!("The value of x is: {}", x);

    let _y = 4;
    let _y = "I can also be bound to text!";

    println!("Success!");
}



#[test]
fn test7() {
    let x = 1;
    println!("The value of x is: {}", x);
}

#[test]
fn test8() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
#[test]
fn test9() {
    let x = 1;
    let (a, y);
    (a, ..) = (3, 4);
    [.., y] = [1, 2];
    assert_eq!([a, y], [3, 2]);
    println!("The value of x is: {}", x);
    println!("Success!");
}






