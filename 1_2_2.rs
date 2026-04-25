use std::fmt;

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i32, i32);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
    // add code here
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

fn pretty(f: f64) -> String {
    if f < 0.0 {
        return format!("- {:.1}", f.abs());
    } else if f > 0.0 {
        return format!("+ {:.1}", f.abs());
    } else {
        return format!("{:.1}", f.abs());
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1} {}i", self.real, pretty(self.imag))
    }
}

fn main() {
    
    println!("Test printing Structure: {}", Structure(39));
    println!("Test debug Structure: {:?}", Structure(39));

    let minmax = MinMax(99, 32);
    println!("Test display MinMax: {}", minmax);
    println!("Test debug MinMax: {:?}", minmax);
    println!("Test binary MinMax: {:0>8b}, {:0>8b}", minmax.0, minmax.1);

    let point = Point2D { x: 3.14, y: 2.12 };
    println!("Test display Point2D: {}", point);
    println!("Test debug Point2D: {:?}", point);

    let c1 = Complex { real: 3.3, imag: 7.2 };
    println!("Display: {}", c1);
    println!("Debug: {:?}", c1);

    let c2 = Complex { real: 4.7, imag: -2.3 };
    println!("Display: {}", c2);
    println!("Debug: {:?}", c2);
}

