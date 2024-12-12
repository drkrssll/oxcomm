use std::error::Error;

#[allow(unused)]
pub struct Translator {
    to: String,
    from: String,
}

#[allow(unused)]
impl Translator {
    pub fn new(to: &str, from: &str) -> Self {
        Self {
            to: to.to_string(),
            from: from.to_string(),
        }
    }

    pub async fn translate(&self, text: &str) -> Result<String, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let url = format!(
            "https://translate.google.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
            self.from,
            self.to,
            urlencoding::encode(&text)
        );

        let response = client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .send()
            .await?
            .text()
            .await?;

        let parsed: serde_json::Value = serde_json::from_str(&response)?;

        let translated_text = parsed[0][0][0]
            .as_str()
            .ok_or("Failed to parse translation")?
            .to_string();

        Ok(translated_text)
    }
}

#[tokio::test]
async fn test_translation() -> Result<(), Box<dyn Error>> {
    let translator = Translator::new("en", "auto");

    let translated_text = translator.translate("Hola").await?;

    assert_eq!(translated_text, "Hello");

    Ok(())
}
