# network-analysis

In this project we will capture network packets using Rust and dump these packets to Kafka. 
We will then consume these packets and analyze them using Rust.

Analytics will be stored in a database and visualized using Grafana.

Execute `make run_producer` to run the producer. Need sudo permissions to capture packets.
