use http_stub::server::Server;

#[tokio::main]
async fn main() {
  let server = Server::new().await;

  server.run().await;

  print!("Shutting down, ran for TODOms");
}
