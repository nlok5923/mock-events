use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Serialize)]
struct EventData {
    id: u64,
    calldata: String,
}

#[tokio::main]
async fn main() {
    // Set up Kafka producer
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation error");

    let topic = "PET_EVENTS"; // You can change this topic as needed

    for i in 0..100000000000 {
        let task = EventData {
            id: i,
            calldata: "0x616e6f6e0101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101".to_string(), // event for anon transfer
        };

        let tps = 100; // running with 10 tps
        for j in 0..tps {
            let payload = serde_json::to_string(&task).expect("Failed to serialize task");
            let task_id = task.id.to_string();

            println!("➡️ Sending string #{}...", i);
            let record = FutureRecord::to(topic).payload(&payload).key(&task_id);

            match producer.send(record, Duration::from_secs(0)).await {
                Ok(delivery) => println!("✅ Sent: {:?}", delivery),
                Err((err, _)) => eprintln!("❌ Send failed: {err}"),
            }
        }

        sleep(Duration::from_millis(1000)).await;
    }

    println!("🚀 Done sending!");
}
