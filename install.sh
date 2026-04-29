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
TOTAL_STEPS=7
CURRENT_STEP=0
INSTALL_MODE="stable" 

# Helper functions
print_header() {
    echo -e "${BLUE}=== $1 ===${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

next_step() {
    ((CURRENT_STEP++))
    echo ""
    echo -e "${BLUE}[Step $CURRENT_STEP/$TOTAL_STEPS]${NC} $1"
    echo "$1" >> "$INSTALL_LOG"
}

log_command() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $*" >> "$INSTALL_LOG"
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
    
    next_step "Validating Hyprland setup"
    
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
local -A package_groups=(

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
        print_error "Scripts directory not found at $script_dir"
        exit 1
    fi
    
    cd "$script_dir"
    
    if [ "$INTERACTIVE" = false ]; then
        print_warning "Running cargo install in non-interactive mode..."
    else
        print_warning "Installing aurora scripts (this may take a few minutes)..."
    fi
    
    cargo install --path . || {
        print_error "Failed to install Rust scripts"
        log_command "ERROR: cargo install failed"
        exit 1
    }
    
    log_command "Successfully installed Rust scripts"
    print_success "Rust scripts installed successfully to ~/.cargo/bin"
    
    cd - > /dev/null
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
    cp -rv "$config_src"/* "$config_dest/" 2>&1 | grep -v '^' || true
    
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
    ║      Aurora Uninstall Script          ║
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

Examples:
  ./install.sh                    # Interactive installation
  ./install.sh --dry-run          # Preview what will be installed
  ./install.sh --non-interactive  # Automated installation
  ./install.sh --uninstall        # Remove Aurora and restore backups

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
    
    log_command "Aurora Installation Started"
    log_command "Script location: $SCRIPT_DIR"
    log_command "Interactive mode: $INTERACTIVE"
    log_command "Dry-run mode: $DRY_RUN"
    
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
    check_existing_install
    create_directories
    check_dependencies
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
