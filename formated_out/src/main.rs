fn main() {
    println!("January has ");

    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {predicate}",
            predicate="over the lazy dog",
            subject="the quick brown fox",
            verb="jumps");
    
    println!("{} of {:b} people know binary, the other half doesn't", 1,2);

    println!("\t{number:>width$}", number=1, width=6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);
    // println!("This struct `{}` won't print", Structure(3));

    let pi = 3.1415926;

    println!("Pi is roughly {:.3}", pi);


}
