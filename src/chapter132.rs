#[test]
fn test1(){

    // FILL in the blanks and FIX the errors
    use std::num::ParseIntError;

    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        let n1 = n1_str.parse::<i32>();
        let n2 = n2_str.parse::<i32>();
        Ok(n1.unwrap() * n2.unwrap())
    }

    fn main() {
        let result = multiply("10", "2");
        assert_eq!(result, Ok(20));

        let result = multiply("4", "2");
        assert_eq!(result.unwrap(), 8);

        println!("Success!")
    }
}

#[test]
fn test2(){
    use std::num::ParseIntError;

    // IMPLEMENT multiply with ?
    // DON'T use unwrap here
    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        let n1 = n1_str.parse::<i32>()?;
        let n2 = n2_str.parse::<i32>()?;
        Ok(n1 * n2)
    }

    fn main() {
        assert_eq!(multiply("3", "4").unwrap(), 12);
        println!("Success!")
    }
}

#[test]
fn test3(){
    use std::fs::File;
    use std::io::{self, Read};

    fn read_file1() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn read_file2() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }

    fn main() {
        assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
        println!("Success!")
    }
}
