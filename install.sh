#!/bin/bash

# Hamster VCS Installation Script

set -e

echo "🐹 Hamster VCS Installer"
echo "======================="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed!"
    echo ""
    echo "Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    exit 1
fi

echo "✅ Rust is installed ($(rustc --version))"
echo ""

# Build the project
echo "🔨 Building Hamster VCS..."
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    
    # Offer to install
    echo "The 'ham' binary is now available at:"
    echo "  $(pwd)/target/release/ham"
    echo ""
    
    read -p "Would you like to install 'ham' to ~/.cargo/bin? (y/n) " -n 1 -r
    echo ""
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cargo install --path .
        echo ""
        echo "✅ Hamster installed successfully!"
        echo ""
        echo "You can now use 'ham' from anywhere:"
        echo "  ham init"
        echo "  ham add ."
        echo "  ham commit -m 'message'"
        echo ""
        echo "Make sure ~/.cargo/bin is in your PATH!"
    else
        echo ""
        echo "You can run Hamster using:"
        echo "  ./target/release/ham"
        echo ""
        echo "Or add it to your PATH manually."
    fi
    
    echo ""
    echo "📚 Documentation:"
    echo "  README.md          - Full documentation"
    echo "  QUICK_REFERENCE.md - Command cheat sheet"
    echo "  EXAMPLES.md        - Usage examples"
    echo "  ARCHITECTURE.md    - Technical details"
    echo ""
    echo "🎉 Happy version controlling with Hamster! 🐹"
else
    echo ""
    echo "❌ Build failed!"
    exit 1
fi
