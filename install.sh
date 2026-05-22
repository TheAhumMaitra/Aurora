#!/usr/bin/env bash
# Aurora Installation Script for Arch Linux

#  SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
#  SPDX-License-Identifier: GPL-3.0-or-later

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

set -Eeuo pipefail

# Colors for output
RESET='\033[0m'
RED='\033[1;38;5;203m'
GREEN='\033[1;38;5;120m'
YELLOW='\033[1;38;5;221m'
BLUE='\033[1;38;5;111m'
MAGENTA='\033[1;38;5;213m'
CYAN='\033[1;38;5;159m'
WHITE='\033[1;97m'
DARK='\033[38;5;244m'
BOLD='\033[1m'
DIM='\033[2m'
NC="$RESET"

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_LOG="$HOME/.local/share/Aurora/install.log"
BACKUP_DIR="$HOME/.config/aurora_backup_$(date +%s)"
INTERACTIVE=true
DRY_RUN=false
CURRENT_STEP=0
INSTALL_MODE="stable"
INSTALL_STATE_FILE="$HOME/.aurora_install_state"
LOG_LEVEL="${LOG_LEVEL:-INFO}"
DETECTED_INSTALL_TYPE="fresh" # fresh, update, reinstall
DISCOVERED_BINS=()
SDDM_THEME_STATUS="not-run"
MIN_HOME_FREE_MB="${AURORA_MIN_HOME_FREE_MB:-5120}"

error_handler() {
  local exit_code=$?
  local line_number="$1"

  echo ""
  echo -e "${RED}[ERROR] Exit code: $exit_code${NC}"
  echo -e "${RED}[ERROR] Line: $line_number${NC}"
  printf "${RED}[ERROR] Failed command: %q${NC}\n" "$BASH_COMMAND"
}

trap 'error_handler $LINENO' ERR

# Structured Logging System
log_message() {
  local level="$1"
  shift
  local message="$*"
  local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
  local log_dir

  log_dir="$(dirname "$INSTALL_LOG")"
  mkdir -p "$log_dir"

  # Log to file with level
  echo "[$timestamp] [$level] $message" >>"$INSTALL_LOG"

  # Display to console based on log level
  case "$level" in
  ERROR)
    echo -e "${RED}${BOLD}✗ ERROR${NC} ${WHITE}$message${NC}" >&2
    ;;
  WARN)
    echo -e "${YELLOW}${BOLD}▲ WARN ${NC} ${WHITE}$message${NC}"
    ;;
  INFO)
    if [ "$LOG_LEVEL" = "INFO" ] || [ "$LOG_LEVEL" = "DEBUG" ]; then
      echo -e "${CYAN}${BOLD}• INFO ${NC} ${DARK}$message${NC}"
    fi
    ;;
  DEBUG)
    if [ "$LOG_LEVEL" = "DEBUG" ]; then
      echo -e "${BLUE}${BOLD}◌ DEBUG${NC} ${DARK}$message${NC}"
    fi
    ;;
  SUCCESS)
    echo -e "${GREEN}${BOLD}✓ OK   ${NC} ${WHITE}$message${NC}"
    ;;
  esac
}

log_error() { log_message "ERROR" "$@"; }
log_warn() { log_message "WARN" "$@"; }
log_info() { log_message "INFO" "$@"; }
log_debug() { log_message "DEBUG" "$@"; }
log_success() { log_message "SUCCESS" "$@"; }

# Helper functions
print_rule() {
  echo -e "${DIM}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

print_spacer() {
  echo ""
}

render_banner() {
  print_spacer
echo -e "${MAGENTA}${BOLD}        ▄▄▄    █    ██   ██▀███   ▒█████   ██▀███   ▄▄▄       ${NC}"
echo -e "${MAGENTA}${BOLD}      ▒████▄   ██  ▓██▒ ▓██ ▒ ██▒▒██▒  ██▒▓██ ▒ ██▒ ▒████▄     ${NC}"
echo -e "${BLUE}${BOLD}        ▒██  ▀█▄  ▓██  ▒██ ░▓██ ░▄█ ▒▒██░  ██▒▓██ ░▄█  ▒██  ▀█▄   ${NC}"
echo -e "${CYAN}${BOLD}        ░██▄▄▄▄██ ▓▓█  ░██ ░▒██▀▀█▄  ▒██   ██░▒██▀▀█▄  ░██▄▄▄▄██  ${NC}"
echo -e "${GREEN}${BOLD}        ▓█   ▓██ ▒▒█████▓ ░██▓ ▒██▒░ ████▓▒░░██▓ ▒██▒ ▓█   ▓██▒ ${NC}"
  echo -e "${WHITE}${BOLD}  Arch Linux Hyprland setup, tuned for Aurora${NC}"
  echo -e "${DARK}  Minimal shell noise. Clear steps. Safer install flow.${NC}"
  print_rule
}

print_header() {
  print_spacer
  print_rule
  echo -e "${WHITE}${BOLD}  $1${NC}"
  echo -e "${DARK}  Aurora installer interface${NC}"
  print_rule
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

clear_screen() {
  command -v clear &>/dev/null && clear || true
}

next_step() {
  ((++CURRENT_STEP))
  echo ""
  echo -e "${MAGENTA}${BOLD}◉ Step ${CURRENT_STEP}${NC} ${WHITE}${BOLD}$1${NC}"
  echo -e "${DIM}${CYAN}  Preparing this stage...${NC}"
  log_info "Step $CURRENT_STEP: $1"
}

log_command() {
  log_debug "$*"
}

cargo_bin_in_path() {
  case ":${PATH:-}:" in
  *":$HOME/.cargo/bin:"*) return 0 ;;
  *) return 1 ;;
  esac
}

