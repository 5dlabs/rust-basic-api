#!/bin/bash
# Deployment script for Kubernetes

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
NAMESPACE="${NAMESPACE:-default}"
K8S_DIR="$(cd "$(dirname "$0")/.." && pwd)/k8s"
DEPLOYMENT_NAME="rust-basic-api"

# Function to print colored messages
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_step() {
    echo -e "${BLUE}[STEP]${NC} $1"
}

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    print_error "kubectl is not installed or not in PATH"
    exit 1
fi

# Verify cluster connectivity
print_step "Verifying Kubernetes cluster connectivity..."
if ! kubectl cluster-info &> /dev/null; then
    print_error "Cannot connect to Kubernetes cluster"
    print_error "Please check your kubeconfig and cluster availability"
    exit 1
fi
print_info "✓ Connected to Kubernetes cluster"

# Check if k8s directory exists
if [ ! -d "$K8S_DIR" ]; then
    print_error "Kubernetes manifests directory not found: $K8S_DIR"
    exit 1
fi

# Create namespace if it doesn't exist
print_step "Checking namespace: $NAMESPACE"
if ! kubectl get namespace "$NAMESPACE" &> /dev/null; then
    print_info "Creating namespace: $NAMESPACE"
    kubectl create namespace "$NAMESPACE"
else
    print_info "✓ Namespace '$NAMESPACE' exists"
fi

# Validate manifests
print_step "Validating Kubernetes manifests..."
for manifest in "$K8S_DIR"/*.yaml; do
    if [ -f "$manifest" ]; then
        print_info "Validating $(basename "$manifest")..."
        if ! kubectl apply --dry-run=client -f "$manifest" -n "$NAMESPACE" &> /dev/null; then
            print_error "Invalid manifest: $manifest"
            exit 1
        fi
    fi
done
print_info "✓ All manifests are valid"

# Apply manifests
print_step "Applying Kubernetes manifests..."
for manifest in "$K8S_DIR"/*.yaml; do
    if [ -f "$manifest" ]; then
        print_info "Applying $(basename "$manifest")..."
        kubectl apply -f "$manifest" -n "$NAMESPACE"
    fi
done

# Wait for deployment to be ready
print_step "Waiting for deployment to be ready..."
if kubectl wait --for=condition=available --timeout=300s \
    deployment/"$DEPLOYMENT_NAME" -n "$NAMESPACE" 2>/dev/null; then
    print_info "✓ Deployment is ready"
else
    print_warning "Deployment did not become ready within timeout"
    print_info "Checking deployment status..."
fi

# Show deployment status
print_step "Deployment Status:"
kubectl get deployment "$DEPLOYMENT_NAME" -n "$NAMESPACE"

print_step "Pod Status:"
kubectl get pods -l app="$DEPLOYMENT_NAME" -n "$NAMESPACE"

print_step "Service Status:"
kubectl get service "$DEPLOYMENT_NAME" -n "$NAMESPACE"

# Show rollout status
print_step "Rollout Status:"
kubectl rollout status deployment/"$DEPLOYMENT_NAME" -n "$NAMESPACE" --timeout=60s || true

# Display recent events
print_step "Recent Events:"
kubectl get events -n "$NAMESPACE" --sort-by='.lastTimestamp' | tail -n 10

print_info "Deployment complete!"
print_info "To check logs, run: kubectl logs -f deployment/$DEPLOYMENT_NAME -n $NAMESPACE"
print_info "To access the service, run: kubectl port-forward service/$DEPLOYMENT_NAME 3000:80 -n $NAMESPACE"
