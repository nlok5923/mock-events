use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use tokio::time::sleep;
use serde::Serialize;

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

    // Sample hash string to send
    let payload = "39985c362bca8da009f91cf3d75ce1a23a5cd74417ec1e74c44dec66faea04d3";
    let trival_encrypt = "0000000000000000000000000000000000000000000000000000000000000004";

    for i in 0..100000000000 {

        let task = EventData {
            id: i,
            // calldata: "f953e4273087dfa89c3902fad6209c15cc647321f9a4e93ce47ee93b8f7c8cfbd44c04133087dfa89c3902fad6209c15cc647321f9a4e93ce47ee93b8f7c8cfbd44c041300".to_string()
            calldata: "e71746b8000000000000000000000000000000000000000000000000000000000000000402".to_string() // for trivial encrypt
        };

        let payload = serde_json::to_string(&task).expect("Failed to serialize task");
        let task_id = task.id.to_string();

        println!("➡️ Sending string #{}...", i);
        let record = FutureRecord::to(topic)
            .payload(&payload)
            .key(&task_id);

        match producer.send(record, Duration::from_secs(0)).await {
            Ok(delivery) => println!("✅ Sent: {:?}", delivery),
            Err((err, _)) => eprintln!("❌ Send failed: {err}"),
        }

        sleep(Duration::from_millis(10000)).await;
    }

    println!("🚀 Done sending!");
}

            /* c58f61eaa5eebcf18e94f9fbe513c1bbf886f2809f923ed3e5e29739b2900405acacb16838726cd156d6a0bda678441fcb854a8a68eaacbfce7a17ba129804ac00".to_string(), */
