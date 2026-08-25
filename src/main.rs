use http_stub::{cli::parse_port, server::Server};

#[tokio::main]
async fn main() {
	let port = parse_port(std::env::args().nth(1));

  let server = Server::new_on_port(port).await;

  println!("Management interface available at http://{}", server.address);

  server.run().await;
}
