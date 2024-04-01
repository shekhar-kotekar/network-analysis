run_producer:
	@echo "INFO: Need sudo permission to run the producer and capture packets"
	cargo fmt --manifest-path=producer/Cargo.toml
	cargo check --manifest-path=producer/Cargo.toml
	cargo build --release --manifest-path=producer/Cargo.toml
	sudo ./producer/target/release/producer

kafka_topic:
	@echo "INFO: Creating Kafka topic"
#	kafka-topics --create --topic test --bootstrap-server localhost:9092 --partitions 1 --replication-factor 1

kafka_cluster:
	@echo "INFO: Creating Kafka cluster"
