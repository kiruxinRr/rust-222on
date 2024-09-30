//4-4.1//

#[test]
fn test1() {
    let x: i32 = 5;
    let mut _y: u32 = 5;

    _y = x as u32;

    let _z = 10;

    println!("Success!");
}

#[test]
fn test2() {
    let _v: u16 = 38_u8 as u16;

    println!("Success!");
}

#[test]
fn test3() {
    let x = 5;
    assert_eq!("i32", type_of(&x));

    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test4() {
    assert_eq!(i8::MAX, std::i8::MAX);
    assert_eq!(u8::MAX, std::u8::MAX);

    println!("Success!");
}

#[test]
fn test5() {

    let v1 = 251_u8.checked_add(8).unwrap_or(0);

    let v2 = 251_u8.checked_add(8).unwrap_or(0);

    println!("{}, {}", v1, v2);
}

#[test]
fn test6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}

#[test]
fn test7() {
    let x = 1_000.000_1;
    let _y: f32 = 0.12;
    let _z = 0.01_f64;

    assert_eq!(type_of_value(&x), "f64".to_string());
    println!("Success!");
}

fn type_of_value<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test8() {
    assert!(0.1_f32+0.2_f32==0.3_f32);
}

#[test]
fn test9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}

use std::ops::{Range, RangeInclusive}; // Keep this only once

#[test]
fn test10() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

#[test]
fn test11() {
    assert!(1u32 + 2 == 3);
    assert!(1i32 - 2 == -1);
    assert!(1u8.wrapping_sub(2) == 255);

    assert!(3 * 50 == 150);


    let epsilon = 1e-10;
    assert!((9.6_f64 / 3.2_f64 - 3.0_f64).abs() < epsilon);

    assert!(24 % 5 == 4);


    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);


    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}


//4-4.2//



use std::mem::size_of_val;

#[test]
fn test12() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);
}


#[test]
fn test13() {
    let c1 = '中';
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}

#[test]
fn test14() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}
#[test]
fn test15() {
    let f = false;
    let t = false; // true && false завжди дорівнює false
    assert_eq!(t, f);

    println!("Success!");
}


#[test]
fn test16() {
    let _v: () = ();

    let v = ();
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

#[cfg(test)]
mod tests {
    use std::mem::size_of_val;

    #[test]
    fn test17() {
        let unit: () = ();
        assert!(size_of_val(&unit) == 0);
    }
}



//4-4.3//
#[test]
fn test18() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}

#[test]
fn test19() {
    let v = {
        let x = 3;
        x
    };

    assert!(v == 3);

    println!("Success!");
}

#[test]
fn test20() {
    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}









