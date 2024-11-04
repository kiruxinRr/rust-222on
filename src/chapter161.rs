#[test]
fn test1(){

    fn main() {
        let s1 = "hello";
        /* Fill in the blank */
        let s = format!("{}, world!", s1);
        assert_eq!(s, "hello, world!");
    }
}

#[test]
fn test2(){
    fn main() {
        print!("hello world, ");
        println!("I am");
        println!("Sunface!");
    }
}


