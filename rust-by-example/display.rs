 /// Sometimes the output of fmt::Debug is not desirable
 /// We can customize the output appearance by manually implementing
 /// the fmt::Display trait which also alllows printing with {}
 
use std::fmt;


fn main() {
    
    struct Structure(i32);

    // To use use the {} marker, we must implement the 'fmt::Display' trait
    // on Structure
    impl fmt::Display for Structure {
        // this trait requires 'fmt' with this signature
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    /// generics like Vec<T> do not have display implemented 
    /// because there is no ideal display style for all types
    /// under this generic. Generics must then use the fmt::Debug
    /// trait
    
    /// Any new non generic type can have the fmt::Display trait implemented

    // A structure holding two numbers
    #[derive(Debug)]
    struct MinMax(i64, i64);

    //implement display
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    // implementing display for a structure with nameable fields
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    // implementing binary on Point2D
    impl fmt::Binary for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let magnitude = (self.x * self.x + self.y * self.y) as f64;
            let magnitude = magnitude.sqrt();

            let decimals = f.precision().unwrap_or(3);
            let string = format!("{:.*}", decimals, magnitude);
            f.pad_integral(true, "", &string)
        }
    }



    // Compare the outputs of MinMax and Point2D

    let minmax = MinMax(0, 14);

    println!("COmpare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?},", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big}, and the small is {small}",
            small = small_range,
            big = big_range);

    let point = Point2D {x: 3.3, y: 7.2};
    
    println!("Comparing points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    println!("Binary: {:b}", point);


    // Creating a Comlpex structure
    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let complex = Complex {real: 3.3, imag: 7.2};

    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}