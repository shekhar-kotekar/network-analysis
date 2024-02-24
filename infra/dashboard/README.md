# Install k8s dashboard using following commands:
```
helm repo add kubernetes-dashboard https://kubernetes.github.io/dashboard/
helm upgrade \
    --install kubernetes-dashboard kubernetes-dashboard/kubernetes-dashboard \
    --create-namespace \
    --namespace kubernetes-dashboard
```

# Create k8s resources
```
k apply -f namespace.yaml
k apply -f admin-user.yaml
k apply -f role-binding.yaml
```

- Create a token for the dashboard

`kubectl -n kubernetes-dashboard create token admin-user`

- Copy the generated token

- Open k8s dashboard on https://127.0.0.1:8443/

- Provide token generated in previous step
