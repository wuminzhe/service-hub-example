use xtra::Address;

use async_std::task;

lazy_static::lazy_static! {
    static ref SERVICES: Vec<Box<dyn Service + Sync>> = {
        vec![
            Box::new(AbcService {}),
            Box::new(DefService {})
        ]
    };
}

#[derive(Clone)]
pub struct Message {
    from: &'static str,
    route_key: &'static str,
    content: String,
}

#[async_trait::async_trait]
pub trait Service {
    async fn send(&self, msg: Message);
}

pub async fn broadcast(msg: Message) {
    for service in SERVICES.iter() {
        service.send(msg.clone()).await;
    }
}

/////////////////////////////////////////////

pub struct AbcService {
}

#[async_trait::async_trait]
impl Service for AbcService {
    async fn send(&self, msg: Message) {
        if msg.from != "abc" {
            println!("in abc ---------- from: {}", msg.from);
            let msg = Message {
                from: "abc",
                route_key: "",
                content: "".to_string()
            };
            broadcast(msg).await;
        }

    }
}


pub struct DefService {
}

#[async_trait::async_trait]
impl Service for DefService {
    async fn send(&self, msg: Message) {
        println!("in def ---------- from: {}", msg.from);
    }
}


pub struct ServiceHub {
}


fn main() {
    let msg = Message {
        from: "admin",
        route_key: "",
        content: "".to_string()
    };
    task::block_on(
        broadcast(msg)
    );
}



