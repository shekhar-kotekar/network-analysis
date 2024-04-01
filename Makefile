run_producer:
	echo "INFO: Need sudo permission to run the producer and capture packets"
	cargo fmt --manifest-path=producer/Cargo.toml
	cargo check --manifest-path=producer/Cargo.toml
	cargo build --release --manifest-path=producer/Cargo.toml
	sudo ./producer/target/release/producer
