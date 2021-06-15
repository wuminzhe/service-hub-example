use std::time::Duration;
use flume::{Sender, Receiver};

#[macro_use]
extern crate async_trait;

struct AbcService {
    tx: Sender<String>,
    rx: Receiver<String>
}

impl AbcService {
    pub fn new(tx: Sender<String>, rx: Receiver<String>) -> Self {
        AbcService {
            tx, rx
        }
    }

    pub async fn start_listening(&self) {
        let rx_cloned = self.rx.clone();
        let _t = async_std::task::spawn(async move {
            while let Ok(msg) = rx_cloned.recv_async().await {
                println!("Abc Received: {}", msg);
            }
        });
    }

}


struct HelloService {
    tx: Sender<String>,
    rx: Receiver<String>
}

impl HelloService {
    pub fn new(tx: Sender<String>, rx: Receiver<String>) -> Self {
        HelloService {
            tx, rx
        }
    }

    pub async fn start_listening(&self) {
        let rx_cloned = self.rx.clone();
        let _t = async_std::task::spawn(async move {
            while let Ok(msg) = rx_cloned.recv_async().await {
                println!("Hello Received: {}", msg);
            }
        });
    }
}

#[async_std::main]
async fn main() {
    let (tx, rx) = flume::unbounded();

    let hello = HelloService::new(tx.clone(), rx.clone());
    let abc = AbcService::new(tx.clone(), rx.clone());
    hello.start_listening().await;
    abc.start_listening().await;

    for i in (0.. 100) {
        tx.send_async(format!("msg - {}", i)).await.unwrap();
        async_std::task::sleep(Duration::from_secs(1)).await;
    }
}