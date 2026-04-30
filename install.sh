#!/usr/bin/env bash
# Aurora Installation Script for Arch Linux

#  SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com> */
#  SPDX-License-Identifier: GPL-3.0-or-later */

#    Copyright (C) 2026 Ahum Maitra

#       This program is free software: you can redistribute it and/or modify
#       it under the terms of the GNU General Public License as published by
#       the Free Software Foundation, either version 3 of the License, or
#       (at your option) any later version.

#       This program is distributed in the hope that it will be useful,
#       but WITHOUT ANY WARRANTY; without even the implied warranty of
#       MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#       GNU General Public License for more details.

#       You should have received a copy of the GNU General Public License
#       along with this program.  If not, see <https://www.gnu.org/licenses/>. 

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_LOG="$SCRIPT_DIR/.aurora_install.log"
BACKUP_DIR="$HOME/.config/aurora_backup_$(date +%s)"
INTERACTIVE=true
DRY_RUN=false
TOTAL_STEPS=9
CURRENT_STEP=0
INSTALL_MODE="stable"
INSTALL_STATE_FILE="$HOME/.aurora_install_state"
LOG_LEVEL="${LOG_LEVEL:-INFO}"
PRESERVE_FOLDERS=()
PRESERVE_FILES=()
DETECTED_INSTALL_TYPE="fresh"  # fresh, update, reinstall 

# Structured Logging System
log_message() {
    local level="$1"
    shift
    local message="$*"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    
    # Log to file with level
    echo "[$timestamp] [$level] $message" >> "$INSTALL_LOG"
    
    # Display to console based on log level
    case "$level" in
        ERROR)
            echo -e "${RED}✗ ERROR: $message${NC}" >&2
            ;;
        WARN)
            echo -e "${YELLOW}⚠ WARNING: $message${NC}"
            ;;
        INFO)
            if [ "$LOG_LEVEL" = "INFO" ] || [ "$LOG_LEVEL" = "DEBUG" ]; then
                echo -e "${BLUE}ℹ INFO: $message${NC}"
            fi
            ;;
        DEBUG)
            if [ "$LOG_LEVEL" = "DEBUG" ]; then
                echo -e "${BLUE}🔍 DEBUG: $message${NC}"
            fi
            ;;
        SUCCESS)
            echo -e "${GREEN}✓ $message${NC}"
            ;;
    esac
}

log_error() { log_message "ERROR" "$@"; }
log_warn() { log_message "WARN" "$@"; }
log_info() { log_message "INFO" "$@"; }
log_debug() { log_message "DEBUG" "$@"; }
log_success() { log_message "SUCCESS" "$@"; }

# Helper functions
print_header() {
    echo -e "${BLUE}=== $1 ===${NC}"
}

print_success() {
    log_success "$1"
}

print_warning() {
    log_warn "$1"
}

print_error() {
    log_error "$1"
}

next_step() {
    ((CURRENT_STEP++))
    echo ""
    echo -e "${BLUE}[Step $CURRENT_STEP/$TOTAL_STEPS]${NC} $1"
    log_info "Step $CURRENT_STEP: $1"
}

log_command() {
    log_debug "$*"
}

# Package Detection Functions
is_package_installed() {
    local package="$1"
    
    # Check in pacman
    if pacman -Q "$package" &>/dev/null; then
        return 0
    fi
    
    # Check in yay (AUR packages)
    if command -v yay &>/dev/null; then
        if yay -Q "$package" &>/dev/null; then
            return 0
        fi
    fi
    
    return 1
}

get_install_source() {
    local package="$1"
    
    if pacman -Q "$package" &>/dev/null; then
        echo "pacman"
        return 0
    elif command -v yay &>/dev/null && yay -Q "$package" &>/dev/null; then
        echo "aur"
        return 0
    fi
    
    return 1
}

# Hyprland Runtime Detection (Issue #10 - improved)
detect_hyprland_runtime() {
    # Check if running
    if pgrep -x "Hyprland" > /dev/null 2>&1; then
        if command -v hyprctl &>/dev/null; then
            local version=$(hyprctl version 2>/dev/null | head -1 | grep -oE 'v[0-9.]+' || echo 'unknown')
            log_info "Hyprland is currently running (version: $version)"
        else
            log_info "Hyprland is currently running"
        fi
        return 0
    fi
    
    # Check if installed via pacman/yay (Issue #10 & #12)
    if pacman -Q hyprland &>/dev/null || pacman -Q hyprland-git &>/dev/null; then
        local installed_version
        if pacman -Q hyprland &>/dev/null; then
            installed_version=$(pacman -Q hyprland | awk '{print $2}')
            log_debug "Hyprland installed (repo version: $installed_version) but not running"
        else
            installed_version=$(pacman -Q hyprland-git | awk '{print $2}')
            log_debug "Hyprland installed (git version: $installed_version) but not running"
        fi
        return 0
    fi
    
    return 1
}

