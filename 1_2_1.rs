
#[derive(Debug)]
struct DebugPrintable(i32);

fn main() {
    println!("{} days", 31);

    println!("{subject} {object} {verb}",
        subject="ali",
        object="topu",
        verb="at");

    println!("base 10: {n} - base 2: {n:b}", n=10);

    println!("{n:0>w$}", n=3, w=10);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);

    println!("Now {:?} will print", DebugPrintable(4));
}
