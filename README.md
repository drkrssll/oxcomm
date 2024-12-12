# Oxcomm
A basic library containing a struct that translates text using Google Translate.

## Usage
```rust
use oxcomm::Translator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let translator = Translator::new("en", "auto");

    let translated_text = translator.translate("Hola").await?;

    println!("{}", translated_text);

    Ok(())
}