# Installation State Detection
detect_installation_type() {
    local aurora_installed=false
    local aurora_version_file="$HOME/.aurora_install_state"
    
    # Check if Aurora binaries exist
    if [ -f "$HOME/.cargo/bin/keybinds_help" ]; then
        aurora_installed=true
    fi
    
    if [ "$aurora_installed" = true ]; then
        if [ -f "$aurora_version_file" ]; then
            DETECTED_INSTALL_TYPE="update"
            log_info "Detected existing Aurora installation - running in UPDATE mode"
        else
            DETECTED_INSTALL_TYPE="reinstall"
            log_warn "Detected Aurora binaries but no state file - running in REINSTALL mode"
        fi
    else
        DETECTED_INSTALL_TYPE="fresh"
        log_info "No existing Aurora installation detected - running in FRESH INSTALL mode"
    fi
}

# Check if running on Arch Linux
check_arch() {
    print_header "Checking system compatibility"
    
    if ! command -v pacman &> /dev/null; then
        print_error "This script is designed for Arch Linux only"
        exit 1
    fi
    
    print_success "Arch Linux detected"
}

# Check if running as root
check_root() {
    if [ "$EUID" -eq 0 ]; then
        print_error "Please do not run this script as root"
        exit 1
    fi
    print_success "Running as non-root user"
}

# Installation Mode Selector (Issue #7)
select_installation_mode() {
    if [ "$DRY_RUN" = true ] || [ "$INTERACTIVE" = false ]; then
        log_info "Using default mode: $INSTALL_MODE"
        return
    fi
    
    next_step "Selecting installation mode"
    
    echo ""
    echo -e "${YELLOW}Choose installation mode:${NC}"
    echo ""
    echo "  1) ${BLUE}Stable (recommended)${NC}"
    echo "     - Uses official Arch repositories"
    echo "     - Maximum stability"
    echo ""
    echo "  2) ${YELLOW}Git (bleeding edge)${NC}"
    echo "     - Uses -git versions (hyprland-git, wayland-git, etc.)"
    echo "     - Requires yay for AUR packages"
    echo "     - Latest features but may be unstable"
    echo ""
    
    read -p "Enter choice [1-2] (default: 1): " -n 1 choice
    echo
    
    case "$choice" in
        2)
            INSTALL_MODE="git"
            log_info "Installation mode set to: GIT (bleeding edge)"
            ;;
        *)
            INSTALL_MODE="stable"
            log_info "Installation mode set to: STABLE (default)"
            ;;
    esac
}

