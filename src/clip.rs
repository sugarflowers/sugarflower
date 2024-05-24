use clipboard::{ClipboardProvider, ClipboardContext};

pub fn set_clipboard(text: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(text.to_owned()).unwrap();
}

pub fn get_clipboard() -> String { 
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let text = ctx.get_contents().unwrap();
    text.to_string()
}

#[test]
fn clipboard_test() {
    let _ = set_clipboard("hello!!!!");
    println!("{}", get_clipboard());
}

