use clipboard::{ClipboardContext, ClipboardProvider};

pub fn set_clipboard_content(content: &String) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(content.to_owned())?;
    println!("The content has been copied to clipboard");
    Ok(())
}
