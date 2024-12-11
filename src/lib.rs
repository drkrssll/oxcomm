use std::error::Error;

#[allow(unused)]
struct Translator {
    to: String,
    from: String,
    client: reqwest::Client,
}

#[allow(unused)]
impl Translator {
    fn new(to: String, from: String) -> Self {
        Self {
            to,
            from,
            client: reqwest::Client::new(),
        }
    }

    async fn translate(&self, text: String) -> Result<String, Box<dyn Error>> {
        let url = format!(
            "https://translate.google.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
            self.from,
            self.to,
            urlencoding::encode(&text)
        );

        let response = self.client
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
    let translator = Translator::new("en".to_string(), "auto".to_string());

    let translated_text = translator.translate("Hola".to_string()).await?;

    assert_eq!(translated_text, "Hello");

    Ok(())
}
