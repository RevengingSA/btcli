#!/usr/bin/env bash
# Copyright 2026 S.A. (@snoware)
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

set -euo pipefail

# Get project version from Cargo.toml
PROJECT_VERSION=$(grep "^version" Cargo.toml | head -n1 | cut -d'"' -f2)

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Detect package manager
detect_package_manager() {
    if [[ -n "${TERMUX_VERSION:-}" ]] || [[ -d "${PREFIX:-}" && -d "$HOME/bin" ]]; then
        PACKAGE_MANAGER="termux"
        print_info "Detected Termux environment"
    elif command_exists apt || command_exists dpkg; then
        PACKAGE_MANAGER="apt"
        print_info "Detected Debian/Ubuntu package manager (apt/dpkg)"
    elif command_exists yum || command_exists rpm; then
        PACKAGE_MANAGER="yum"
        print_info "Detected Red Hat/CentOS package manager (yum/rpm)"
    elif command_exists pacman; then
        PACKAGE_MANAGER="pacman"
        print_info "Detected Arch Linux package manager (pacman)"
    elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
        PACKAGE_MANAGER="msys2"
        print_info "Detected MSYS2 environment"
    else
        PACKAGE_MANAGER="unknown"
        print_warning "Could not detect package manager, some dependencies might be missing"
    fi
}

# Install dependencies based on detected package manager
install_dependencies() {
    local deps_missing=()
    
    # Check for required tools
    ! command_exists cargo && deps_missing+=("rustc/cargo")
    ! command_exists tar && deps_missing+=("tar")
    ! command_exists zip && deps_missing+=("zip")
    ! command_exists upx && deps_missing+=("upx")
    ! command_exists zstd && deps_missing+=("zstd")
    
    if [[ ${#deps_missing[@]} -gt 0 ]]; then
        print_warning "Missing required dependencies: ${deps_missing[*]}"
        
        if [[ "$PACKAGE_MANAGER" != "unknown" ]]; then
            print_info "Checking missing dependencies..."
            
            case $PACKAGE_MANAGER in
                termux)
                    print_warning "Missing packages: ${deps_missing[*]}"
                    print_info "Please install manually using: pkg install rust cargo tar zip upx zstd"
                    ;;
                apt)
                    print_warning "Missing packages: ${deps_missing[*]}"
                    print_info "Please install manually using: apt install rustc cargo tar zip upx zstd"
                    ;;
                yum)
                    print_warning "Missing packages: ${deps_missing[*]}"
                    print_info "Please install manually using: yum install rust cargo tar zip upx zstd"
                    ;;
                pacman)
                    print_warning "Missing packages: ${deps_missing[*]}"
                    print_info "Please install manually using: pacman -S rust tar zip upx zstd"
                    ;;
                msys2)
                    print_info "Detected MSYS2 environment, checking packages:"
                    # In MSYS2, we typically don't automatically install packages
                    # Instead, we list what's missing and let the user install manually
                    local missing_pkgs=()
                    ! command_exists cargo && missing_pkgs+=("mingw-w64-x86_64-rust")
                    ! command_exists upx && missing_pkgs+=("mingw-w64-x86_64-upx")
                    ! command_exists zip && missing_pkgs+=("zip")
                    ! command_exists tar && missing_pkgs+=("tar")
                    ! command_exists zstd && missing_pkgs+=("zstd")
                    
                    if [[ ${#missing_pkgs[@]} -gt 0 ]]; then
                        print_warning "Missing packages in MSYS2: ${missing_pkgs[*]}"
                        print_info "Please install manually using: pacman -S ${missing_pkgs[*]}"
                        print_info "Example: pacman -S mingw-w64-x86_64-rust mingw-w64-x86_64-upx zip zstd"
                    else
                        print_success "All required dependencies are installed in MSYS2! ~(=^‥^)~"
                    fi
                    ;;
            esac
        else
            print_error "Please install missing dependencies manually."
            exit 1
        fi
    else
        print_info "All required dependencies are installed! ~(=^‥^)~"
    fi
}

# Clean build artifacts
clean_build() {
    print_info "Cleaning build artifacts..."
    for dir in btcli-*; do
        if [[ -d "$dir" ]]; then
            rm -rf "$dir"
        fi
    done
    print_success "Removed btcli-* directories"
    
    if command_exists cargo; then
        cargo clean
        print_success "Executed cargo clean"
    else
        print_error "cargo not found, skipping cargo clean"
    fi
}

# Build the project
build_project() {
    local compression_formats=()
    local compression_enabled=false
    
    # Parse compression formats if provided - handle multiple argument formats
    local args=("$@")
    local i=0
    local arg_count=${#args[@]}
    
    while [[ $i -lt $arg_count ]]; do
        case "${args[$i]}" in
            --format|--formats|-f)
                local j=$((i + 1))
                if [[ $j -lt $arg_count ]]; then
                    IFS=',' read -ra temp_formats <<< "${args[$j]}"
                    # Filter out invalid formats
                    for fmt in "${temp_formats[@]}"; do
                        case $fmt in
                            zip)
                                compression_formats+=("$fmt")
                                compression_enabled=true
                                ;;
                            zst|tar.zst|zstd|tar.gz|gz|tar.xz|xz)
                                print_warning "Format '$fmt' is not supported, skipping..."
                                ;;
                            *)
                                print_warning "Invalid format '$fmt', skipping..."
                                ;;
                        esac
                    done
                    i=$((i + 2))  # Skip both flag and value
                    continue
                else
                    i=$((i + 1))  # Skip just the flag
                    continue
                fi
                ;;
        esac
        i=$((i + 1))
    done
    
    print_info "Building project version $PROJECT_VERSION for current platform... (¬‿¬)"
    
    # Build for current platform
    cargo build
    print_success "Debug build completed for current platform"
    
    cargo build --release
    print_success "Release build completed for current platform"
    
    # Process built files
    if [[ "$compression_enabled" == "true" ]]; then
        process_built_files "$compression_enabled" "${compression_formats[@]}"
    else
        process_built_files "$compression_enabled"
    fi
    
    print_success "Build and packaging completed successfully! ~(^-^)~"
}

