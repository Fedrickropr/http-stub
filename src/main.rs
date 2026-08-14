use http_stub::server::Server;

#[tokio::main]
async fn main() {
  Server::new().await;

  print!("Shutting down, ran for TODOms");
}