append_unique() {
  local new_item="$1"
  local existing_item

  for existing_item in "${DISCOVERED_BINS[@]}"; do
    if [ "$existing_item" = "$new_item" ]; then
      return
    fi
  done

  DISCOVERED_BINS+=("$new_item")
}

discover_cargo_binaries() {
  local manifest_dir="$1"
  local manifest="$manifest_dir/Cargo.toml"
  local package_name=""
  local bin_name
  local bin_file
  DISCOVERED_BINS=()

  if [ ! -f "$manifest" ]; then
    return 1
  fi

  if cargo metadata --manifest-path "$manifest" --no-deps --format-version 1 >/dev/null 2>&1; then
    log_debug "Cargo metadata validated for $manifest"
  else
    log_warn "Cargo metadata validation failed for $manifest"
  fi

  package_name="$(awk -F= '
        /^\[package\]/ { in_package=1; next }
        /^\[/ { in_package=0 }
        in_package {
            key=$1
            gsub(/[[:space:]]/, "", key)
            if (key == "name") {
                value=$2
                gsub(/^[[:space:]]*"/, "", value)
                gsub(/".*$/, "", value)
                print value
                exit
            }
        }
    ' "$manifest")"

  if [ -n "$package_name" ] && [ -f "$manifest_dir/src/main.rs" ]; then
    append_unique "$package_name"
  fi

  while IFS= read -r bin_name; do
    [ -n "$bin_name" ] && append_unique "$bin_name"
  done < <(awk -F= '
        /^\[\[bin\]\]/ { in_bin=1; next }
        /^\[/ { in_bin=0 }
        in_bin {
            key=$1
            gsub(/[[:space:]]/, "", key)
            if (key == "name") {
                value=$2
                gsub(/^[[:space:]]*"/, "", value)
                gsub(/".*$/, "", value)
                print value
            }
        }
    ' "$manifest")

  if [ -d "$manifest_dir/src/bin" ]; then
    while IFS= read -r bin_file; do
      append_unique "$(basename "$bin_file" .rs)"
    done < <(find "$manifest_dir/src/bin" -maxdepth 1 -type f -name '*.rs' | sort)
  fi

  [ ${#DISCOVERED_BINS[@]} -gt 0 ]
}

initialize_logging() {
  # Stream all output to both console and log file.
  exec > >(tee -a "$INSTALL_LOG")
  exec 2>&1
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
  # Primary installation check requested by user: hyprland command exists.
  if command -v hyprland &>/dev/null; then
    local hyprland_path
    hyprland_path="$(command -v hyprland)"
    log_info "Hyprland command found at: $hyprland_path"
    return 0
  fi

  # Check if running
  if pgrep -x "Hyprland" >/dev/null 2>&1; then
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

  if ! command -v pacman &>/dev/null; then
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

check_home_disk_space() {
  next_step "Checking available disk space"

  local disk_target="/home"
  local available_mb

  if [ ! -d "$disk_target" ]; then
    disk_target="$HOME"
  fi

  available_mb="$(df -Pm "$disk_target" 2>/dev/null | awk 'NR==2 {print $4}')"

  if [[ ! "$available_mb" =~ ^[0-9]+$ ]]; then
    print_error "Could not determine free disk space for $disk_target"
    exit 1
  fi

  log_info "Available space on $disk_target: ${available_mb}MB"

  if [ "$available_mb" -lt "$MIN_HOME_FREE_MB" ]; then
    print_error "Low disk space detected"
    echo "  Free up disk space and rerun the installer."
    exit 1
  fi

  print_success "Sufficient disk space available on $disk_target (${available_mb}MB free)"
}

# Installation Mode Selector (Issue #7)
select_installation_mode() {
  if [ "$DRY_RUN" = true ] || [ "$INTERACTIVE" = false ]; then
    log_info "Using default mode: $INSTALL_MODE"
    return
  fi

  next_step "Selecting installation mode"

  echo ""
  echo -e "${MAGENTA}${BOLD}Choose installation mode${NC}"
  print_rule
  echo ""
  echo -e "  ${YELLOW}1)${NC} ${WHITE}${BOLD}Stable${NC} ${GREEN}(recommended)${NC}"
  echo -e "     ${DARK}Uses official Arch repositories${NC}"
  echo -e "     ${DARK}Maximum stability${NC}"
  echo ""
  echo -e "  ${YELLOW}2)${NC} ${WHITE}${BOLD}Git${NC} ${MAGENTA}(bleeding edge)${NC}"
  echo -e "     ${DARK}Uses -git versions (hyprland-git, wayland-git, etc.)${NC}"
  echo -e "     ${DARK}Requires yay for AUR packages${NC}"
  echo -e "     ${DARK}Latest features but may be unstable${NC}"
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

# Install AUR helper if needed
install_aur_helper() {
  if command -v yay &>/dev/null; then
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
  local old_pwd="$PWD"
  local tmp_dir
  tmp_dir=$(mktemp -d) || {
    print_error "Failed to create temporary directory"
    return 1
  }

  cd "$tmp_dir" || {
    rm -rf "$tmp_dir"
    return 1
  }

  git clone https://aur.archlinux.org/yay.git 2>/dev/null || {
    print_warning "Failed to clone yay repository"
    cd "$old_pwd" || true
    rm -rf "$tmp_dir"
    return 1
  }

  cd yay || {
    cd "$old_pwd" || true
    rm -rf "$tmp_dir"
    return 1
  }
  makepkg -si --noconfirm 2>/dev/null || {
    print_warning "Failed to build yay"
    cd "$old_pwd" || true
    rm -rf "$tmp_dir"
    return 1
  }

  cd "$old_pwd" || true
  rm -rf "$tmp_dir"
  print_success "yay installed successfully"
  return 0
}

# Rotate log files
rotate_logs() {
  local max_lines=1000

  if [ -f "$INSTALL_LOG" ]; then
    local line_count=$(wc -l <"$INSTALL_LOG")
    if [ "$line_count" -gt "$max_lines" ]; then
      tail -n "$max_lines" "$INSTALL_LOG" >"$INSTALL_LOG.tmp"
      mv "$INSTALL_LOG.tmp" "$INSTALL_LOG"
    fi
  fi
}

prepare_install_log() {
  local log_dir

  log_dir="$(dirname "$INSTALL_LOG")"
  mkdir -p "$log_dir"

  if [ -f "$INSTALL_LOG" ]; then
    rotate_logs
  fi

  : >"$INSTALL_LOG"
}

copy_hypr_children_without_user() {
  local src_hypr="$1"
  local dest_hypr="$2"
  local item
  local item_name

  [ -d "$src_hypr" ] || return 0
  mkdir -p "$dest_hypr"

  while IFS= read -r -d '' item; do
    item_name="${item##*/}"
    [ "$item_name" = "User" ] && continue
    cp -rfv "$item" "$dest_hypr/"
  done < <(find "$src_hypr" -mindepth 1 -maxdepth 1 -print0)
}

backup_hypr_without_user() {
  local target_hypr="$1"
  local backup_hypr="$2"
  local item
  local item_name

  [ -d "$target_hypr" ] || return 0
  mkdir -p "$backup_hypr"

  while IFS= read -r -d '' item; do
    item_name="${item##*/}"
    [ "$item_name" = "User" ] && continue
    cp -r "$item" "$backup_hypr/"
  done < <(find "$target_hypr" -mindepth 1 -maxdepth 1 -print0)
}

remove_hypr_children_without_user() {
  local target_hypr="$1"
  local item
  local item_name

  [ -d "$target_hypr" ] || return 0

  while IFS= read -r -d '' item; do
    item_name="${item##*/}"
    [ "$item_name" = "User" ] && continue
    rm -rf "$item"
  done < <(find "$target_hypr" -mindepth 1 -maxdepth 1 -print0)
}

restore_config_from_backup() {
  local config_name="$1"
  local backup_root="$2"
  local backup_item="$backup_root/$config_name"
  local target_item="$HOME/.config/$config_name"

  [ -d "$backup_item" ] || return 0

  if [ "$config_name" = "hypr" ]; then
    mkdir -p "$target_item"
    remove_hypr_children_without_user "$target_item"
    copy_hypr_children_without_user "$backup_item" "$target_item"
    return 0
  fi

  rm -rf "$target_item" 2>/dev/null
  cp -r "$backup_item" "$HOME/.config/" 2>/dev/null
}

# Rollback on critical failure (Issue #8)
rollback_on_failure() {
  local failure_reason="$1"
  local config_dir
  log_error "Critical failure: $failure_reason"
  print_error "CRITICAL FAILURE: $failure_reason"
  print_warning "Attempting to restore from backup..."

  if [ -d "$BACKUP_DIR" ]; then
    echo ""
    echo "Backup found at $BACKUP_DIR"
    read -p "Restore backed up configs? (y/n) " -n 1 -r
    echo

    if [[ $REPLY =~ ^[Yy]$ ]]; then
      for config_dir in hypr waybar kitty fish rofi; do
        restore_config_from_backup "$config_dir" "$BACKUP_DIR"
      done
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
  if ! command -v cargo &>/dev/null; then
    missing_deps+=("cargo (Rust package manager)")
  fi

  # Check for git
  if ! command -v git &>/dev/null; then
    missing_deps+=("git")
  fi

  # Check for make
  if ! command -v make &>/dev/null; then
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

  local hyprland_pkg="hyprland"
  local xdp_hyprland_pkg="xdg-desktop-portal-hyprland"
  if [ "$INSTALL_MODE" = "git" ]; then
    hyprland_pkg="hyprland-git"
    xdp_hyprland_pkg="xdg-desktop-portal-hyprland-git"
  fi

  local -A pacman_package_groups=(
    [core]="
            $hyprland_pkg
            $xdp_hyprland_pkg
            pipewire
            pipewire-pulse
            wireplumber
            hyprshutdown
        "
    [daemons]="
            swaync
            hypridle
            hyprlock
            polkit-gnome
        "
    [ui]="
            waybar
            rofi
            wlogout
            gtk3
            gtk4
            qt6-svg
            qt6-virtualkeyboard
            qt6-multimedia
            qt6-multimedia-ffmpeg
        "
    [utils]="
            kitty
            cliphist
            nautilus
            wl-clipboard
            hyprshot
            brightnessctl
            libnotify
            ttf-dejavu
            noto-fonts
            noto-fonts-emoji
            awww
            papirus-icon-theme
            rofi-emoji
            ttf-jetbrains-mono-nerd
            mise
            starship
            neovim
            sddm
            wiremix
            bluetui
            btop
            xcb-util-cursor
        "
    [build]="
            git
            base-devel
            glib2
            uv
            sudo-rs
        "
  )

  local -A yay_package_groups=(
    [aur_extras]="
            nordzy-hyprcursors
            zen-browser-bin
            wifitui-bin
            weathr-bin
            jolt
            leenfetch
            yaru-icon-theme
            yaru-gtk-theme
        "
  )

  echo ""
  echo -e "${MAGENTA}${BOLD}Aurora requires the following packages${NC}"
  print_rule
  echo ""

  # Display packages grouped by manager and category
  echo -e "  ${CYAN}${BOLD}pacman packages${NC}"
  for category in core daemons ui utils build; do
    category_name="${category^}"
    [ "$category" = "daemons" ] && category_name="Daemons"
    [ "$category" = "ui" ] && category_name="UI Components"
    [ "$category" = "utils" ] && category_name="Utilities"
    [ "$category" = "build" ] && category_name="Build & Toolchain"
    echo -e "    ${YELLOW}${BOLD}${category_name}:${NC}"
    for pkg in ${pacman_package_groups[$category]}; do
      echo -e "      ${GREEN}•${NC} ${WHITE}$pkg${NC}"
    done
  done

  echo -e "  ${CYAN}${BOLD}yay (AUR) packages${NC}"
  for category in aur_extras; do
    echo -e "    ${YELLOW}${BOLD}AUR Extras:${NC}"
    for pkg in ${yay_package_groups[$category]}; do
      echo -e "      ${MAGENTA}•${NC} ${WHITE}$pkg${NC}"
    done
  done

  echo ""
  read -p "Install Aurora packages? (y/n) " -n 1 -r
  echo

  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_warning "Skipping package installation"
    return
  fi

  print_warning "Updating Arch Linux package databases and installed packages..."
  if ! sudo pacman -Syu --noconfirm; then
    print_error "Failed to update Arch Linux. Resolve the pacman error and rerun the installer."
    exit 1
  fi

  print_warning "Installing Aurora dependencies (requires sudo)..."

  local failed_packages=()
  local installed_count=0
  local total_packages=0

  # Install pacman packages
  for category in core daemons ui utils build; do
    for package in ${pacman_package_groups[$category]}; do
      ((++total_packages))

      if pacman -Q "$package" &>/dev/null; then
        print_success "Package '$package' already installed"
      else
        if sudo pacman -S "$package" --noconfirm --needed 2>/dev/null; then
          ((++installed_count))
          log_command "Installed: $package"
        else
          failed_packages+=("$package")
          log_command "Failed to install: $package"
        fi
      fi
    done
  done

  if [ ${#failed_packages[@]} -gt 0 ]; then
    print_warning "Some pacman packages failed (${#failed_packages[@]}/${total_packages}):"
    printf '%s\n' "${failed_packages[@]}" | sed 's/^/  - /'
    echo ""
  fi

  # Ensure yay exists for AUR package installation.
  if ! command -v yay &>/dev/null; then
    print_warning "AUR helper 'yay' is not installed. Installing it now..."
    if ! install_aur_helper; then
      print_warning "Could not install yay automatically. AUR packages will be skipped."
    fi
  fi

  if command -v yay &>/dev/null; then
    for category in aur_extras; do
      for package in ${yay_package_groups[$category]}; do
        ((++total_packages))
        if pacman -Q "$package" &>/dev/null; then
          print_success "AUR package '$package' already installed"
        else
          if yay -S "$package" --noconfirm --needed 2>/dev/null; then
            ((++installed_count))
            log_command "Installed AUR package: $package"
          else
            print_warning "Failed to install AUR package: $package"
          fi
        fi
      done
    done
  else
    print_warning "Skipping AUR packages because yay is unavailable"
  fi

  print_success "Package installation completed ($installed_count/$total_packages packages installed/updated)"
}

install_sddm_theme() {
  next_step "Installing SDDM astronaut theme"

  local theme_repo="https://github.com/keyitdev/sddm-astronaut-theme.git"
  local theme_dir="/usr/share/sddm/themes/sddm-astronaut-theme"
  local sddm_conf="/etc/sddm.conf"
  local virtualkbd_conf="/etc/sddm.conf.d/virtualkbd.conf"

  if [ "$DRY_RUN" = true ]; then
    SDDM_THEME_STATUS="dry-run"
    print_warning "[DRY RUN] Would clone $theme_repo into $theme_dir"
    print_warning "[DRY RUN] Would copy theme fonts into /usr/share/fonts and refresh font cache"
    print_warning "[DRY RUN] Would write $sddm_conf and $virtualkbd_conf"
    return 0
  fi

  if ! pacman -Q sddm &>/dev/null; then
    SDDM_THEME_STATUS="skipped"
    print_warning "sddm is not installed according to pacman; skipping theme setup"
    log_warn "Skipped SDDM theme installation because sddm package is missing"
    return 0
  fi

  print_warning "Installing SDDM astronaut theme (requires sudo)..."

  sudo mkdir -p /usr/share/sddm/themes
  sudo mkdir -p /etc/sddm.conf.d

  if [ -d "$theme_dir/.git" ]; then
    log_info "Updating existing SDDM astronaut theme checkout"
    sudo git -C "$theme_dir" pull --ff-only
  elif [ -d "$theme_dir" ]; then
    log_warn "Theme directory already exists without git metadata: $theme_dir"
    print_warning "Reusing existing SDDM theme directory at $theme_dir"
  else
    log_info "Cloning SDDM astronaut theme from $theme_repo"
    sudo git clone -b master --depth 1 "$theme_repo" "$theme_dir"
  fi

  if [ -d "$theme_dir/Fonts" ]; then
    sudo mkdir -p /usr/share/fonts
    sudo cp -rf "$theme_dir/Fonts/." /usr/share/fonts/
    if command -v fc-cache &>/dev/null; then
      sudo fc-cache -f /usr/share/fonts
    fi
    log_info "Installed SDDM theme fonts into /usr/share/fonts"
  else
    log_warn "Fonts directory not found in $theme_dir"
  fi

  cat <<'EOF' | sudo tee "$sddm_conf" >/dev/null
[Theme]
Current=sddm-astronaut-theme
EOF

  cat <<'EOF' | sudo tee "$virtualkbd_conf" >/dev/null
[General]
InputMethod=qtvirtualkeyboard
EOF

  sudo systemctl enable sddm.service

  SDDM_THEME_STATUS="configured and enabled"
  print_success "SDDM astronaut theme installed and SDDM enabled at boot"
  log_info "Configured SDDM to use sddm-astronaut-theme with qtvirtualkeyboard and enabled sddm.service"
}

# Build Rust scripts
build_rust_scripts() {
  next_step "Building and installing Rust scripts"

  local script_dir="$SCRIPT_DIR/dotfiles/.config/hypr/scripts"
  local old_pwd="$PWD"

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
    cd "$old_pwd" || true
    return 1
  fi

  rm -f "$cargo_log"
  log_info "Successfully installed Rust scripts to ~/.cargo/bin"
  print_success "Rust scripts installed successfully to ~/.cargo/bin"

  cd "$old_pwd" || true
  return 0
}

install_waytrogen_aurora() {
  next_step "Installing waytrogen-aurora"

  local repo_url="https://github.com/TheAhumMaitra/waytrogen-aurora.git"
  local repo_dir="$HOME/.local/share/Aurora/src/waytrogen-aurora"

  local schema_src="$repo_dir/org.Waytrogen.Waytrogen.gschema.xml"
  local user_schema_dir="$HOME/.local/share/glib-2.0/schemas"

  mkdir -p "$(dirname "$repo_dir")"
  mkdir -p "$user_schema_dir"

  if [ -d "$repo_dir/.git" ]; then
    log_info "Updating existing waytrogen-aurora checkout at $repo_dir"
    git -C "$repo_dir" pull --ff-only
  else
    log_info "Cloning waytrogen-aurora from $repo_url"
    git clone "$repo_url" "$repo_dir"
  fi

  log_info "Installing waytrogen-aurora via cargo"
  cargo install --path "$repo_dir" --locked

  if [ -f "$schema_src" ]; then
    if ! command -v glib-compile-schemas &>/dev/null; then
      print_warning "glib-compile-schemas not found; skipping GLib schema compilation"
      return 0
    fi

    cp -f "$schema_src" "$user_schema_dir/"
    glib-compile-schemas "$user_schema_dir"

    log_success "Installed and compiled user GLib schemas in $user_schema_dir"
  else
    log_warn "No schema file found in waytrogen-aurora repo, skipping schema install"
  fi
}

move_to_backup() {
  local source_path="$1"
  local backup_path="$2"
  local resolved_backup="$backup_path"

  [ -e "$source_path" ] || [ -L "$source_path" ] || return 0

  if [ -e "$resolved_backup" ] || [ -L "$resolved_backup" ]; then
    resolved_backup="${backup_path}.$(date +%s)"
  fi

  mv "$source_path" "$resolved_backup"
  log_info "Moved $source_path to $resolved_backup"
}

setup_lazyvim() {
  next_step "Installing LazyVim starter"

  local nvim_config_dir="$HOME/.config/nvim"
  local nvim_data_dir="$HOME/.local/share/nvim"
  local nvim_state_dir="$HOME/.local/state/nvim"
  local nvim_cache_dir="$HOME/.cache/nvim"

  if [ "$DRY_RUN" = true ]; then
    print_warning "[DRY RUN] Would backup existing Neovim config/data/cache directories"
    print_warning "[DRY RUN] Would clone LazyVim starter into ~/.config/nvim and remove its .git directory"
    return 0
  fi

  if ! command -v git &>/dev/null; then
    print_error "git is required to install LazyVim"
    return 1
  fi

  move_to_backup "$nvim_config_dir" "${nvim_config_dir}.bak"
  move_to_backup "$nvim_data_dir" "${nvim_data_dir}.bak"
  move_to_backup "$nvim_state_dir" "${nvim_state_dir}.bak"
  move_to_backup "$nvim_cache_dir" "${nvim_cache_dir}.bak"

  mkdir -p "$(dirname "$nvim_config_dir")"
  git clone https://github.com/LazyVim/starter "$nvim_config_dir"
  rm -rf "$nvim_config_dir/.git"

  print_success "LazyVim starter installed to ~/.config/nvim"
  log_info "LazyVim starter installed and repository metadata removed"
}

# Copy dotfiles
copy_dotfiles() {
  next_step "Installing configuration files"

  local config_src="$SCRIPT_DIR/dotfiles/.config"
  local config_dest="$HOME/.config"
  local config_dir
  local config_name
  local target_item

  if [ ! -d "$config_src" ]; then
    print_error "Dotfiles directory not found at $config_src"
    exit 1
  fi

  mkdir -p "$config_dest"

  if [ "$DRY_RUN" = true ]; then
    print_warning "[DRY RUN] Would backup existing Aurora configs to $BACKUP_DIR"
    print_warning "[DRY RUN] Would remove existing Aurora config files and directories, preserving ~/.config/hypr/User"
    print_warning "[DRY RUN] Would copy config files from $config_src to $config_dest"
    return
  fi

  print_warning "Forcefully replacing existing Aurora configs..."
  mkdir -p "$BACKUP_DIR"

  while IFS= read -r -d '' config_dir; do
    config_name="${config_dir##*/}"
    target_item="$config_dest/$config_name"

    if [ "$config_name" = "hypr" ]; then
      rm -rf "$BACKUP_DIR/$config_name"
      backup_hypr_without_user "$target_item" "$BACKUP_DIR/$config_name"
      mkdir -p "$target_item"
      remove_hypr_children_without_user "$target_item"
      continue
    fi

    if [ -e "$target_item" ] || [ -L "$target_item" ]; then
      rm -rf "$BACKUP_DIR/$config_name"
      cp -r "$target_item" "$BACKUP_DIR/"
      rm -rf "$target_item"
    fi
  done < <(find "$config_src" -mindepth 1 -maxdepth 1 -print0)

  while IFS= read -r -d '' config_dir; do
    config_name="${config_dir##*/}"

    if [ "$config_name" = "hypr" ]; then
      copy_hypr_children_without_user "$config_dir" "$config_dest/$config_name"
      continue
    fi

    cp -rfv "$config_dir" "$config_dest/"
  done < <(find "$config_src" -mindepth 1 -maxdepth 1 -print0)

  log_command "Configuration files installed"
  print_success "Configuration files installed successfully"
}

# Set up shell configuration
setup_shell_config() {
  next_step "Setting up shell configuration"

  # Add ~/.cargo/bin to PATH if not already there
  local add_to_path="export PATH=\"\$HOME/.cargo/bin:\$PATH\""
  local path_was_missing=false
  local shell_name
  shell_name="$(basename "${SHELL:-}")"

  if ! cargo_bin_in_path; then
    path_was_missing=true
  fi

  # For bash
  if [ -f ~/.bashrc ]; then
    if ! grep -q "\.cargo/bin" ~/.bashrc; then
      echo "" >>~/.bashrc
      echo "# Aurora binaries" >>~/.bashrc
      echo "$add_to_path" >>~/.bashrc
      print_success "Updated .bashrc"
      log_command "Updated .bashrc with PATH"
    fi
  fi

  # For zsh
  if [ -f ~/.zshrc ]; then
    if ! grep -q "\.cargo/bin" ~/.zshrc; then
      echo "" >>~/.zshrc
      echo "# Aurora binaries" >>~/.zshrc
      echo "$add_to_path" >>~/.zshrc
      print_success "Updated .zshrc"
      log_command "Updated .zshrc with PATH"
    fi
  fi

  # For fish
  if [ -f ~/.config/fish/config.fish ]; then
    if ! grep -q "\.cargo/bin" ~/.config/fish/config.fish; then
      echo "" >>~/.config/fish/config.fish
      echo "# Aurora binaries" >>~/.config/fish/config.fish
      echo "set -gx PATH \$HOME/.cargo/bin \$PATH" >>~/.config/fish/config.fish
      print_success "Updated fish config"
      log_command "Updated fish config.fish with PATH"
    fi
  fi

  if [ "$path_was_missing" = true ]; then
    export PATH="$HOME/.cargo/bin:$PATH"
    print_warning "Aurora binaries were added to shell config, but your current terminal may need to reload PATH."
    case "$shell_name" in
    fish)
      echo "  Run: source ~/.config/fish/config.fish"
      ;;
    zsh)
      echo "  Run: source ~/.zshrc"
      ;;
    bash)
      echo "  Run: source ~/.bashrc"
      ;;
    *)
      echo "  Run: exec \$SHELL"
      ;;
    esac
    echo "  Then verify with: command -v <installed-binary>"
  fi
}

verify_installation() {
  next_step "Verifying installation"

  local cargo_bin="$HOME/.cargo/bin"
  local script_dir="$SCRIPT_DIR/dotfiles/.config/hypr/scripts"
  local required_bins=()
  local missing_bins=()
  local bin
  local first_bin=""

  if discover_cargo_binaries "$script_dir"; then
    required_bins=("${DISCOVERED_BINS[@]}")
  else
    print_error "Could not determine Aurora Rust binaries from $script_dir"
    return 1
  fi

  for bin in "${required_bins[@]}"; do
    if [ ! -x "$cargo_bin/$bin" ]; then
      missing_bins+=("$bin")
    fi
  done

  if [ ${#missing_bins[@]} -gt 0 ]; then
    print_error "Missing or non-executable Aurora binaries in ~/.cargo/bin:"
    printf '%s\n' "${missing_bins[@]}" | sed 's/^/  - /'
    return 1
  fi

  print_success "Aurora binaries found in ~/.cargo/bin"
  first_bin="${required_bins[0]}"

  if ! cargo_bin_in_path; then
    print_warning "~/.cargo/bin is not in PATH for this installer process"
  fi

  if command -v "$first_bin" &>/dev/null; then
    print_success "$first_bin is accessible from PATH"
  else
    print_warning "Aurora binaries are installed but not accessible in the current shell"
    echo "  Example installed binary: $cargo_bin/$first_bin"
    echo "  Reload your shell, then run: command -v $first_bin"
  fi
}

wait_for_pacman_settle() {
  local timeout_seconds=60
  local sleep_seconds=2
  local elapsed_seconds=0
  local pacman_lock="/var/lib/pacman/db.lck"

  log_info "Waiting for package-manager processes to settle before replacing sudo"

  while pgrep -x pacman &>/dev/null || pgrep -x yay &>/dev/null || pgrep -x makepkg &>/dev/null || [ -e "$pacman_lock" ]; do
    if [ "$elapsed_seconds" -ge "$timeout_seconds" ]; then
      print_warning "Timed out waiting for pacman/yay to settle; skipping sudo-rs switch"
      log_warn "pacman/yay still active or lock file present after ${timeout_seconds}s"
      return 1
    fi

    sleep "$sleep_seconds"
    elapsed_seconds=$((elapsed_seconds + sleep_seconds))
  done

  return 0
}

switch_to_sudo_rs() {
  next_step "Switching sudo to sudo-rs"

  if [ "$DRY_RUN" = true ]; then
    print_warning "[DRY RUN] Would atomically replace /usr/bin/sudo with a symlink to /usr/bin/sudo-rs"
    return 0
  fi

  if ! wait_for_pacman_settle; then
    return 0
  fi

  if ! pacman -Q sudo-rs &>/dev/null; then
    print_warning "sudo-rs package is not installed according to pacman; skipping sudo switch"
    log_warn "sudo-rs package missing from pacman database"
    return 0
  fi

  if [ ! -x /usr/bin/sudo-rs ]; then
    print_warning "sudo-rs is not installed at /usr/bin/sudo-rs; skipping sudo switch"
    log_warn "sudo-rs binary missing at /usr/bin/sudo-rs"
    return 0
  fi

  if [ -L /usr/bin/sudo ] && [ "$(readlink /usr/bin/sudo)" = "/usr/bin/sudo-rs" ]; then
    print_success "sudo already points to sudo-rs"
    log_info "sudo already linked to sudo-rs"
    return 0
  fi

  if ! sudo /bin/bash -c '
        set -e

        if [ ! -x /usr/bin/sudo-rs ]; then
            echo "sudo-rs binary missing at /usr/bin/sudo-rs" >&2
            exit 1
        fi

        if [ ! -e /usr/bin/sudo-original ]; then
            if [ ! -e /usr/bin/sudo ] && [ ! -L /usr/bin/sudo ]; then
                echo "/usr/bin/sudo does not exist and no backup is present" >&2
                exit 1
            fi

            mv /usr/bin/sudo /usr/bin/sudo-original
        fi

        rm -f /usr/bin/sudo
        ln -s /usr/bin/sudo-rs /usr/bin/sudo
    '; then
    print_error "Failed to link /usr/bin/sudo to /usr/bin/sudo-rs"
    log_error "Could not atomically switch /usr/bin/sudo to /usr/bin/sudo-rs"
    return 1
  fi

  print_success "sudo now points to sudo-rs"
  log_info "Switched /usr/bin/sudo to /usr/bin/sudo-rs"
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
    if grep -R -q --exclude-dir=User "Aurora" "$HOME/.config/hypr" 2>/dev/null; then
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
  clear_screen
  echo -e "${BLUE}"
  cat <<"EOF"
    ╔═══════════════════════════════════════╗
    ║      Aurora ™  Uninstallation Script  ║
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
  local latest_backup=""
  latest_backup="$(find "$HOME/.config" -maxdepth 1 -type d -name 'aurora_backup_*' -printf '%T@ %p\n' 2>/dev/null | sort -nr | awk 'NR == 1 { sub(/^[^ ]+ /, ""); print }')" || true

  if [ -d "$latest_backup" ]; then
    print_warning "Found backup at $latest_backup"
    read -p "Restore backed up configs? (y/n) " -n 1 -r
    echo

    if [[ $REPLY =~ ^[Yy]$ ]]; then
      print_warning "Restoring configs..."
      restore_config_from_backup "hypr" "$latest_backup"
      restore_config_from_backup "waybar" "$latest_backup"
      restore_config_from_backup "kitty" "$latest_backup"
      restore_config_from_backup "fish" "$latest_backup"
      restore_config_from_backup "rofi" "$latest_backup"
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
  echo -e "${MAGENTA}${BOLD}Installation Summary${NC}"
  print_rule
  echo -e "  ${GREEN}✓${NC} ${WHITE}System dependencies verified${NC}"
  echo -e "  ${CYAN}•${NC} ${WHITE}SDDM theme setup:${NC} ${YELLOW}${SDDM_THEME_STATUS}${NC}"
  echo -e "  ${GREEN}✓${NC} ${WHITE}Rust scripts installed to ~/.cargo/bin${NC}"
  echo -e "  ${GREEN}✓${NC} ${WHITE}LazyVim starter installed to ~/.config/nvim${NC}"
  echo -e "  ${GREEN}✓${NC} ${WHITE}Configuration files installed${NC}"
  echo -e "  ${GREEN}✓${NC} ${WHITE}Shell environment configured${NC}"
  print_rule
  echo -e "  ${BLUE}${BOLD}Mode:${NC} ${WHITE}${INSTALL_MODE^^}${NC}"
  echo -e "  ${BLUE}${BOLD}Install Type:${NC} ${WHITE}${DETECTED_INSTALL_TYPE^^}${NC}"
  echo ""

  # Save installation state (Issue #13)
  cat >"$INSTALL_STATE_FILE" <<STATE_EOF
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

  echo -e "${CYAN}${BOLD}Installation log:${NC} ${WHITE}$INSTALL_LOG${NC}"
  echo ""

  if [ "$INTERACTIVE" = true ] && [ "$DRY_RUN" = false ] && [ "${SHELL##*/}" != "fish" ]; then
    echo ""
    print_warning "Aurora is optimized for Fish shell."

    read -p "Change default shell to Fish? (y/n) " -n 1 -r
    echo

    if [[ $REPLY =~ ^[Yy]$ ]]; then
      if command -v fish >/dev/null 2>&1; then
        local fish_path
        fish_path="$(command -v fish)"

        if chsh -s "$fish_path"; then
          print_success "Default shell changed to Fish"
          print_warning "Log out and log back in for changes to apply"
        else
          print_error "Failed to change default shell to Fish"
        fi
      else
        print_error "Fish shell is not installed"
      fi
    fi
  fi

  echo -e "${MAGENTA}${BOLD}Next Steps${NC}"
  print_rule
  echo -e "  ${YELLOW}1.${NC} ${WHITE}Reload your shell configuration if ~/.cargo/bin was newly added${NC}"
  echo -e "     ${DARK}exec \$SHELL${NC}"
  echo ""
  echo -e "  ${YELLOW}2.${NC} ${WHITE}Start Hyprland from your login manager${NC}"
  echo ""
  echo -e "  ${YELLOW}3.${NC} ${WHITE}Preview the SDDM theme if needed${NC}"
  echo -e "     ${DARK}sddm-greeter-qt6 --test-mode --theme /usr/share/sddm/themes/sddm-astronaut-theme/${NC}"
  echo ""
  echo -e "  ${YELLOW}4.${NC} ${WHITE}Check keybindings${NC}"
  echo -e "     ${DARK}Super + H${NC}"
  echo ""

  echo -e "${MAGENTA}${BOLD}Restore Backups${NC}"
  print_rule
  if [ -d "$BACKUP_DIR" ]; then
    echo -e "  ${WHITE}Backup location:${NC} ${CYAN}$BACKUP_DIR${NC}"
  else
    echo -e "  ${DARK}No backups created during this installation${NC}"
  fi
  echo ""

  echo -e "${MAGENTA}${BOLD}Uninstall Aurora${NC}"
  print_rule
  echo -e "  ${DARK}./install.sh --uninstall${NC}"
  echo ""
}

# Print usage
print_usage() {
  cat <<"EOF"
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
    if [ -n "${1:-}" ]; then
      print_error "Unknown option: ${1:-}"
      echo ""
      print_usage
      exit 1
    fi
    ;;
  esac

  prepare_install_log
  initialize_logging

  log_info "Aurora Installation Started"
  log_debug "Script location: $SCRIPT_DIR"
  log_debug "Interactive mode: $INTERACTIVE"
  log_debug "Dry-run mode: $DRY_RUN"

  clear_screen
  render_banner

  if [ "$DRY_RUN" = true ]; then
    echo -e "${YELLOW}${BOLD}[DRY RUN MODE]${NC} ${WHITE}No changes will be applied${NC}"
    echo ""
  fi

  # Run installation steps
  check_arch
  check_root
  check_home_disk_space
  detect_installation_type # Issue #5
  check_existing_install
  create_directories
  check_dependencies
  select_installation_mode # Issue #7 - Feature request
  validate_hyprland
  install_packages

  if [ "$DRY_RUN" = false ]; then
    install_sddm_theme
    build_rust_scripts
    install_waytrogen_aurora
    setup_lazyvim
    copy_dotfiles
    setup_shell_config
    verify_installation
  else
    next_step "Installing SDDM astronaut theme"
    print_warning "[DRY RUN] Would clone/configure the SDDM astronaut theme and install fonts"

    next_step "Building and installing Rust scripts"
    print_warning "[DRY RUN] Would build and install Rust scripts"

    next_step "Installing waytrogen-aurora"
    print_warning "[DRY RUN] Would clone/build/install waytrogen-aurora and compile schemas"

    next_step "Installing LazyVim starter"
    print_warning "[DRY RUN] Would backup Neovim files and install LazyVim starter"

    next_step "Installing configuration files"
    print_warning "[DRY RUN] Would copy configuration files"

    next_step "Setting up shell configuration"
    print_warning "[DRY RUN] Would update shell PATH"

    next_step "Verifying installation"
    print_warning "[DRY RUN] Would verify installed binaries and PATH"

    next_step "Switching sudo to sudo-rs"
    print_warning "[DRY RUN] Would switch /usr/bin/sudo to /usr/bin/sudo-rs"
  fi

  final_setup

  if [ "$DRY_RUN" = false ]; then
    switch_to_sudo_rs
  fi

  log_command "Aurora Installation Completed Successfully"
}

# Run main function
main "$@"