# Package the project for distribution
package_project() {
    print_info "Packaging project version $PROJECT_VERSION for distribution... (¬‿¬)"
    
    # Determine current platform
    local os_arch_part=""
    if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
        if [[ "$(uname -m)" == "x86_64" ]]; then
            os_arch_part="windows-x86_64"
        elif [[ "$(uname -m)" == "aarch64" ]]; then
            os_arch_part="windows-aarch64"
        else
            os_arch_part="windows-$(uname -m)"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        if [[ "$(uname -m)" == "x86_64" ]]; then
            os_arch_part="macos-x86_64"
        elif [[ "$(uname -m)" == "arm64" ]]; then
            os_arch_part="macos-arm64"
        else
            os_arch_part="macos-$(uname -m)"
        fi
    else
        if [[ "$(uname -m)" == "x86_64" ]]; then
            os_arch_part="linux-x86_64"
        elif [[ "$(uname -m)" == "aarch64" ]]; then
            os_arch_part="linux-aarch64"
        elif [[ "$(uname -m)" == "armv7l" ]]; then
            os_arch_part="linux-armv7"
        elif [[ "$(uname -m)" == "riscv64" ]]; then
            os_arch_part="linux-riscv64"
        else
            os_arch_part="linux-$(uname -m)"
        fi
    fi
    
    # First, build the release binary
    cargo build --release
    
    # Create package using cargo package
    # Use --allow-dirty to allow packaging with uncommitted changes
    cargo package --allow-dirty
    
    if [[ $? -eq 0 ]]; then
        print_success "Package created successfully!"
        
        # Create a unified output directory
        local output_dir="btcli-$PROJECT_VERSION-$os_arch_part-package"
        mkdir -p "$output_dir"
        
        # Copy the .crate file and rename it to have source suffix
        cp "target/package/btcli-$PROJECT_VERSION.crate" "$output_dir/btcli-$PROJECT_VERSION-$os_arch_part-source.crate"
        
        # Copy the release executable to the output directory
        local release_executable=""
        local release_executable_name=""
        if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
            release_executable="target/release/btcli.exe"
            release_executable_name="btcli-$PROJECT_VERSION-$os_arch_part-release.exe"
        else
            release_executable="target/release/btcli"
            release_executable_name="btcli-$PROJECT_VERSION-$os_arch_part-release"
        fi
        
        if [[ -f "$release_executable" ]]; then
            # Copy original release executable
            cp "$release_executable" "$output_dir/$release_executable_name"
            
            # If it's Windows and UPX is available, create UPX compressed version
            if [[ "$os_arch_part" =~ .*windows.* ]]; then
                if command_exists upx; then
                    print_info "Compressing executable with UPX..."
                    upx -9 "$output_dir/$release_executable_name" 2>/dev/null || print_warning "UPX compression may have failed, continuing anyway"
                    # Rename the UPX compressed file
                    mv "$output_dir/$release_executable_name" "$output_dir/btcli-$PROJECT_VERSION-$os_arch_part-release_upx.exe"
                    print_success "Created UPX compressed executable: btcli-$PROJECT_VERSION-$os_arch_part-release_upx.exe"
                else
                    print_warning "UPX not found, skipping executable compression"
                    # Just rename the original release executable to match the desired naming
                    mv "$output_dir/$release_executable_name" "$output_dir/btcli-$PROJECT_VERSION-$os_arch_part-release.exe"
                fi
            fi
        else
            print_warning "Release executable not found at $release_executable"
        fi
        
        # Create debug package with all debug artifacts
        local debug_zip_name="btcli-$PROJECT_VERSION-$os_arch_part-debug.zip"
        cd target/debug
        if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
            zip -9 -r "../package/$debug_zip_name" btcli.exe btcli.d libbtcli_lib.d libbtcli_lib.rlib *.pdb 2>/dev/null || print_warning "Some debug files may be missing"
        else
            zip -9 -r "../package/$debug_zip_name" btcli btcli.d libbtcli_lib.d libbtcli_lib.rlib 2>/dev/null || print_warning "Some debug files may be missing"
        fi
        cd ../..
        
        # Move the debug zip to our output directory
        if [[ -f "target/package/$debug_zip_name" ]]; then
            mv "target/package/$debug_zip_name" "$output_dir/"
        fi
        
        # Copy LICENSE file to output directory
        if [[ -f "LICENSE" ]]; then
            cp "LICENSE" "$output_dir/"
        else
            print_warning "LICENSE file not found in project root"
        fi
        
        print_info "All packages are in $output_dir/"
        print_info "Package contents:"
        ls -la "$output_dir/"
        
        print_success "Packaging completed successfully!"
    else
        print_error "Package creation failed!"
        exit 1
    fi
}

