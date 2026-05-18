# TermiPet Windows Build Script
# Requires: PowerShell 5.1+ or PowerShell Core

param(
    [switch]$Dev,
    [switch]$Release,
    [switch]$Clean,
    [switch]$Setup
)

$ErrorActionPreference = "Stop"

# Colors
$Green = "`e[32m"
$Blue = "`e[34m"
$Yellow = "`e[33m"
$Red = "`e[31m"
$Reset = "`e[0m"

function Write-Status($message) {
    Write-Host "${Blue}[TermiPet]${Reset} $message"
}

function Write-Success($message) {
    Write-Host "${Green}[SUCCESS]${Reset} $message"
}

function Write-Warning($message) {
    Write-Host "${Yellow}[WARNING]${Reset} $message"
}

function Write-Error($message) {
    Write-Host "${Red}[ERROR]${Reset} $message"
}

# Check prerequisites
function Test-Prerequisites {
    Write-Status "Checking prerequisites..."
    
    # Check Node.js
    try {
        $nodeVersion = node --version
        Write-Success "Node.js found: $nodeVersion"
    } catch {
        Write-Error "Node.js not found. Please install Node.js 18+ from https://nodejs.org/"
        exit 1
    }
    
    # Check Rust
    try {
        $rustVersion = rustc --version
        Write-Success "Rust found: $rustVersion"
    } catch {
        Write-Error "Rust not found. Please install Rust from https://rustup.rs/"
        exit 1
    }
    
    # Check Tauri CLI
    try {
        $tauriVersion = cargo tauri --version
        Write-Success "Tauri CLI found: $tauriVersion"
    } catch {
        Write-Warning "Tauri CLI not found. Installing..."
        cargo install tauri-cli
    }
}

# Setup development environment
function Invoke-Setup {
    Write-Status "Setting up development environment..."
    
    # Install npm dependencies
    Write-Status "Installing npm dependencies..."
    npm install
    
    # Create necessary directories
    $dirs = @(
        "src-tauri/icons",
        "src/assets/pets"
    )
    
    foreach ($dir in $dirs) {
        if (!(Test-Path $dir)) {
            New-Item -ItemType Directory -Path $dir -Force | Out-Null
            Write-Success "Created directory: $dir"
        }
    }
    
    Write-Success "Setup complete!"
}

# Clean build artifacts
function Invoke-Clean {
    Write-Status "Cleaning build artifacts..."
    
    $paths = @(
        "node_modules",
        "dist",
        "src-tauri/target",
        "src-tauri/Cargo.lock"
    )
    
    foreach ($path in $paths) {
        if (Test-Path $path) {
            Remove-Item -Path $path -Recurse -Force
            Write-Status "Removed: $path"
        }
    }
    
    Write-Success "Clean complete!"
}

# Development build
function Invoke-DevBuild {
    Write-Status "Starting development server..."
    cargo tauri dev
}

# Release build
function Invoke-ReleaseBuild {
    Write-Status "Building release version..."
    
    # Build frontend
    Write-Status "Building frontend..."
    npm run build
    
    # Build Tauri
    Write-Status "Building Tauri application..."
    cargo tauri build
    
    $bundlePath = "src-tauri/target/release/bundle"
    if (Test-Path $bundlePath) {
        Write-Success "Build complete! Bundles available at:"
        Get-ChildItem -Path $bundlePath -Recurse -File | ForEach-Object {
            Write-Host "  - $($_.FullName)"
        }
    }
}

# Main execution
Write-Host @"
╔══════════════════════════════════════╗
║     TermiPet Windows Build Tool      ║
║                                      ║
║  A desktop pet for Windows          ║
╚══════════════════════════════════════╝
"@ -ForegroundColor Cyan

if ($Setup) {
    Invoke-Setup
    exit 0
}

if ($Clean) {
    Invoke-Clean
    exit 0
}

# Check prerequisites
Test-Prerequisites

if ($Dev) {
    Invoke-DevBuild
} elseif ($Release) {
    Invoke-ReleaseBuild
} else {
    Write-Host @"

Usage:
  .\Scripts\build.ps1 -Setup    # Setup development environment
  .\Scripts\build.ps1 -Dev      # Start development server
  .\Scripts\build.ps1 -Release  # Build release version
  .\Scripts\build.ps1 -Clean    # Clean build artifacts

Examples:
  # First time setup
  .\Scripts\build.ps1 -Setup

  # Development
  .\Scripts\build.ps1 -Dev

  # Build for distribution
  .\Scripts\build.ps1 -Release
"@
}
