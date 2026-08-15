use http_stub::server::Server;

#[tokio::main]
async fn main() {
  let server = Server::new_on_port(8080).await;

	println!("Server address: {}", server.address);

  server.run().await;
}