# Process built files for current platform
process_built_files() {
    local compression_enabled=${1:-false}
    shift
    local compression_formats=()
    if [[ "$compression_enabled" == "true" ]]; then
        compression_formats=("$@")
    fi
    local os_arch_part=""
    
    # Determine current platform
    if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
        if [[ "$(uname -m)" == "x86_64" ]]; then
            os_arch_part="windows-x86_64"
        elif [[ "$(uname -m)" == "aarch64" ]]; then
            os_arch_part="windows-aarch64"
        else
            os_arch_part="windows-$(uname -m)"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        if [[ "$(uname -m)" == "x86_64" ]]; then
            os_arch_part="macos-x86_64"
        elif [[ "$(uname -m)" == "arm64" ]]; then
            os_arch_part="macos-arm64"
        else
            os_arch_part="macos-$(uname -m)"
        fi
    else
        if [[ "$(uname -m)" == "x86_64" ]]; then
            os_arch_part="linux-x86_64"
        elif [[ "$(uname -m)" == "aarch64" ]]; then
            os_arch_part="linux-aarch64"
        elif [[ "$(uname -m)" == "armv7l" ]]; then
            os_arch_part="linux-armv7"
        elif [[ "$(uname -m)" == "riscv64" ]]; then
            os_arch_part="linux-riscv64"
        else
            os_arch_part="linux-$(uname -m)"
        fi
    fi
    
    print_info "Processing built files for current platform (OS/Arch: $os_arch_part)"
    
    # Store the original directory to calculate correct paths
    local original_dir="$(pwd)"
    
    # Create package directory (only normal mode now, allin removed)
    local package_dir="btcli-$PROJECT_VERSION-$os_arch_part"
    
    mkdir -p "$package_dir"
    cd "$package_dir"
    
    # Copy the executable based on OS
    if [[ "$os_arch_part" =~ .*windows.* ]]; then
        cp "$original_dir/target/debug/btcli.exe" "./btcli-$PROJECT_VERSION-$os_arch_part-debug.exe"
        cp "$original_dir/target/release/btcli.exe" "./btcli-$PROJECT_VERSION-$os_arch_part-release.exe"
        EXECUTABLE_NAME_DEBUG="btcli-$PROJECT_VERSION-$os_arch_part-debug.exe"
        EXECUTABLE_NAME_RELEASE="btcli-$PROJECT_VERSION-$os_arch_part-release.exe"
    else
        cp "$original_dir/target/debug/btcli" "./btcli-$PROJECT_VERSION-$os_arch_part-debug"
        cp "$original_dir/target/release/btcli" "./btcli-$PROJECT_VERSION-$os_arch_part-release"
        EXECUTABLE_NAME_DEBUG="btcli-$PROJECT_VERSION-$os_arch_part-debug"
        EXECUTABLE_NAME_RELEASE="btcli-$PROJECT_VERSION-$os_arch_part-release"
    fi
    
    # Normal mode: create debug and release directories
    mkdir -p debug
    mkdir -p release
    
    # Copy executables to respective directories
    cp "$EXECUTABLE_NAME_DEBUG" debug/
    cp "$EXECUTABLE_NAME_RELEASE" release/
    
    # Copy only top-level debug artifacts, not the entire deps directory
    if [[ -f "$original_dir/target/debug/btcli.d" ]]; then
        cp "$original_dir/target/debug/btcli.d" debug/
    fi
    if [[ -f "$original_dir/target/debug/libbtcli_lib.d" ]]; then
        cp "$original_dir/target/debug/libbtcli_lib.d" debug/
    fi
    if [[ -f "$original_dir/target/debug/libbtcli_lib.rlib" ]]; then
        cp "$original_dir/target/debug/libbtcli_lib.rlib" debug/
    fi
    if [[ -f "$original_dir/target/debug/btcli.exe" ]]; then
        cp "$original_dir/target/debug/btcli.exe" debug/
    fi
    
    # Copy release artifacts
    if [[ -f "$original_dir/target/release/btcli.d" ]]; then
        cp "$original_dir/target/release/btcli.d" release/
    fi
    if [[ -f "$original_dir/target/release/libbtcli_lib.d" ]]; then
        cp "$original_dir/target/release/libbtcli_lib.d" release/
    fi
    if [[ -f "$original_dir/target/release/libbtcli_lib.rlib" ]]; then
        cp "$original_dir/target/release/libbtcli_lib.rlib" release/
    fi
    if [[ -f "$original_dir/target/release/btcli" ]]; then
        cp "$original_dir/target/release/btcli" release/
    fi
    if [[ -f "$original_dir/target/release/btcli.exe" ]]; then
        cp "$original_dir/target/release/btcli.exe" release/
    fi
    
    # Compress based on specified formats or default to zip
    local compression_formats_var=("zip")
    if [[ "$compression_enabled" == "true" ]]; then
        # Filter out invalid formats
        local valid_formats=()
        for fmt in "${compression_formats[@]}"; do
            case $fmt in
                zip)
                    valid_formats+=("$fmt")
                    ;;
                zst|tar.zst|zstd|gz|tar.gz|xz|tar.xz)
                    print_warning "Format '$fmt' is not supported, skipping..."
                    ;;
                *)
                    print_warning "Invalid format '$fmt', skipping..."
                    ;;
            esac
        done
        
        # Use only valid formats, fallback to defaults if none are valid
        if [[ ${#valid_formats[@]} -gt 0 ]]; then
            compression_formats_var=("${valid_formats[@]}")
        else
            print_warning "No valid formats specified, using defaults: zip"
        fi
    fi
    
    print_info "Creating archives with formats: ${compression_formats_var[*]} ^_^"
    
    for format in "${compression_formats_var[@]}"; do
        case $format in
            zip)
                if command_exists zip; then
                    # Use maximum compression level (-9)
                    zip -9 -r "btcli-$PROJECT_VERSION-$os_arch_part.zip" debug/ release/
                    print_success "Created btcli-$PROJECT_VERSION-$os_arch_part.zip"
                else
                    print_error "zip not available, skipping .zip format"
                fi
                ;;
            *)
                print_warning "Unknown format: $format, skipping"
                ;;
        esac
    done
    
    cd ../
    
    # If it's Windows, compress executable with UPX if available
    if [[ "$os_arch_part" =~ .*windows.* ]]; then
        if [[ -f "target/release/btcli.exe" ]]; then
            cd target/release
            print_info "Compressing executable with UPX..."
            if command_exists upx; then
                upx -9 btcli.exe 2>/dev/null || print_warning "UPX compression may have failed, continuing anyway"
            else
                print_warning "UPX not found, skipping executable compression"
            fi
            cd ../../
        fi
    fi
}

