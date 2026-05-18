#!/bin/bash

# TermiPet Windows Setup Script (for WSL/Unix users)
# This script helps set up the development environment

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[TermiPet]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Print banner
echo -e "${BLUE}"
cat << "EOF"
╔══════════════════════════════════════╗
║     TermiPet Windows Setup Tool      ║
║                                      ║
║  A desktop pet for Windows          ║
╚══════════════════════════════════════╝
EOF
echo -e "${NC}"

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check Node.js
    if command -v node &> /dev/null; then
        NODE_VERSION=$(node --version)
        log_success "Node.js found: $NODE_VERSION"
    else
        log_error "Node.js not found. Please install Node.js 18+"
        exit 1
    fi
    
    # Check Rust
    if command -v rustc &> /dev/null; then
        RUST_VERSION=$(rustc --version)
        log_success "Rust found: $RUST_VERSION"
    else
        log_error "Rust not found. Please install Rust from https://rustup.rs/"
        exit 1
    fi
    
    # Check cargo
    if command -v cargo &> /dev/null; then
        CARGO_VERSION=$(cargo --version)
        log_success "Cargo found: $CARGO_VERSION"
    else
        log_error "Cargo not found"
        exit 1
    fi
}

# Setup function
setup() {
    log_info "Setting up development environment..."
    
    # Install npm dependencies
    log_info "Installing npm dependencies..."
    npm install
    
    # Install Tauri CLI if not present
    if ! command -v cargo-tauri &> /dev/null; then
        log_info "Installing Tauri CLI..."
        cargo install tauri-cli
    fi
    
    # Create necessary directories
    mkdir -p src-tauri/icons
    mkdir -p src/assets/pets
    
    log_success "Setup complete!"
    log_info "You can now run: npm run tauri:dev"
}

# Main
main() {
    check_prerequisites
    setup
}

main "$@"
