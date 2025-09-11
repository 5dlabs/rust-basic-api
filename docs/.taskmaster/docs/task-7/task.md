# Task 7: Containerization and Deployment Setup

## Overview
Implement Docker containerization and Kubernetes deployment configuration for the Rust API, enabling consistent deployment across development, staging, and production environments. This task establishes the foundation for scalable, cloud-native deployment.

## Technical Context

### Containerization Benefits
- **Environment Consistency**: Same container runs everywhere
- **Dependency Isolation**: All dependencies packaged together
- **Scalability**: Easy horizontal scaling with orchestration
- **Resource Efficiency**: Lightweight compared to VMs
- **DevOps Integration**: Seamless CI/CD pipeline integration

### Technology Stack
- **Docker**: Container runtime and build system
- **Docker Compose**: Local development orchestration
- **Kubernetes**: Production orchestration platform
- **Multi-stage Builds**: Optimized container images
- **Health Checks**: Application readiness monitoring

## Implementation Guide

### Step 1: Create Multi-Stage Dockerfile
Implement an optimized Dockerfile with separate build and runtime stages:

```dockerfile
# Build stage
FROM rust:1.70 as builder
WORKDIR /app

# Dependency caching layer
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Application build
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim
WORKDIR /app
```

**Build Optimization:**
- Separate dependency and application layers
- Minimal runtime image (debian-slim)
- Only necessary runtime dependencies
- Reduced attack surface

### Step 2: Docker Compose Configuration
Create `docker-compose.yml` for local development:

```yaml
version: '3.8'
services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: rust_api_dev
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
    volumes:
      - postgres_data:/var/lib/postgresql/data
```

**Development Features:**
- PostgreSQL service configuration
- Volume mounting for hot reload
- Environment variable management
- Service dependencies

### Step 3: Kubernetes Deployment Manifest
Create `k8s/deployment.yaml`:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rust-basic-api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: rust-basic-api
```

**Deployment Configuration:**
- Replica management for high availability
- Resource limits and requests
- Health check probes
- Environment configuration via secrets

### Step 4: Kubernetes Service Definition
Create `k8s/service.yaml`:

```yaml
apiVersion: v1
kind: Service
metadata:
  name: rust-basic-api
spec:
  selector:
    app: rust-basic-api
  ports:
  - port: 80
    targetPort: 3000
  type: ClusterIP
```

**Service Features:**
- Internal cluster networking
- Load balancing across pods
- Service discovery
- Port mapping

### Step 5: Build Automation Script
Create `scripts/build_image.sh`:

```bash
#!/bin/bash
set -e

IMAGE_NAME="rust-basic-api"
IMAGE_TAG="$(git rev-parse --short HEAD)"

docker build -t $IMAGE_NAME:$IMAGE_TAG .
docker tag $IMAGE_NAME:$IMAGE_TAG $IMAGE_NAME:latest
```

**Script Features:**
- Git-based versioning
- Latest tag management
- Error handling
- Build status reporting

### Step 6: Deployment Script
Create `scripts/deploy_k8s.sh`:

```bash
#!/bin/bash
set -e

NAMESPACE="default"

kubectl apply -f k8s/deployment.yaml -n $NAMESPACE
kubectl apply -f k8s/service.yaml -n $NAMESPACE
```

**Deployment Process:**
- Namespace management
- Manifest application
- Status verification
- Rollout monitoring

## Container Optimization

### Image Size Reduction
- Use multi-stage builds
- Choose minimal base images
- Remove unnecessary dependencies
- Clean package manager caches

### Security Hardening
- Run as non-root user
- Use read-only filesystem where possible
- Scan images for vulnerabilities
- Keep base images updated

### Build Cache Optimization
```dockerfile
# Optimize layer caching
COPY Cargo.toml Cargo.lock ./
# Build dependencies first
RUN cargo build --release --locked
# Then copy source
COPY src ./src
```

## Kubernetes Configuration

### Resource Management
```yaml
resources:
  requests:
    memory: "128Mi"
    cpu: "100m"
  limits:
    memory: "512Mi"
    cpu: "500m"
