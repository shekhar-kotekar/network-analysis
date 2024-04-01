## Steps to setup infrastructure
- Create k8s dashboard first
- Create Kafka cluster using command: `k apply -f kafka.yaml`

## Some useful commands
### Check if Kafka cluster is running

`kubectl get pods -o wide --namespace kafka`

### check volume claims
`kubectl get pvc --all-namespaces`

### Kafka console producer
```
kubectl exec --stdin --tty pod/kafka-0 --namespace kafka -- /bin/bash
cd /opt/kafka/bin

# list topics
./kafka-topics.sh --list --bootstrap-server localhost:9092

# create a topic
./kafka-topics.sh --create --topic test-topic --partitions 3 --replication-factor 1 --bootstrap-server localhost:9092

# start a producer
./kafka-console-producer.sh --topic test-topic --bootstrap-server localhost:9092
```

### Kafka console consumer
```
kubectl exec --stdin --tty pod/kafka-2 --namespace kafka -- /bin/bash
cd /opt/kafka/bin

# start a consumer in another terminal. You can connect to same pod as producer or any other pod
./kafka-console-consumer.sh --topic test-topic --from-beginning --bootstrap-server localhost:9092
```

### To destroy the Kafka cluster
- `k delete -f kafka.yaml`

## References:
- https://kafka.apache.org/quickstart
- https://www.mitrais.com/news-updates/deploying-a-multi-broker-kafka-cluster-in-kubernetes/
- https://kubernetes.io/docs/tasks/access-application-cluster/web-ui-dashboard/