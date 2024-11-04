#[test]
fn test1(){
    fn main() {
        let i = 3; // Lifetime for `i` starts. ────────────────┐
        //                                                     │
        { //                                                   │
            let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
            //                                                ││
            println!("borrow1: {}", borrow1); //              ││
        } // `borrow1 ends. ──────────────────────────────────┘│
        //                                                     │
        //                                                     │
        { //                                                   │
            let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
            //                                                ││
            println!("borrow2: {}", borrow2); //              ││
        } // `borrow2` ends. ─────────────────────────────────┘│
        //                                                     │
    }   // Lifetime ends. ─────────────────────────────────────┘
}

#[test]
fn test2(){
    fn main() {
        {
            let r;                // ---------+-- 'a
            //          |
            {                     //          |
                let x = 5;        // -+-- 'b  |
                r = &x;           //  |       |
            }                     // -+       |
            //          |
            println!("r: {}", r); //          |
        }                         // ---------+
    }
}

#[test]
fn test3(){
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    fn main() {}
}

#[test]
fn test4(){
    fn invalid_output<'a>(s: &'a String) -> &'a String {
        s
    }

    fn main() {}
}

#[test]
fn test5(){
    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x is {} and y is {}", x, y);
    }

    /* Make it work */
    fn failed_borrow<'a>() {
        let _x = 12;

        let y: &i32 = &_x;
    }

    fn main() {
        let (four, nine) = (4, 9);

        print_refs(&four, &nine);

        failed_borrow();
    }
}

#[test]
fn test6(){
    // A type `Borrowed` which houses a reference to an
    // `i32`. The reference to `i32` must outlive `Borrowed`.
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    // Similarly, both references here must outlive this structure.
    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    // An enum which is either an `i32` or a reference to one.
    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    fn main() {
        let x = 18;
        let y = 15;

        let single = Borrowed(&x);
        let double = NamedBorrowed { x: &x, y: &y };
        let reference = Either::Ref(&x);
        let number    = Either::Num(y);

        println!("x is borrowed in {:?}", single);
        println!("x and y are borrowed in {:?}", double);
        println!("x is borrowed in {:?}", reference);
        println!("y is *not* borrowed in {:?}", number);
    }
}

#[test]
fn test7(){
    /* Make it work */

    #[derive(Debug)]
    struct NoCopyType {}

    #[derive(Debug)]
    struct Example<'a, 'b> {
        a: &'a u32,
        b: &'b NoCopyType
    }

    fn main()
    {
        /* 'a tied to fn-main stackframe */
        let var_a = 35;
        let example: Example;

        // {
        /* lifetime 'b tied to new stackframe/scope */
        let var_b = NoCopyType {};

        /* fixme */
        example = Example { a: &var_a, b: &var_b };
        // }

        println!("(Success!) {:?}", example);
    }
}

#[test]
fn test8(){

    #[derive(Debug)]
    struct NoCopyType {}

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Example<'a, 'b> {
        a: &'a u32,
        b: &'b NoCopyType
    }

    /* Fix function signature */
    fn fix_me<'b>(foo: &Example<'_, 'b>) -> &'b NoCopyType
    { foo.b }

    fn main()
    {
        let no_copy = NoCopyType {};
        let example = Example { a: &1, b: &no_copy };
        fix_me(&example);
        print!("Success!")
    }
}

#[test]
fn test9(){
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&'a self) -> i32 {
            3
        }
    }

    fn main() {}
}

#[test]
fn test10(){

    fn nput(x: &i32) {
        println!("`annotated_input`: {}", x);
    }

    fn pass(x: &i32) -> &i32 { x }

    fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
        x
    }

    struct Owner(i32);

    impl Owner {
        // Annotate lifetimes as in a standalone function.
        fn add_one(&mut self) { self.0 += 1; }
        fn print(&self) {
            println!("`print`: {}", self.0);
        }
    }

    struct Person<'a> {
        age: u8,
        name: &'a str,
    }

    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    fn main() {}
}
