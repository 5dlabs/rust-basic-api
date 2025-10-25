#!/bin/bash
set -e

# Kubernetes deployment script for rust-basic-api
# This script deploys the application to a Kubernetes cluster

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script configuration
NAMESPACE="${NAMESPACE:-default}"
KUBECTL="${KUBECTL:-kubectl}"
K8S_DIR="$(dirname "$0")/../k8s"

# Print colored messages
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_step() {
    echo -e "${BLUE}[STEP]${NC} $1"
}

# Check if kubectl is available
check_kubectl() {
    if ! command -v "$KUBECTL" &> /dev/null; then
        print_error "kubectl not found. Please install kubectl first."
        exit 1
    fi
    
    # Check cluster connectivity
    if ! $KUBECTL cluster-info &> /dev/null; then
        print_error "Cannot connect to Kubernetes cluster. Please check your kubeconfig."
        exit 1
    fi
    
    print_info "Connected to Kubernetes cluster"
}

# Create namespace if it doesn't exist
ensure_namespace() {
    if $KUBECTL get namespace "$NAMESPACE" &> /dev/null; then
        print_info "Namespace '$NAMESPACE' already exists"
    else
        print_step "Creating namespace '$NAMESPACE'"
        $KUBECTL create namespace "$NAMESPACE"
        print_info "Namespace created"
    fi
}

# Check if secret exists
check_secret() {
    if $KUBECTL get secret rust-basic-api-secrets -n "$NAMESPACE" &> /dev/null; then
        print_info "Secret 'rust-basic-api-secrets' found"
        return 0
    else
        print_warn "Secret 'rust-basic-api-secrets' not found"
        print_warn "Please create the secret before deploying:"
        echo ""
        echo "  kubectl create secret generic rust-basic-api-secrets \\"
        echo "    --namespace=$NAMESPACE \\"
        echo "    --from-literal=database-url=\"\${DATABASE_URL}\""
        echo ""
        return 1
    fi
}

# Apply Kubernetes manifests
apply_manifests() {
    print_step "Applying Kubernetes manifests"
    
    # Apply service first (for DNS)
    if [ -f "$K8S_DIR/service.yaml" ]; then
        print_info "Applying service.yaml"
        $KUBECTL apply -f "$K8S_DIR/service.yaml" -n "$NAMESPACE"
    fi
    
    # Apply deployment
    if [ -f "$K8S_DIR/deployment.yaml" ]; then
        print_info "Applying deployment.yaml"
        $KUBECTL apply -f "$K8S_DIR/deployment.yaml" -n "$NAMESPACE"
    fi
    
    print_info "Manifests applied successfully"
}

# Wait for deployment to be ready
wait_for_deployment() {
    print_step "Waiting for deployment to be ready"
    
    if $KUBECTL rollout status deployment/rust-basic-api -n "$NAMESPACE" --timeout=300s; then
        print_info "Deployment is ready!"
        return 0
    else
        print_error "Deployment failed or timed out"
        return 1
    fi
}

# Show deployment status
show_status() {
    print_step "Deployment status:"
    echo ""
    
    print_info "Pods:"
    $KUBECTL get pods -n "$NAMESPACE" -l app=rust-basic-api
    echo ""
    
    print_info "Service:"
    $KUBECTL get service rust-basic-api -n "$NAMESPACE"
    echo ""
    
    print_info "Deployment:"
    $KUBECTL get deployment rust-basic-api -n "$NAMESPACE"
    echo ""
}

# Show logs
show_logs() {
    print_step "Recent logs from pods:"
    echo ""
    $KUBECTL logs -n "$NAMESPACE" -l app=rust-basic-api --tail=20 --prefix=true
}

# Main deployment function
main() {
    print_info "Starting deployment to Kubernetes"
    print_info "Namespace: $NAMESPACE"
    echo ""
    
    # Check prerequisites
    check_kubectl
    
    # Ensure namespace exists
    ensure_namespace
    
    # Check for secret
    if ! check_secret; then
        if [ "${SKIP_SECRET_CHECK:-false}" != "true" ]; then
            print_error "Secret check failed. Set SKIP_SECRET_CHECK=true to skip."
            exit 1
        else
            print_warn "Skipping secret check (SKIP_SECRET_CHECK=true)"
        fi
    fi
    
    # Apply manifests
    apply_manifests
    
    # Wait for deployment
    if wait_for_deployment; then
        # Show status
        show_status
        
        # Show logs if requested
        if [ "${SHOW_LOGS:-false}" = "true" ]; then
            show_logs
        fi
        
        print_info "Deployment completed successfully!"
        echo ""
        print_info "To view logs, run:"
        echo "  kubectl logs -n $NAMESPACE -l app=rust-basic-api -f"
        echo ""
        print_info "To port-forward the service, run:"
        echo "  kubectl port-forward -n $NAMESPACE service/rust-basic-api 3000:80"
        
        return 0
    else
        print_error "Deployment failed!"
        show_status
        return 1
    fi
}

# Show usage
usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Deploy rust-basic-api to Kubernetes cluster

OPTIONS:
    -h, --help              Show this help message
    -n, --namespace NS      Set namespace (default: default)
    -s, --skip-secret       Skip secret existence check
    -l, --logs              Show logs after deployment
    -d, --dry-run           Dry-run mode (validate only)

ENVIRONMENT VARIABLES:
    NAMESPACE               Kubernetes namespace (default: default)
    KUBECTL                 kubectl binary path (default: kubectl)
    SKIP_SECRET_CHECK       Set to 'true' to skip secret check
    SHOW_LOGS               Set to 'true' to show logs after deployment

EXAMPLES:
    # Deploy to default namespace
    $0

    # Deploy to custom namespace
    $0 --namespace production

    # Deploy with logs
    $0 --logs

    # Dry-run (validate manifests)
    $0 --dry-run
EOF
}

# Parse arguments
DRY_RUN=false
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            usage
            exit 0
            ;;
        -n|--namespace)
            NAMESPACE="$2"
            shift 2
            ;;
        -s|--skip-secret)
            SKIP_SECRET_CHECK=true
            shift
            ;;
        -l|--logs)
            SHOW_LOGS=true
            shift
            ;;
        -d|--dry-run)
            DRY_RUN=true
            shift
            ;;
        *)
            print_error "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

# Handle dry-run
if [ "$DRY_RUN" = true ]; then
    print_info "Running in dry-run mode (validation only)"
    KUBECTL="$KUBECTL --dry-run=client"
fi

# Run main function
main
