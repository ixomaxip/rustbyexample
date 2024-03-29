use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

//ACTIVITY
struct List1(Vec<i32>);

impl fmt::Display for List1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = & self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1,2,3]);
    let v1 = List1(vec![1,2,3]);
    println!("list:\n{}", v);
    println!("list1:\n{}", v1);
}
