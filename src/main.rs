use serde_json::json;

#[tokio::main]
async fn main() {
    let url = "http://localhost:11434/api/generate";

    let parameters = json!({
        "model": "llama3.2:1b",
        "prompt": "Why is the sky blue?",
        "stream": false,

    });

    let client = reqwest::Client::new();
    let res = client.post(url).json(&parameters).send().await.unwrap();
    let output = res.json::<serde_json::Value>().await.unwrap();

    dbg!(output);
}
