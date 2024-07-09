use tokio::net::{TcpListener, TcpStream};
use tokio::io::{copy_bidirectional};
use std::sync::{Arc, Mutex};

mod weighted_round_robin;
use weighted_round_robin::{Backend, WeightedRoundRobin, SharedRoundRobin};

async fn handle_client(mut client: TcpStream, backends: SharedRoundRobin) {
    let backend_address;
    {
        let mut wrr = backends.lock().unwrap();
        backend_address = wrr.get_next_backend().address.clone();
    }

    match TcpStream::connect(backend_address).await {
        Ok(mut backend) => {
            let _ = copy_bidirectional(&mut client, &mut backend).await;
        }
        Err(e) => {
            eprintln!("Failed to connect to backend: {}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    let backends = vec![
        Backend { address: "127.0.0.1:8081".to_string(), weight: 5 },
        Backend { address: "127.0.0.1:8082".to_string(), weight: 1 },
    ];

    let round_robin = Arc::new(Mutex::new(WeightedRoundRobin::new(backends)));
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("Load balancer running on 127.0.0.1:8080");

    loop {
        let (client, _) = listener.accept().await.unwrap();
        let round_robin = Arc::clone(&round_robin);

        tokio::spawn(async move {
            handle_client(client, round_robin).await;
        });
    }
}
