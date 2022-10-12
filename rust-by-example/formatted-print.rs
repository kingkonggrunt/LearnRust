// Formatted Printing

fn main() {
    // write formatted text to String
    format!("Hello World")

    // Print text to console (io::stdout)
    print!("Print to console")

    // Print to console with newline appened
    println!("Print to console with newline")

    // Print to standard error (io::stderr) with or without newline
    eprint!("Print error")
    eprintln!("Print error with newline")

    // Printing can be formatted using {}
    println!("{} days", 31)
    println!("{0} and {1}", "Bob", "Alice")
    println!("{name}, {verb}, {object}",
            name="Bob",
            verb="goes",
            object="shopping")
    
    // Formatting using the ':' character
    println!("Base 10 repr:                 {}", 69420)
    println!("Base 2 (binary) repr:         {:b}", 69420)
    println!("Base 8 (octal) repr:          {:o}", 69420)
    println!("Base 16 (hexidecimal) repr:   {:x}", 69420)
    
    // Right alignment
    println!("{number:>5}", number=1)
    // With Zero Padding
    println!("{number:0>5}", number=1)
    // Padding and alignment using parameters
    println!("{number:0>width$}", number=1, width=5)

    // User defined types cannot be formated using {} unless they implement fmt::Display
    // fmt::Debug available for debugging purposes {:?}
    // fmt::Display implements the ToString trait allowing us to convert the type to String

}