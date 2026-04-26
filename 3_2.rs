enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("User pressed {}", c),
        WebEvent::Paste(s) => println!("User pasted string: {}", s),
        WebEvent::Click { x, y } => {
            println!("User clicked at x={}, y={}", x, y);
        },
    }
}

fn main() {
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    let pressed = WebEvent::KeyPress('Ü');
    let pasted = WebEvent::Paste("üzüldüm".to_owned());
    let clicked = WebEvent::Click { x: 10, y: 66 };

    inspect(load);
    inspect(unload);
    inspect(pressed);
    inspect(pasted);
    inspect(clicked);

    let x = Operations::Add;
}   