```

**Resource Planning:**
- Set appropriate requests for scheduling
- Define limits to prevent resource exhaustion
- Monitor actual usage and adjust
- Consider vertical pod autoscaling

### Health Checks
```yaml
readinessProbe:
  httpGet:
    path: /health
    port: 3000
  initialDelaySeconds: 5
  periodSeconds: 10

livenessProbe:
  httpGet:
    path: /health
    port: 3000
  initialDelaySeconds: 15
  periodSeconds: 20
```

**Probe Configuration:**
- Readiness: Traffic routing control
- Liveness: Automatic pod restart
- Appropriate delays and periods
- Failure thresholds

### Configuration Management
```yaml
env:
- name: DATABASE_URL
  valueFrom:
    secretKeyRef:
      name: db-secret
      key: database-url
```

**Best Practices:**
- Use secrets for sensitive data
- ConfigMaps for configuration
- Environment-specific values
- Version configuration changes

## Local Development Workflow

### Docker Compose Commands
```bash
# Start services
docker-compose up -d

# View logs
docker-compose logs -f api

# Stop services
docker-compose down

# Rebuild images
docker-compose build
```

### Volume Mounting
```yaml
volumes:
  - .:/app          # Source code
  - /app/target     # Exclude build artifacts
```

**Development Benefits:**
- Hot reload capability
- Fast iteration cycles
- Consistent environment
- Database persistence

## Production Deployment

### Deployment Strategy
- Rolling updates for zero downtime
- Blue-green deployments for major changes
- Canary releases for gradual rollout
- Rollback capability

### Scaling Configuration
```bash
# Manual scaling
kubectl scale deployment/rust-basic-api --replicas=5

# Horizontal Pod Autoscaler
kubectl autoscale deployment/rust-basic-api --min=2 --max=10 --cpu-percent=80
```

### Monitoring and Observability
- Prometheus metrics export
- Log aggregation with Fluentd
- Distributed tracing with Jaeger
- Dashboard with Grafana

## CI/CD Integration

### Build Pipeline
```yaml
steps:
  - name: Build Docker image
    run: |
      docker build -t $IMAGE_NAME:${{ github.sha }} .
      docker push $IMAGE_NAME:${{ github.sha }}
```

### Deployment Pipeline
```yaml
steps:
  - name: Deploy to Kubernetes
    run: |
      kubectl set image deployment/rust-basic-api api=$IMAGE_NAME:${{ github.sha }}
      kubectl rollout status deployment/rust-basic-api
```

## Security Considerations

### Container Security
- Scan images with Trivy or Clair
- Sign images with Cosign
- Use distroless or scratch images
- Implement Pod Security Policies

### Network Security
- Network policies for pod communication
- Service mesh for mTLS
- Ingress with TLS termination
- API gateway for external access

## Testing Strategy

### Container Testing
```bash
# Build test
docker build -t rust-basic-api:test .

# Run test
docker run --rm rust-basic-api:test

# Health check test
docker run -d --name test-api rust-basic-api:test
curl http://localhost:3000/health
```

### Kubernetes Testing
```bash
# Deploy to test namespace
kubectl create namespace test
kubectl apply -f k8s/ -n test

# Verify deployment
kubectl get pods -n test
kubectl port-forward -n test deployment/rust-basic-api 3000:3000
```

## Troubleshooting Guide

### Common Issues
1. **Build failures**: Check Dockerfile syntax and paths
2. **Connection refused**: Verify service ports and networking
3. **Pod crashes**: Check logs with `kubectl logs`
4. **Resource limits**: Monitor with `kubectl top`

### Debugging Commands
```bash
# Pod logs
kubectl logs -f deployment/rust-basic-api

# Pod shell access
kubectl exec -it deployment/rust-basic-api -- /bin/bash

# Describe pod
kubectl describe pod <pod-name>

# Events
kubectl get events --sort-by='.lastTimestamp'
```

## Next Steps
After completing this task:
1. Implement Helm charts for templating
2. Set up GitOps with ArgoCD
3. Configure service mesh (Istio/Linkerd)
4. Add distributed tracing
5. Implement progressive delivery
6. Set up multi-region deployment
7. Configure backup and disaster recovery
8. Implement cost optimization strategies