#!/bin/bash
set -e

# Build Docker image script for rust-basic-api
# This script builds a Docker image tagged with git commit SHA and 'latest'

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Script configuration
IMAGE_NAME="${IMAGE_NAME:-rust-basic-api}"
REGISTRY="${REGISTRY:-}"

# Print colored message
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Get git commit SHA
get_git_sha() {
    if command -v git &> /dev/null; then
        GIT_SHA=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
        if [ "$GIT_SHA" = "unknown" ]; then
            print_warn "Not in a git repository, using 'dev' tag"
            echo "dev"
        else
            echo "$GIT_SHA"
        fi
    else
        print_warn "Git not found, using 'dev' tag"
        echo "dev"
    fi
}

# Main build function
main() {
    print_info "Starting Docker image build for $IMAGE_NAME"
    
    # Get version tag from git
    VERSION_TAG=$(get_git_sha)
    print_info "Using version tag: $VERSION_TAG"
    
    # Construct full image names
    if [ -n "$REGISTRY" ]; then
        IMAGE_FULL="$REGISTRY/$IMAGE_NAME"
    else
        IMAGE_FULL="$IMAGE_NAME"
    fi
    
    IMAGE_VERSION="$IMAGE_FULL:$VERSION_TAG"
    IMAGE_LATEST="$IMAGE_FULL:latest"
    
    print_info "Building image: $IMAGE_VERSION"
    
    # Build Docker image
    if docker build -t "$IMAGE_VERSION" -t "$IMAGE_LATEST" .; then
        print_info "Build successful!"
        print_info "Tagged images:"
        echo "  - $IMAGE_VERSION"
        echo "  - $IMAGE_LATEST"
        
        # Show image size
        IMAGE_SIZE=$(docker images "$IMAGE_FULL" --format "{{.Size}}" | head -n 1)
        print_info "Image size: $IMAGE_SIZE"
        
        # Optional: Push to registry if PUSH=true
        if [ "${PUSH:-false}" = "true" ]; then
            print_info "Pushing images to registry..."
            docker push "$IMAGE_VERSION"
            docker push "$IMAGE_LATEST"
            print_info "Images pushed successfully"
        fi
        
        print_info "Build completed successfully!"
        return 0
    else
        print_error "Build failed!"
        return 1
    fi
}

# Show usage
usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Build Docker image for rust-basic-api

OPTIONS:
    -h, --help          Show this help message
    -p, --push          Push images to registry after build
    -n, --name NAME     Set image name (default: rust-basic-api)
    -r, --registry REG  Set registry prefix (default: none)

ENVIRONMENT VARIABLES:
    IMAGE_NAME          Image name (default: rust-basic-api)
    REGISTRY            Registry prefix (e.g., ghcr.io/myorg)
    PUSH                Set to 'true' to push after build

EXAMPLES:
    # Build locally
    $0

    # Build and push to registry
    $0 --push --registry ghcr.io/5dlabs

    # Build with custom name
    IMAGE_NAME=my-api $0
EOF
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            usage
            exit 0
            ;;
        -p|--push)
            PUSH=true
            shift
            ;;
        -n|--name)
            IMAGE_NAME="$2"
            shift 2
            ;;
        -r|--registry)
            REGISTRY="$2"
            shift 2
            ;;
        *)
            print_error "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

# Run main function
main