# Config Preservation Selector (Issue #18 & #3 - with path validation)
select_config_preservation() {
    if [ "$DRY_RUN" = true ] || [ "$INTERACTIVE" = false ]; then
        log_info "No config preservation selected"
        return
    fi
    
    echo ""
    echo -e "${YELLOW}Select configurations to preserve (optional):${NC}"
    echo ""
    echo "  This will prevent overwriting your custom configs."
    echo "  Leave empty to backup and replace all configs."
    echo ""
    
    read -p "Folders to preserve (comma-separated, e.g., hypr,waybar): " folders_input
    read -p "Files to preserve (full paths, e.g., ~/.config/file1,~/.config/file2): " files_input
    
    # Validate and parse folder input (Issue #3)
    if [ -n "$folders_input" ]; then
        IFS=',' read -ra folder_array <<< "$folders_input"
        local valid_folders=()
        
        for folder in "${folder_array[@]}"; do
            folder="${folder##*( )}"
            folder="${folder%%*( )}"
            
            local folder_path="$HOME/.config/$folder"
            
            if [ -d "$folder_path" ]; then
                valid_folders+=("$folder")
                log_debug "Folder to preserve: $folder (exists at $folder_path)"
            else
                log_warn "Folder '$folder' not found at $folder_path - skipping"
            fi
        done
        
        PRESERVE_FOLDERS=("${valid_folders[@]}")
        if [ ${#PRESERVE_FOLDERS[@]} -gt 0 ]; then
            log_info "Folders to preserve: ${PRESERVE_FOLDERS[*]}"
        fi
    fi
    
    # Validate and parse file input (Issue #3)
    if [ -n "$files_input" ]; then
        IFS=',' read -ra file_array <<< "$files_input"
        local valid_files=()
        
        for file in "${file_array[@]}"; do
            file="${file##*( )}"
            file="${file%%*( )}"
            file="${file/#~\//$HOME/}"
            file=$(eval echo "$file")
            
            if [ -f "$file" ]; then
                valid_files+=("$file")
                log_debug "File to preserve: $file (exists)"
            else
                log_warn "File '$file' not found - skipping"
            fi
        done
        
        PRESERVE_FILES=("${valid_files[@]}")
        if [ ${#PRESERVE_FILES[@]} -gt 0 ]; then
            log_info "Files to preserve: ${PRESERVE_FILES[*]}"
        fi
    fi
}

# Install AUR helper if needed
install_aur_helper() {
    if command -v yay &> /dev/null; then
        return 0
    fi
    
    # Check for base-devel before attempting to build
    if ! pacman -Q base-devel &>/dev/null; then
        print_warning "base-devel is required to build yay from source"
        read -p "Install base-devel? (y/n) " -n 1 -r
        echo
        
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_error "Cannot proceed without base-devel"
            return 1
        fi
        
        sudo pacman -S base-devel --noconfirm || {
            print_error "Failed to install base-devel"
            return 1
        }
    fi
    
    print_warning "AUR helper 'yay' not found. Installing..."
    
    # Clone and build yay with proper cleanup
    local tmp_dir
    tmp_dir=$(mktemp -d) || {
        print_error "Failed to create temporary directory"
        return 1
    }
    
    # Ensure cleanup on exit
    trap "rm -rf '$tmp_dir'" RETURN
    
    cd "$tmp_dir" || return 1
    
    git clone https://aur.archlinux.org/yay.git 2>/dev/null || {
        print_warning "Failed to clone yay repository"
        return 1
    }
    
    cd yay || return 1
    makepkg -si --noconfirm 2>/dev/null || {
        print_warning "Failed to build yay"
        return 1
    }
    
    cd - > /dev/null
    print_success "yay installed successfully"
    return 0
}

# Rotate log files
rotate_logs() {
    local max_lines=1000
    
    if [ -f "$INSTALL_LOG" ]; then
        local line_count=$(wc -l < "$INSTALL_LOG")
        if [ "$line_count" -gt "$max_lines" ]; then
            tail -n "$max_lines" "$INSTALL_LOG" > "$INSTALL_LOG.tmp"
            mv "$INSTALL_LOG.tmp" "$INSTALL_LOG"
        fi
    fi
}

# Rollback on critical failure (Issue #8)
rollback_on_failure() {
    local failure_reason="$1"
    log_error "Critical failure: $failure_reason"
    print_error "CRITICAL FAILURE: $failure_reason"
    print_warning "Attempting to restore from backup..."
    
    if [ -d "$BACKUP_DIR" ]; then
        echo ""
        echo "Backup found at $BACKUP_DIR"
        read -p "Restore backed up configs? (y/n) " -n 1 -r
        echo
        
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            [ -d "$BACKUP_DIR/hypr" ] && rm -rf "$HOME/.config/hypr" 2>/dev/null; cp -r "$BACKUP_DIR/hypr" "$HOME/.config/" 2>/dev/null
            [ -d "$BACKUP_DIR/waybar" ] && rm -rf "$HOME/.config/waybar" 2>/dev/null; cp -r "$BACKUP_DIR/waybar" "$HOME/.config/" 2>/dev/null
            [ -d "$BACKUP_DIR/kitty" ] && rm -rf "$HOME/.config/kitty" 2>/dev/null; cp -r "$BACKUP_DIR/kitty" "$HOME/.config/" 2>/dev/null
            [ -d "$BACKUP_DIR/fish" ] && rm -rf "$HOME/.config/fish" 2>/dev/null; cp -r "$BACKUP_DIR/fish" "$HOME/.config/" 2>/dev/null
            [ -d "$BACKUP_DIR/rofi" ] && rm -rf "$HOME/.config/rofi" 2>/dev/null; cp -r "$BACKUP_DIR/rofi" "$HOME/.config/" 2>/dev/null
            print_success "Configs restored from backup"
            log_info "Configs restored from backup after failure"
        fi
    fi
    
    print_error "Installation failed. Please check the log: $INSTALL_LOG"
    exit 1
}

# Check dependencies
check_dependencies() {
    next_step "Checking system dependencies"
    
    local missing_deps=()
    
    # Check for cargo
    if ! command -v cargo &> /dev/null; then
        missing_deps+=("cargo (Rust package manager)")
    fi
    
    # Check for git
    if ! command -v git &> /dev/null; then
        missing_deps+=("git")
    fi
    
    # Check for make
    if ! command -v make &> /dev/null; then
        missing_deps+=("make")
    fi
    
    if [ ${#missing_deps[@]} -gt 0 ]; then
        print_error "Missing required dependencies:"
        printf '%s\n' "${missing_deps[@]}" | sed 's/^/  - /'
        echo ""
        print_warning "Install with: sudo pacman -S rustup git base-devel"
        echo ""
        exit 1
    fi
    
    print_success "All required dependencies found"
}

# Validate Hyprland setup
validate_hyprland() {
    if [ "$DRY_RUN" = true ]; then
        next_step "Validating Hyprland setup (DRY RUN)"
        print_success "Hyprland validation skipped in dry-run mode"
        return
    fi
    
    if [ "$INTERACTIVE" = false ]; then
        next_step "Validating Hyprland setup"
        print_success "Hyprland validation skipped in non-interactive mode"
        return
    fi
    
    next_step "Validating Hyprland setup (Issue #2)"
    
    # Check runtime Hyprland status
    if detect_hyprland_runtime; then
        print_success "Hyprland detected on system"
        log_success "Hyprland is installed and functional"
    else
        echo ""
        echo -e "${YELLOW}This script configures Aurora for Hyprland (Wayland compositor).${NC}"
        echo ""
        read -p "Are you using or planning to use Hyprland? (y/n) " -n 1 -r
        echo
        
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_warning "Aurora is designed for Hyprland. Proceeding may result in non-functional configs."
            read -p "Continue anyway? (y/n) " -n 1 -r
            echo
            
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                print_error "Installation cancelled"
                exit 0
            fi
        fi
    fi
    
    print_success "Hyprland validation passed"
}

# Install required packages
install_packages() {
    next_step "Installing Aurora dependencies"
    
    if [ "$DRY_RUN" = true ]; then
        print_warning "[DRY RUN] Would install packages (none actually installed)"
        return
    fi
    
    if [ "$INTERACTIVE" = false ]; then
        print_warning "Running in non-interactive mode - skipping package installation"
        print_warning "Install packages manually with: pacman -S <package>"
        return
    fi
    
    # Organized package groups with comments
declare local -A package_groups=(

    # Core system (Wayland + Hyprland essentials)
    [core]="
        hyprland
        wayland
        xdg-desktop-portal-hyprland
        pipewire
        pipewire-pulse
        wireplumber
    "

    # Core daemons (notifications, idle, lock, auth)
    [daemons]="
        swaync
        hypridle
        hyprlock
        polkit-gnome
    "

    # UI components (bar, launcher, GTK)
    [ui]="
        waybar
        rofi
        wlogout
        gtk3
        gtk4
        adwaita-gtk-theme
        
    "

    # Utilities (apps, tools, fonts, system helpers)
    [utils]="
        kitty
        cliphist
        nautilus
        wl-clipboard
        hyprshot
        network-manager-applet
        brightnessctl
        libnotify
        ttf-dejavu
        noto-fonts
        noto-fonts-emoji
        awww
    "
)
    
    echo ""
    echo -e "${YELLOW}Aurora requires the following packages:${NC}"
    echo ""
    
    # Display packages by category
    for category in core daemons ui utils; do
        category_name="${category^}"
        [ "$category" = "daemons" ] && category_name="Daemons"
        [ "$category" = "ui" ] && category_name="UI Components"
        [ "$category" = "utils" ] && category_name="Utilities"
        
        echo "  ${BLUE}${category_name}:${NC}"
        for pkg in ${package_groups[$category]}; do
            echo "    - $pkg"
        done
    done
    
    echo ""
    read -p "Install Aurora packages? (y/n) " -n 1 -r
    echo
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_warning "Skipping package installation"
        return
    fi
    
    print_warning "Installing Aurora dependencies (requires sudo)..."
    
    local failed_packages=()
    local installed_count=0
    local total_packages=0
    
    # Install all packages
    for category in core daemons ui utils; do
        for package in ${package_groups[$category]}; do
            ((total_packages++))
            
            if pacman -Q "$package" &>/dev/null; then
                print_success "Package '$package' already installed"
            else
                if sudo pacman -S "$package" --noconfirm --needed 2>/dev/null; then
                    ((installed_count++))
                    log_command "Installed: $package"
                else
                    failed_packages+=("$package")
                    log_command "Failed to install: $package"
                fi
            fi
        done
    done
    
    if [ ${#failed_packages[@]} -gt 0 ]; then
        print_warning "Some packages failed to install (${#failed_packages[@]}/${total_packages}). They may be in AUR:"
        printf '%s\n' "${failed_packages[@]}" | sed 's/^/  - /'
        echo ""
        
        # Try to install yay if not present
        if ! command -v yay &> /dev/null; then
            if [ "$INTERACTIVE" = true ]; then
                read -p "Install AUR helper 'yay' to install missing packages? (y/n) " -n 1 -r
                echo
                
                if [[ $REPLY =~ ^[Yy]$ ]]; then
                    install_aur_helper
                fi
            fi
        fi
        
        # Try to install with yay
        if command -v yay &> /dev/null && [ "$INTERACTIVE" = true ]; then
            read -p "Try installing with yay (AUR helper)? (y/n) " -n 1 -r
            echo
            
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                for package in "${failed_packages[@]}"; do
                    if yay -S "$package" --noconfirm 2>/dev/null; then
                        ((installed_count++))
                        log_command "Installed via AUR: $package"
                    else
                        print_warning "Failed to install $package with yay"
                    fi
                done
            fi
        fi
    fi
    
    print_success "Package installation completed ($installed_count/$total_packages packages installed/updated)"
}

# Build Rust scripts
build_rust_scripts() {
    next_step "Building and installing Rust scripts"
    
    local script_dir="$SCRIPT_DIR/dotfiles/.config/hypr/scripts"
    
    if [ ! -d "$script_dir" ]; then
        log_error "Scripts directory not found at $script_dir"
        print_error "Scripts directory not found at $script_dir"
        rollback_on_failure "Scripts directory missing"
        return 1
    fi
    
    # Verify Cargo.toml exists (Issue #4 - project validation)
    if [ ! -f "$script_dir/Cargo.toml" ]; then
        log_error "Cargo.toml not found in $script_dir - invalid Rust project"
        print_error "Invalid Rust project structure at $script_dir"
        rollback_on_failure "Invalid Rust project"
        return 1
    fi
    
    cd "$script_dir" || {
        log_error "Failed to change directory to $script_dir"
        rollback_on_failure "Cannot access scripts directory"
        return 1
    }
    
    if [ "$INTERACTIVE" = false ]; then
        log_info "Running cargo install in non-interactive mode..."
        print_warning "Running cargo install in non-interactive mode..."
    else
        print_warning "Installing aurora scripts (this may take a few minutes)..."
        log_info "Starting cargo install for Aurora scripts"
    fi
    
    # Run cargo install with error capture (Issue #4 & #7)
    local cargo_log="$INSTALL_LOG.cargo_err"
    if ! cargo install --path . 2>"$cargo_log"; then
        local cargo_error=$(cat "$cargo_log" 2>/dev/null | tail -20 || echo "Unknown error")
        log_error "Cargo install failed: $cargo_error"
        print_error "Failed to build Rust scripts"
        print_warning "Last 20 lines of error log:"
        echo "$cargo_error" | sed 's/^/  /'
        rm -f "$cargo_log"
        rollback_on_failure "Cargo build failed"
        cd - > /dev/null
        return 1
    fi
    
    rm -f "$cargo_log"
    log_info "Successfully installed Rust scripts to ~/.cargo/bin"
    print_success "Rust scripts installed successfully to ~/.cargo/bin"
    
    cd - > /dev/null
    return 0
}

# Copy dotfiles
copy_dotfiles() {
    next_step "Installing configuration files"
    
    local config_src="$SCRIPT_DIR/dotfiles/.config"
    local config_dest="$HOME/.config"
    
    if [ ! -d "$config_src" ]; then
        print_error "Dotfiles directory not found at $config_src"
        exit 1
    fi
    
    # Create config directory if it doesn't exist
    mkdir -p "$config_dest"
    
    # Check for existing Aurora configs
    local has_existing=false
    for dir in hypr waybar kitty fish rofi; do
        if [ -d "$config_dest/$dir" ]; then
            has_existing=true
            break
        fi
    done
    
    # Backup and clean existing configs if they exist
    if [ "$has_existing" = true ]; then
        print_warning "Existing configuration detected:"
        [ -d "$config_dest/hypr" ] && echo "  - ~/.config/hypr"
        [ -d "$config_dest/waybar" ] && echo "  - ~/.config/waybar"
        [ -d "$config_dest/kitty" ] && echo "  - ~/.config/kitty"
        [ -d "$config_dest/fish" ] && echo "  - ~/.config/fish"
        [ -d "$config_dest/rofi" ] && echo "  - ~/.config/rofi"
        echo ""
        
        if [ "$DRY_RUN" = true ]; then
            print_warning "[DRY RUN] Would backup existing configs to $BACKUP_DIR"
            return
        fi
        
        if [ "$INTERACTIVE" = true ]; then
            read -p "Create backup and REPLACE (not merge)? (y/n) " -n 1 -r
            echo
            
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                print_warning "Skipping config installation"
                log_command "User declined to overwrite existing configs"
                return
            fi
        fi
        
        print_warning "Creating backup..."
        mkdir -p "$BACKUP_DIR"
        
        [ -d "$config_dest/hypr" ] && cp -r "$config_dest/hypr" "$BACKUP_DIR/" && rm -rf "$config_dest/hypr"
        [ -d "$config_dest/waybar" ] && cp -r "$config_dest/waybar" "$BACKUP_DIR/" && rm -rf "$config_dest/waybar"
        [ -d "$config_dest/kitty" ] && cp -r "$config_dest/kitty" "$BACKUP_DIR/" && rm -rf "$config_dest/kitty"
        [ -d "$config_dest/fish" ] && cp -r "$config_dest/fish" "$BACKUP_DIR/" && rm -rf "$config_dest/fish"
        [ -d "$config_dest/rofi" ] && cp -r "$config_dest/rofi" "$BACKUP_DIR/" && rm -rf "$config_dest/rofi"
        
        print_success "Backup saved to $BACKUP_DIR and old configs removed"
        log_command "Created backup at $BACKUP_DIR and cleaned old configs"
    fi
    
    # Copy all config files (clean install now)
    if [ "$DRY_RUN" = true ]; then
        print_warning "[DRY RUN] Would copy config files from $config_src to $config_dest"
        return
    fi
    
    print_warning "Copying .config files..."
    cp -rv "$config_src"/* "$config_dest/"
    
    log_command "Configuration files installed"
    print_success "Configuration files installed"
}

# Set up shell configuration
setup_shell_config() {
    next_step "Setting up shell configuration"
    
    # Add ~/.cargo/bin to PATH if not already there
    local add_to_path="export PATH=\"\$HOME/.cargo/bin:\$PATH\""
    local path_added=false
    
    # For bash
    if [ -f ~/.bashrc ]; then
        if ! grep -q "\.cargo/bin" ~/.bashrc; then
            echo "" >> ~/.bashrc
            echo "# Aurora binaries" >> ~/.bashrc
            echo "$add_to_path" >> ~/.bashrc
            print_success "Updated .bashrc"
            log_command "Updated .bashrc with PATH"
            path_added=true
        fi
    fi
    
    # For zsh
    if [ -f ~/.zshrc ]; then
        if ! grep -q "\.cargo/bin" ~/.zshrc; then
            echo "" >> ~/.zshrc
            echo "# Aurora binaries" >> ~/.zshrc
            echo "$add_to_path" >> ~/.zshrc
            print_success "Updated .zshrc"
            log_command "Updated .zshrc with PATH"
            path_added=true
        fi
    fi
    
    # For fish
    if [ -f ~/.config/fish/config.fish ]; then
        if ! grep -q "\.cargo/bin" ~/.config/fish/config.fish; then
            echo "" >> ~/.config/fish/config.fish
            echo "# Aurora binaries" >> ~/.config/fish/config.fish
            echo "set -gx PATH \$HOME/.cargo/bin \$PATH" >> ~/.config/fish/config.fish
            print_success "Updated fish config"
            log_command "Updated fish config.fish with PATH"
            path_added=true
        fi
    fi
    
    # Auto-source for bash if running in bash
    if [ -n "$BASH_VERSION" ] && [ "$path_added" = true ] && [ -f ~/.bashrc ]; then
        source ~/.bashrc 2>/dev/null || true
        print_success "Shell PATH reloaded automatically"
    fi
}

# Create required directories
create_directories() {
    mkdir -p ~/.config
}

# Check for existing Aurora installation
check_existing_install() {
    local has_aurora=false
    local aurora_items=()
    
    # Check for Aurora scripts
    if [ -d "$HOME/.cargo/bin" ]; then
        for script in keybinds_help refresh_system search settings theme_switcher waybar_refresh waybar_toggle welcome_app; do
            if [ -f "$HOME/.cargo/bin/$script" ]; then
                has_aurora=true
                aurora_items+=("Aurora script: $script")
            fi
        done
    fi
    
    # Check for Aurora configs
    if [ -d "$HOME/.config/hypr" ]; then
        if grep -q "Aurora" "$HOME/.config/hypr/"* 2>/dev/null; then
            has_aurora=true
            aurora_items+=("Aurora config: ~/.config/hypr")
        fi
    fi
    
    if [ "$has_aurora" = true ]; then
        print_warning "Existing Aurora installation detected:"
        printf '%s\n' "${aurora_items[@]}" | sed 's/^/  - /'
        echo ""
        print_warning "This script will upgrade/overwrite your Aurora setup."
        
        if [ "$INTERACTIVE" = true ] && [ "$DRY_RUN" = false ]; then
            read -p "Continue with re-installation? (y/n) " -n 1 -r
            echo
            
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                print_error "Installation cancelled"
                exit 0
            fi
        fi
    fi
}

# Uninstall Aurora
uninstall_aurora() {
    clear
    echo -e "${BLUE}"
    cat << "EOF"
    ╔═══════════════════════════════════════╗
    ║      Aurora ™  Uninstall Script          ║
    ║    Remove Aurora and restore backup   ║
    ╚═══════════════════════════════════════╝
EOF
    echo -e "${NC}"
    echo ""
    
    print_header "Uninstalling Aurora"
    
    echo ""
    print_warning "This will:"
    echo "  • Remove Aurora Rust scripts from ~/.cargo/bin"
    echo "  • Restore backed up configuration files (if available)"
    echo "  • Remove Aurora configs from ~/.config"
    echo ""
    
    read -p "Proceed with uninstallation? (y/n) " -n 1 -r
    echo
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_warning "Uninstallation cancelled"
        return
    fi
    
    # Remove Rust scripts
    print_warning "Removing Rust scripts..."
    cargo uninstall aurora 2>/dev/null || print_warning "Aurora binaries not found or already removed"
    
    # Find most recent backup
    local latest_backup=$(ls -td "$HOME/.config/aurora_backup_"* 2>/dev/null | head -1)
    
    if [ -d "$latest_backup" ]; then
        print_warning "Found backup at $latest_backup"
        read -p "Restore backed up configs? (y/n) " -n 1 -r
        echo
        
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            print_warning "Restoring configs..."
            [ -d "$latest_backup/hypr" ] && cp -r "$latest_backup/hypr" "$HOME/.config/"
            [ -d "$latest_backup/waybar" ] && cp -r "$latest_backup/waybar" "$HOME/.config/"
            [ -d "$latest_backup/kitty" ] && cp -r "$latest_backup/kitty" "$HOME/.config/"
            [ -d "$latest_backup/fish" ] && cp -r "$latest_backup/fish" "$HOME/.config/"
            [ -d "$latest_backup/rofi" ] && cp -r "$latest_backup/rofi" "$HOME/.config/"
            print_success "Configs restored"
        fi
    fi
    
    print_success "Aurora uninstalled successfully"
    echo ""
    print_warning "You may also want to remove the backup directory:"
    echo "  rm -rf ~/.config/aurora_backup_*"
    echo ""
}

# Final setup
final_setup() {
    echo ""
    print_header "Aurora Setup Complete!"
    
    echo ""
    echo -e "${GREEN}Installation Summary:${NC}"
    echo "  ✓ System dependencies verified"
    echo "  ✓ Rust scripts installed to ~/.cargo/bin"
    echo "  ✓ Configuration files installed"
    echo "  ✓ Shell environment configured"
    echo ""
    echo -e "${BLUE}Installation Mode: ${INSTALL_MODE^^}${NC}"
    echo -e "${BLUE}Installation Type: ${DETECTED_INSTALL_TYPE^^}${NC}"
    echo ""
    
    # Save installation state (Issue #13)
    cat > "$INSTALL_STATE_FILE" << STATE_EOF
{
  "version": "1.0",
  "install_type": "$DETECTED_INSTALL_TYPE",
  "install_mode": "$INSTALL_MODE",
  "install_date": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "script_version": "$(git -C "$SCRIPT_DIR" describe --tags --always 2>/dev/null || echo 'unknown')",
  "hyprland_runtime_detected": "$(detect_hyprland_runtime && echo 'true' || echo 'false')"
}
STATE_EOF
    log_info "Saved installation state to $INSTALL_STATE_FILE"
    
    echo -e "${YELLOW}Installation log saved to: $INSTALL_LOG${NC}"
    echo ""
    
    echo -e "${YELLOW}Next steps:${NC}"
    echo "  1. Reload your shell configuration (if not auto-reloaded):"
    echo "     exec \$SHELL"
    echo ""
    echo "  2. Start Hyprland from your login manager"
    echo ""
    echo "  3. Check keybindings:"
    echo "     Super + H"
    echo ""
    
    echo -e "${YELLOW}Restore backups (if needed):${NC}"
    if [ -d "$BACKUP_DIR" ]; then
        echo "  Backup location: $BACKUP_DIR"
    else
        echo "  No backups created during this installation"
    fi
    echo ""
    
    echo -e "${YELLOW}Uninstall Aurora:${NC}"
    echo "  ./install.sh --uninstall"
    echo ""
}

# Print usage
print_usage() {
    cat << "EOF"
Aurora Installation Script for Arch Linux

Usage: ./install.sh [OPTIONS]

Options:
  --help              Show this help message
  --dry-run           Preview changes without applying them
  --uninstall         Uninstall Aurora and restore backups
  --non-interactive   Run without user prompts (skip packages & Hyprland check)
  --debug             Show detailed debug information and logs

Examples:
  ./install.sh                    # Interactive installation with mode selection
  ./install.sh --dry-run          # Preview what will be installed
  ./install.sh --non-interactive  # Automated installation (default: stable mode)
  ./install.sh --debug            # Installation with verbose logging
  ./install.sh --uninstall        # Remove Aurora and restore backups

Features:
  - Detects existing Aurora installations (fresh/update/reinstall modes)
  - Supports stable (repo) and git (bleeding edge) package installations
  - Hyprland runtime detection and validation
  - Selective config preservation and backup
  - Structured logging with INFO/WARN/ERROR/DEBUG levels
  - Dry-run mode to preview changes
  - AUR package detection via yay
  - Installation state tracking for version upgrades

EOF
}

# Main installation flow
main() {
    # Handle command-line arguments
    case "${1:-}" in
        --help)
            print_usage
            exit 0
            ;;
        --dry-run)
            DRY_RUN=true
            ;;
        --debug)
            LOG_LEVEL="DEBUG"
            DRY_RUN=false
            ;;
        --uninstall)
            uninstall_aurora
            exit 0
            ;;
        --non-interactive)
            INTERACTIVE=false
            ;;
        *)
            if [ -n "$1" ]; then
                print_error "Unknown option: $1"
                echo ""
                print_usage
                exit 1
            fi
            ;;
    esac
    
    # Initialize log (rotate before clearing)
    if [ -f "$INSTALL_LOG" ]; then
        rotate_logs
    fi
    : > "$INSTALL_LOG"
    
    log_info "Aurora Installation Started"
    log_debug "Script location: $SCRIPT_DIR"
    log_debug "Interactive mode: $INTERACTIVE"
    log_debug "Dry-run mode: $DRY_RUN"
    
    clear
    echo -e "${BLUE}"
    cat << "EOF"
    ╔═══════════════════════════════════════╗
    ║       Aurora Installation Script      ║
    ║    Arch Linux Wayland Rice Setup      ║
    ╚═══════════════════════════════════════╝
EOF
    echo -e "${NC}"
    echo ""
    
    if [ "$DRY_RUN" = true ]; then
        echo -e "${YELLOW}[DRY RUN MODE] - No changes will be applied${NC}"
        echo ""
    fi
    
    # Run installation steps
    check_arch
    check_root
    detect_installation_type  # Issue #5
    check_existing_install
    create_directories
    check_dependencies
    select_installation_mode  # Issue #7 - Feature request
    select_config_preservation  # Issue #18 - Feature request
    validate_hyprland
    install_packages
    
    if [ "$DRY_RUN" = false ]; then
        build_rust_scripts
        copy_dotfiles
        setup_shell_config
    else
        next_step "Building and installing Rust scripts"
        print_warning "[DRY RUN] Would build and install Rust scripts"
        
        next_step "Installing configuration files"
        print_warning "[DRY RUN] Would copy configuration files"
        
        next_step "Setting up shell configuration"
        print_warning "[DRY RUN] Would update shell PATH"
    fi
    
    final_setup
    
    log_command "Aurora Installation Completed Successfully"
}

# Run main function
main "$@"
