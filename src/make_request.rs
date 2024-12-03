use reqwest::{Client, RequestBuilder};
pub async fn make_request() {
    let client = Client::new();
    let request = client.get("https://docs.rs/reqwest/0.12.9/reqwest/struct.Client.html");
    let result = RequestBuilder::send(request).await;
    match result {
        Ok(val) => {
            println!("Stat: {:?}", val.status());
            println!("Body: {:?}", val.text().await);
        }
        Err(err) => println!("Error performing op: {:?}", err),
    }
}
