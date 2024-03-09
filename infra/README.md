## Steps to setup infrastructure
- Create k8s dashboard first
- Create Kafka cluster using command: `k apply -f kafka.yaml`

## Check if Kafka cluster is running
- `kubectl get pods -o wide --namespace kafka`
- Create Kafka console producer
```
kubectl exec --stdin --tty pod/kafka-0 --namespace kafka -- /bin/bash
cd /opt/kafka/bin
```

## To destroy the Kafka cluster
- `k delete -f kafka.yaml`

## References:
- https://kafka.apache.org/quickstart
- https://www.mitrais.com/news-updates/deploying-a-multi-broker-kafka-cluster-in-kubernetes/