# Show help message
show_help() {
    cat << EOF
btcli build script ~(=^‥^=)~

USAGE:
    build.sh [OPTIONS] [COMMAND]

COMMANDS:
    build       Build the project (default)
    package     Package the project for distribution
    clean       Clean build artifacts
    check       Check dependencies
    help        Show this help message

OPTIONS:
    --format, -f FORMAT[,FORMAT...]  Specify compression formats (zip only)
                                    Default: zip
                                    
EXAMPLES:
    ./build.sh                                  # Build for current platform with default zip format
    ./build.sh clean                           # Clean build artifacts
    ./build.sh build --format zip              # Build with zip format
    ./build.sh package                         # Package project for distribution
EOF
}

# Main function
main() {
    local command="${1:-build}"
    
    # Handle help option specifically
    if [[ "$#" -eq 0 || "$1" == "--help" || "$1" == "-h" || "$1" == "\?" ]]; then
        show_help
        exit 0
    fi
    
    case $command in
        build|'' )
            detect_package_manager
            install_dependencies
            # Extract remaining arguments after the command
            shift  # Remove the command argument
            build_project "$@"
            ;;
        package)
            detect_package_manager
            install_dependencies
            package_project
            ;;
        clean)
            detect_package_manager
            install_dependencies
            clean_build
            ;;
        check)
            detect_package_manager
            install_dependencies
            ;;
        help|h|\?)
            show_help
            ;;
        *)
            print_error "Unknown command: $command"
            show_help
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"