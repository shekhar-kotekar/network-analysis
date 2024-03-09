# Steps to setup infrastructure
- Create k8s dashboard first
- Create Kafka cluster using command: `k apply -f kafka.yaml`

# References:
- https://kafka.apache.org/quickstart
- https://www.mitrais.com/news-updates/deploying-a-multi-broker-kafka-cluster-in-kubernetes/

# Check if Kafka cluster is running
- `k get pods -n kafka`
- 