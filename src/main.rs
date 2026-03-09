mod dummy_servers;

#[tokio::main]
async fn main() {
    tokio::spawn(dummy_servers::get_servers_up(3001, "Servidor A"));
    tokio::spawn(dummy_servers::get_servers_up(3002, "Servidor B"));
    tokio::spawn(dummy_servers::get_servers_up(3003, "Servidor C"));

    println!("Dummy servers running on port 3001, 3002 and 3003");
    println!("Load balancer running on port 8080");

    tokio::signal::ctrl_c().await.unwrap();
    println!("Finishing...")
}