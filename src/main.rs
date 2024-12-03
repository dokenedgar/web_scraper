use tokio::main;
mod make_request;

#[main]
async fn main() {
    make_request::make_request().await;
}
