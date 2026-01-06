#!/bin/sh
# agentmap installer script
# Usage: curl -fsSL https://raw.githubusercontent.com/nguyenphutrong/agentmap/main/scripts/install.sh | sh

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

REPO="nguyenphutrong/agentmap"
BINARY_NAME="agentmap"
INSTALL_DIR="${HOME}/.local/bin"

info() {
    printf "${BLUE}info${NC}: %s\n" "$1"
}

success() {
    printf "${GREEN}success${NC}: %s\n" "$1"
}

warn() {
    printf "${YELLOW}warn${NC}: %s\n" "$1"
}

error() {
    printf "${RED}error${NC}: %s\n" "$1" >&2
    exit 1
}

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Darwin*)
            echo "darwin"
            ;;
        Linux*)
            echo "linux"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            error "Windows detected. Please use PowerShell installer or download manually from GitHub releases."
            ;;
        *)
            error "Unsupported operating system: $(uname -s)"
            ;;
    esac
}

# Detect architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)
            echo "x86_64"
            ;;
        arm64|aarch64)
            echo "aarch64"
            ;;
        *)
            error "Unsupported architecture: $(uname -m)"
            ;;
    esac
}

# Get latest release version
get_latest_version() {
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
    else
        error "Neither curl nor wget found. Please install one of them."
    fi
}

# Download file
download() {
    url="$1"
    output="$2"
    
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$url" -o "$output"
    elif command -v wget >/dev/null 2>&1; then
        wget -q "$url" -O "$output"
    else
        error "Neither curl nor wget found. Please install one of them."
    fi
}

# Main installation
main() {
    printf "\n"
    printf "${GREEN}  ▄▀█ █▀▀ █▀▀ █▄ █ ▀█▀ █▀▄▀█ ▄▀█ █▀█${NC}\n"
    printf "${GREEN}  █▀█ █▄█ ██▄ █ ▀█  █  █ ▀ █ █▀█ █▀▀${NC}\n"
    printf "\n"
    printf "  Prepare codebases for AI agents\n"
    printf "\n"

    OS=$(detect_os)
    ARCH=$(detect_arch)
    
    info "Detected OS: ${OS}"
    info "Detected architecture: ${ARCH}"

    info "Fetching latest version..."
    VERSION=$(get_latest_version)
    
    if [ -z "$VERSION" ]; then
        error "Failed to get latest version. Please check your internet connection."
    fi
    
    info "Latest version: ${VERSION}"

    # Construct download URL
    FILENAME="${BINARY_NAME}-${OS}-${ARCH}.tar.gz"
    DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${FILENAME}"

    info "Downloading ${FILENAME}..."
    
    # Create temp directory
    TMP_DIR=$(mktemp -d)
    trap 'rm -rf "$TMP_DIR"' EXIT

    download "$DOWNLOAD_URL" "${TMP_DIR}/${FILENAME}"

    info "Extracting..."
    tar -xzf "${TMP_DIR}/${FILENAME}" -C "$TMP_DIR"

    # Create install directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"

    info "Installing to ${INSTALL_DIR}..."
    mv "${TMP_DIR}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

    # Verify installation
    if [ -x "${INSTALL_DIR}/${BINARY_NAME}" ]; then
        success "agentmap ${VERSION} installed successfully!"
    else
        error "Installation failed"
    fi

    # Check if install dir is in PATH
    case ":$PATH:" in
        *":${INSTALL_DIR}:"*)
            printf "\n"
            info "Run 'agentmap --help' to get started"
            ;;
        *)
            printf "\n"
            warn "${INSTALL_DIR} is not in your PATH"
            printf "\n"
            printf "Add this to your shell config (~/.bashrc, ~/.zshrc, etc.):\n"
            printf "\n"
            printf "  ${YELLOW}export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}\n"
            printf "\n"
            printf "Then restart your shell or run:\n"
            printf "\n"
            printf "  ${YELLOW}source ~/.bashrc${NC}  # or ~/.zshrc\n"
            printf "\n"
            ;;
    esac

    printf "\n"
    success "Done! Happy coding with AI agents 🤖"
    printf "\n"
}

main "$@"
