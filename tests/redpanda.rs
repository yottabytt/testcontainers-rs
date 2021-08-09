use futures::StreamExt;
use rdkafka::{
    config::ClientConfig,
    consumer::{stream_consumer::StreamConsumer, Consumer},
    producer::{FutureProducer, FutureRecord},
    Message,
    topic_partition_list::TopicPartitionList,
};
use std::time::Duration;

use testcontainers::{clients, core::RunArgs, images::redpanda};

#[tokio::test]
async fn redpanda_kafka_api() {
    let docker = clients::Cli::default();
    let kafka_node = docker.run_with_args(
        redpanda::Redpanda::default(),
        RunArgs::default().with_mapped_port((9092, 9092)).with_mapped_port((9644, 9644)),
    );

    // try kafkacat and debug. consumers get panicked coz of KafkaError (Message consumption error: NotCoordinator (Broker: Not coordinator))
    // both for kafka and redpanda when run on docker
    // need to figure this properly. also try librdkafka trace logs

    let bootstrap_servers = format!("localhost:{}", kafka_node.get_host_port(9092));

    let producer = ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("message.timeout.ms", "5000")
        .create::<FutureProducer>()
        .expect("Failed to create Kafka FutureProducer");

    let consumer = ClientConfig::new()
        .set("group.id", "testcontainer-rs")
        .set("bootstrap.servers", &bootstrap_servers)
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .create::<StreamConsumer>()
        .expect("Failed to create Kafka StreamConsumer");

    let topic = "rp-test-topic";
    let mut tpl = TopicPartitionList::new();
    tpl.add_partition(topic, 0);
    consumer
        .assign(&tpl)
        .expect("Failed to assign a topic");

    let number_of_messages_to_produce = 5_usize;
    let expected: Vec<String> = (0..number_of_messages_to_produce)
        .map(|i| format!("Message {}", i))
        .collect();

    for (i, message) in expected.iter().enumerate() {
        println!("{:?}", message);
        let res = producer
            .send(
                FutureRecord::to(&topic).partition(0)
                    .payload(message)
                    .key(&format!("Key {}", i)),
                Duration::from_secs(0),
            )
            .await
            .unwrap();
        println!("{:?}", res);
    }

    println!("before consuming");
    let mut message_stream = consumer.stream();
    for produced in expected {
        let borrowed_message = tokio::time::timeout(Duration::from_secs(10), message_stream.next())
            .await
            .unwrap()
            .unwrap();

        println!("{:?}", borrowed_message);

        assert_eq!(
            produced,
            borrowed_message
                .unwrap()
                .payload_view::<str>()
                .unwrap()
                .unwrap()
        );
    }
}
