# Oxcomm
A basic library containing a struct that translates text using Google Translate.

## Usage
```rust
use oxcomm::Translator;

#[tokio::main]
async fn main() {
    let translator = Translator::new("en".to_string(), "auto".to_string());

    let translated_text = translator.translate("Hola".to_string()).await?;

    println!("{}", translated_text);

    Ok(())
}
