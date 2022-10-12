// All types wanting a std::fmt formatting trait require an implementation
// to be printable

// Implementing a debug trait is rather straightforward

fn main() {

    // We derive the Debug attribute to create an implementation
    // that makes this structure printable with 'fmt::Debug'
    #[derive(Debug)]
    struct Structure(i32); 

    // All std library types automatically have the fmt::Debug trait
    // {:?}

    // Put Structure inside a structure 'Deep'. Make it printable as well
    #[derive(Debug)]
    struct Deep(Structure);

    // Printing structures with the Debug trait is done with {:?} instead of {}
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
            "Slater",
            "Christian",
            actor="actor's");
    
    // The problem with derive is that we can't control how the results look.
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(3)));


    // Pretty print can be done with {:#?}
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};

    println!("{:#?}", peter);
}