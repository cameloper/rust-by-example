use std::fmt;

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn sum_value(&self) -> u32 {
        let red = (self.red as u32) << 16;
        let green = (self.green as u32)  << 8;
        let blue = self.blue as u32;
        red + green + blue
    }
}

impl fmt::LowerHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self.sum_value();
        fmt::LowerHex::fmt(&val, f)
    }
}

impl fmt::UpperHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self.sum_value();
        fmt::UpperHex::fmt(&val, f)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGB ({}, {}, {}) {:#08X}",
            self.red,
            self.green,
            self.blue,
            self.sum_value())
    }
}

fn main() {
    let c1 = Color { red: 128, green: 255, blue: 90 };
    println!("{}", c1);
    let c2 = Color { red: 0, green: 3, blue: 254 };
    println!("{}", c2);
    let c3 = Color { red: 0, green: 0, blue: 0 };
    println!("{}", c3);
}
