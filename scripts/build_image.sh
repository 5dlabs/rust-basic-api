#!/bin/bash
# Build script for Docker image with git-based versioning

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
IMAGE_NAME="${IMAGE_NAME:-rust-basic-api}"
REGISTRY="${REGISTRY:-}"

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

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    print_error "Docker is not installed or not in PATH"
    exit 1
fi

# Get git commit SHA for tagging
if command -v git &> /dev/null && git rev-parse --git-dir > /dev/null 2>&1; then
    GIT_SHA=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
    GIT_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
    print_info "Building from git branch: $GIT_BRANCH (SHA: $GIT_SHA)"
else
    GIT_SHA="latest"
    print_warning "Git not available, using 'latest' tag"
fi

# Determine tags
if [ -n "$REGISTRY" ]; then
    IMAGE_TAG="$REGISTRY/$IMAGE_NAME"
else
    IMAGE_TAG="$IMAGE_NAME"
fi

print_info "Image name: $IMAGE_TAG"
print_info "Building Docker image..."

# Build the Docker image
docker build \
    --build-arg BUILD_DATE="$(date -u +'%Y-%m-%dT%H:%M:%SZ')" \
    --build-arg VCS_REF="$GIT_SHA" \
    -t "$IMAGE_TAG:$GIT_SHA" \
    -t "$IMAGE_TAG:latest" \
    .

# Verify the build
if [ $? -eq 0 ]; then
    print_info "✓ Docker image built successfully"
    print_info "Tagged as: $IMAGE_TAG:$GIT_SHA"
    print_info "Tagged as: $IMAGE_TAG:latest"
    
    # Display image information
    print_info "Image details:"
    docker images "$IMAGE_TAG" --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}\t{{.CreatedAt}}"
    
    # Optional: Push to registry if PUSH_IMAGE=true
    if [ "$PUSH_IMAGE" = "true" ] && [ -n "$REGISTRY" ]; then
        print_info "Pushing image to registry..."
        docker push "$IMAGE_TAG:$GIT_SHA"
        docker push "$IMAGE_TAG:latest"
        print_info "✓ Image pushed to registry"
    fi
else
    print_error "Docker build failed"
    exit 1
fi

print_info "Build complete!"
