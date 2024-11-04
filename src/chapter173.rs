#[test]
fn test1(){
    struct DoubleRef<'a,'b:'a, T> {
        r: &'a T,
        s: &'b T
    }
    fn main() {
        println!("Success!")
    }
}

#[test]
fn test2(){
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a: 'b, 'b> ImportantExcerpt<'a> {
        fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    fn main() {
        println!("Success!")
    }
}

#[test]
fn test3(){
    fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) where 'a: 'b {
        y = x;                      // &'a i32 is a subtype of &'b i32 because 'a: 'b
        let r: &'b &'a i32 = &&0;   // &'b &'a i32 is well formed because 'a: 'b
    }
    fn main() {
        println!("Success!")
    }
}

#[test]
fn test4(){
    fn call_on_ref_zero<F>(f: F) where for<'a> F: Fn(&'a i32) {
        let zero = 0;
        f(&zero);
    }

    fn main() {
        println!("Success!")
    }
}

#[test]
fn test5(){
    fn main() {
        let mut data = 10;
        let ref1 = &mut data;
        let ref2 = &mut *ref1;

        *ref2 += 2;
        *ref1 += 1;

        println!("{}", data);
    }
}

#[test]
fn test6(){
    struct Interface<'b, 'a: 'b> {
        manager: &'b mut Manager<'a>
    }

    impl<'b, 'a: 'b> Interface<'b, 'a> {
        pub fn noop(self) {
            println!("interface consumed");
        }
    }

    struct Manager<'a> {
        text: &'a str
    }

    struct List<'a> {
        manager: Manager<'a>,
    }

    impl<'a> List<'a> {
        pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a>
        where 'a: 'b {
            Interface {
                manager: &mut self.manager
            }
        }
    }

    fn main() {

        let mut list = List {
            manager: Manager {
                text: "hello"
            }
        };

        list.get_interface().noop();

        println!("Interface should be dropped here and the borrow released");

        use_list(&list);
    }

    fn use_list(list: &List) {
        println!("{}", list.manager.text);
    }

}