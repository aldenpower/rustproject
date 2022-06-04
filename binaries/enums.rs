#[derive(Debug)]
enum WebEvent {
    Pageload,
    Pageunload,
    KeyPress(char),
    Paste(String),
    Click {x: i64, y:i64},
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::Pageload => println!("Page loaded"),
        WebEvent::Pageunload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Pressed {}", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click{ x, y } => {
            println!("Clicked at x={}, y={}", x, y);
        },
    }
}


fn main () {
    let pressed: WebEvent = WebEvent::KeyPress('x');
    let click: WebEvent = WebEvent:: Click{x:20, y:80};

    inspect(pressed);
    inspect(click);
}