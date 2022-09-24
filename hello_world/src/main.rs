//I am a comment
// The following line is a doc comment  - alternatively I can use //! for the enclosing item

/// Testing struct
#[derive(Debug)]
struct Structure(i32);

/// Nested structure I suppose
#[derive(Debug)]
struct Deep(Structure);


/// Some weird person struct implementation? NOT THERE YET IN THE TUTORIAL, LEAVE ME ALONE, what is this weird <'a> syntax
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

/// Main Program
fn main() {
    /*
        Block comments look like traditional comments
    */
    println!("I'm a macro!");
    
    // Lets do some math and display!
    let my_var = 12 + 12 + 13;
    // We can use format! to write a text to a String, print and println format and print to console
    println!("`my_var` = {}", my_var);

    
    print!("I dont have a newline :)");
    println!("Hello, world!");

    eprintln!("I'm an error D:!");

    println!("{0} this is the first formatted arg. {1} this is the second formatted arg. {0} first again :D!", "My String", "The other my string");

    println!("{subject} {verb} {object}",
            subject="we",
            verb="can name",
            object="formatting names!");

    println!("Base 10: {}", 1234);
    println!("Base 2 {:b}", 1234);
    println!("Base 8 {:o}", 1234);
    println!("Base 16 {0:x} {0:X}", 1234);
    println!("Right aligned {number:>10}", number=123);
    println!("Number format with 0's and right align: {number:0>10}", number=1234);
    println!("Named arguments for formatting vars!!!!{number:0>width$}!!!!", number=123, width=12);//Noticed the $ after width in the format

    //Types need to implement fmt::Display to be formatted

    //We can use variables directly instead of passing them in!
    let my_width:usize = 4;
    println!("{my_var:>my_width$}");

    // Lets debug some structures
    println!("Let's print {:?}", Structure(3)); // Why do we want the ? if it's the same as {}
    println!("This is the deep struct {:?}", Deep(Structure(1)));


    let name = "My Name";
    let age = 12;
    let a_person = Person {name, age};
    println!("{:#?}", a_person);// Pretty printing!
}
