#!/usr/bin/env bash
# Aurora Universal Installation Bootstrap (Arch Linux / Fedora Linux)
#
# The marker below is used by this script to recognise itself, so that it never
# picks itself up as a distribution installer when an Aurora checkout is reused.
# AURORA_UNIVERSAL_BOOTSTRAP

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

# Never ask for usernames, passwords, or passphrases interactively.
# Without this, a typo'd, private, or unreachable HTTPS URL makes git print
# "Username for 'https://github.com':", and an SSH remote on a fresh machine
# (no keys / no known_hosts entries) prompts for passphrases or host
# verification, which looks like Aurora needs git setup or a GitHub account.
# Fail fast with a readable error instead. These exports also propagate to the
# edition installer executed later, so its "pull latest changes" self-update
# cannot prompt either.
# Escape hatches for private edition repositories that truly need auth:
#   GIT_TERMINAL_PROMPT=1 ./install.sh ...   (re-enable HTTPS prompts)
#   GIT_SSH_COMMAND="ssh" ./install.sh ...    (re-enable SSH prompts)
export GIT_TERMINAL_PROMPT="${GIT_TERMINAL_PROMPT:-0}"
export GIT_SSH_COMMAND="${GIT_SSH_COMMAND:-ssh -o BatchMode=yes -o StrictHostKeyChecking=accept-new}"

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
SELF_PATH="$SCRIPT_DIR/$(basename "${BASH_SOURCE[0]}")"
BOOTSTRAP_LOG="${AURORA_BOOTSTRAP_LOG:-$HOME/.local/share/Aurora/bootstrap.log}"
LOG_LEVEL="${LOG_LEVEL:-INFO}"

# Edition repositories
ARCH_REPO="${AURORA_ARCH_REPO:-https://github.com/TheAhumMaitra/Aurora-Arch.git}"
FEDORA_REPO="${AURORA_FEDORA_REPO:-https://github.com/TheAhumMaitra/Aurora-Fedora.git}"
REPO_URL=""
REPO_BRANCH="${AURORA_BRANCH:-}"
EXPECTED_SCRIPT=""

# Runtime state
DISTRO_REQUEST="" # empty = auto-detect, otherwise arch | fedora
DISTRO_FAMILY=""
DISTRO_ID="unknown"
DISTRO_LIKE=""
DISTRO_NAME="unknown"
CLONE_DIR="${AURORA_DIR:-}"
INSTALLER_PATH=""
RUN_INSTALLER=true
FORCE_CLONE=false
ASSUME_YES=false
CURRENT_STEP=0
PASSTHROUGH_ARGS=()

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

  log_dir="$(dirname "$BOOTSTRAP_LOG")"
  mkdir -p "$log_dir"
  echo "[$timestamp] [$level] $message" >>"$BOOTSTRAP_LOG"

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

print_header() {
  print_spacer
  print_rule
  echo -e "${WHITE}${BOLD}  $1${NC}"
  echo -e "${DARK}  Aurora universal installer bootstrap${NC}"
  print_rule
}

print_success() { log_success "$1"; }
print_warning() { log_warn "$1"; }
print_error() { log_error "$1"; }

next_step() {
  ((++CURRENT_STEP))
  echo ""
  echo -e "${MAGENTA}${BOLD}◉ Step ${CURRENT_STEP}${NC} ${WHITE}${BOLD}$1${NC}"
  echo -e "${DIM}${CYAN}  Preparing this stage...${NC}"
  log_info "Step $CURRENT_STEP: $1"
}

clear_screen() {
  command -v clear &>/dev/null && clear || true
}

render_banner() {
  print_spacer
  echo -e "${MAGENTA}${BOLD}        ▄▄▄    █    ██   ██▀███   ▒█████   ██▀███   ▄▄▄       ${NC}"
  echo -e "${MAGENTA}${BOLD}      ▒████▄   ██  ▓██▒ ▓██ ▒ ██▒▒██▒  ██▒▓██ ▒ ██▒ ▒████▄     ${NC}"
  echo -e "${BLUE}${BOLD}        ▒██  ▀█▄  ▓██  ▒██ ░▓██ ░▄█ ▒▒██░  ██▒▓██ ░▄█  ▒██  ▀█▄   ${NC}"
  echo -e "${CYAN}${BOLD}        ░██▄▄▄▄██ ▓▓█  ░██ ░▒██▀▀█▄  ▒██   ██░▒██▀▀█▄  ░██▄▄▄▄██  ${NC}"
  echo -e "${GREEN}${BOLD}        ▓█   ▓██ ▒▒█████▓ ░██▓ ▒██▒░ ████▓▒░░██▓ ▒██▒ ▓█   ▓██▒ ${NC}"
  echo -e "${WHITE}${BOLD}  One installer for the Arch Linux and Fedora Linux editions${NC}"
  echo -e "${DARK}  Detects your distribution, fetches the right edition, then installs it.${NC}"
  print_rule
}

print_usage() {
  cat <<"EOF"
Aurora Universal Installation Bootstrap (Arch Linux / Fedora Linux)

Usage: ./install.sh [OPTIONS] [EDITION INSTALLER OPTIONS]

Options:
  --help                    Show this help message
  --distro <arch|fedora>    Force an edition instead of auto-detecting your distribution
  --dir <path>              Fetch the edition into <path> instead of ./Aurora
  --repo <url>              Fetch a custom edition repository
  --branch <name>           Fetch a specific branch of the edition repository
  --force                   Remove and re-clone an existing Aurora checkout
  --no-run                  Only fetch/prepare the edition, do not run its installer
  -y, --yes                 Assume "yes" for prompts (for example installing git)

Editions:
  Arch Linux    https://github.com/TheAhumMaitra/Aurora-Arch.git    -> install-arch.sh
  Fedora Linux  https://github.com/TheAhumMaitra/Aurora-Fedora.git  -> install-fedora.sh

Environment overrides:
  AURORA_ARCH_REPO, AURORA_FEDORA_REPO, AURORA_BRANCH, AURORA_DIR, AURORA_BOOTSTRAP_LOG

Git never prompts for credentials; unreachable repositories fail fast instead.
For private edition repositories, allow prompts with GIT_TERMINAL_PROMPT=1
(HTTPS) or GIT_SSH_COMMAND="ssh" (SSH).

Examples:
  ./install.sh                    # Detect the distribution, fetch the edition and install it
  ./install.sh --dry-run          # Same, but the edition installer only previews the changes
  ./install.sh --distro arch      # Force the Arch Linux edition
  ./install.sh --no-run           # Only fetch/prepare ./Aurora
  ./install.sh --uninstall        # Forwarded to the edition installer to remove Aurora

EOF
}

confirm() {
  local prompt="$1"
  local reply=""

  if [ "$ASSUME_YES" = true ]; then
    log_info "Auto-confirmed: $prompt"
    return 0
  fi

  if [ ! -t 0 ]; then
    log_warn "Not an interactive shell; skipping prompt: $prompt"
    return 1
  fi

  read -r -p "  ${WHITE}${prompt}${NC} ${DARK}[y/N]${NC} " reply || reply=""
  case "$reply" in
  [yY] | [yY][eE][sS]) return 0 ;;
  *) return 1 ;;
  esac
}

parse_args() {
  while [ $# -gt 0 ]; do
    case "$1" in
    --help | -h)
      print_usage
      exit 0
      ;;
    --distro)
      if [ $# -lt 2 ]; then
        print_error "--distro requires a value (arch or fedora)"
        exit 1
      fi
      case "$2" in
      arch | archlinux | fedora)
        if [ "$2" = "fedora" ]; then
          DISTRO_REQUEST="fedora"
        else
          DISTRO_REQUEST="arch"
        fi
        ;;
      *)
        print_error "Unknown distribution for --distro: $2 (expected arch or fedora)"
        exit 1
        ;;
      esac
      shift 2
      ;;
    --dir)
      if [ $# -lt 2 ]; then
        print_error "--dir requires a path"
        exit 1
      fi
      CLONE_DIR="$2"
      shift 2
      ;;
    --repo)
      if [ $# -lt 2 ]; then
        print_error "--repo requires a git URL"
        exit 1
      fi
      REPO_URL="$2"
      shift 2
      ;;
    --branch)
      if [ $# -lt 2 ]; then
        print_error "--branch requires a branch name"
        exit 1
      fi
      REPO_BRANCH="$2"
      shift 2
      ;;
    --force)
      FORCE_CLONE=true
      shift
      ;;
    --no-run)
      RUN_INSTALLER=false
      shift
      ;;
    --yes | -y)
      ASSUME_YES=true
      shift
      ;;
    --debug)
      LOG_LEVEL="DEBUG"
      # The edition installers understand --debug too, so keep forwarding it
      PASSTHROUGH_ARGS+=("$1")
      shift
      ;;
    *)
      # Unknown options belong to the edition installer and are forwarded as-is
      log_debug "Forwarding option to the edition installer: $1"
      PASSTHROUGH_ARGS+=("$1")
      shift
      ;;
    esac
  done

  if [ -z "$CLONE_DIR" ]; then
    # Default to the folder that already carries the dotfiles layout
    # (Aurora/dotfiles/.config), i.e. this checkout itself when its name is
    # "Aurora", or ./Aurora one level below otherwise. Never nest an extra
    # Aurora/Aurora level, which made the edition installer look for dotfiles
    # in the wrong place.
    if [ "$(basename "$SCRIPT_DIR")" = "Aurora" ]; then
      CLONE_DIR="$SCRIPT_DIR"
    else
      CLONE_DIR="$SCRIPT_DIR/Aurora"
    fi
  fi

  case "$CLONE_DIR" in
  /*) ;;
  *) CLONE_DIR="$PWD/$CLONE_DIR" ;;
  esac
}

in_word_list() {
  local needle="$1"
  shift
  local candidate

  for candidate in "$@"; do
    if [ "$needle" = "$candidate" ]; then
      return 0
    fi
  done

  return 1
}

detect_distro() {
  local family=""

  print_header "Detecting your Linux distribution"

  if [ -r /etc/os-release ]; then
    # shellcheck disable=SC1091
    . /etc/os-release
    DISTRO_ID="${ID:-unknown}"
    DISTRO_LIKE="${ID_LIKE:-}"
    DISTRO_NAME="${PRETTY_NAME:-${NAME:-$DISTRO_ID}}"
  else
    print_warning "/etc/os-release is unavailable; falling back to package manager detection"
    DISTRO_NAME="unknown"
  fi

  log_debug "os-release ID: $DISTRO_ID (ID_LIKE: ${DISTRO_LIKE:-none})"

  if [ -n "$DISTRO_REQUEST" ]; then
    family="$DISTRO_REQUEST"
    log_info "Distribution family forced to $family with --distro"
  else
    if in_word_list "$DISTRO_ID" arch archarm archlinux cachyos endeavouros manjaro garuda artix arcolinux archcraft; then
      family="arch"
    elif in_word_list "$DISTRO_ID" fedora nobara bazzite ultramarine; then
      family="fedora"
    else
      local like=""
      # shellcheck disable=SC2086
      for like in $DISTRO_LIKE; do
        case "$like" in
        arch)
          family="arch"
          break
          ;;
        fedora | rhel | centos)
          family="fedora"
          break
          ;;
        esac
      done
    fi

    if [ -z "$family" ]; then
      if command -v pacman &>/dev/null; then
        family="arch"
        log_debug "pacman is available; assuming the Arch family"
      elif command -v dnf &>/dev/null; then
        family="fedora"
        log_debug "dnf is available; assuming the Fedora family"
      fi
    fi
  fi

  if [ -z "$family" ]; then
    print_error "Unsupported distribution: $DISTRO_NAME"
    echo -e "  ${DARK}Aurora currently ships editions for Arch Linux and Fedora Linux only.${NC}"
    echo -e "  ${DARK}Rerun with --distro arch or --distro fedora to force an edition.${NC}"
    exit 1
  fi

  DISTRO_FAMILY="$family"

  if [ "$DISTRO_FAMILY" = "arch" ]; then
    EXPECTED_SCRIPT="install-arch.sh"
  else
    EXPECTED_SCRIPT="install-fedora.sh"
  fi

  if [ -z "$REPO_URL" ]; then
    if [ "$DISTRO_FAMILY" = "arch" ]; then
      REPO_URL="$ARCH_REPO"
    else
      REPO_URL="$FEDORA_REPO"
    fi
  fi

  print_success "Detected $DISTRO_NAME (family: $DISTRO_FAMILY)"
  log_info "Edition repository: $REPO_URL"
  log_info "Edition installer: $EXPECTED_SCRIPT"
  log_info "Aurora directory: $CLONE_DIR"
}

ensure_git() {
  next_step "Checking for git"

  if command -v git &>/dev/null; then
    print_success "git is available ($(git --version))"
    return 0
  fi

  print_warning "git is not installed and Aurora needs it to fetch its files"

  local -a install_cmd
  if [ "$DISTRO_FAMILY" = "arch" ]; then
    install_cmd=(sudo pacman -S --needed --noconfirm git)
  else
    install_cmd=(sudo dnf install -y git)
  fi

  if ! command -v sudo &>/dev/null; then
    print_error "sudo is not available; install git manually and rerun this script"
    exit 1
  fi

  if ! confirm "Install git now with: ${install_cmd[*]} ?"; then
    print_error "git is required to fetch Aurora"
    echo -e "  ${DARK}Install it with: ${install_cmd[*]}${NC}"
    exit 1
  fi

  log_info "Running: ${install_cmd[*]}"
  "${install_cmd[@]}"

  if ! command -v git &>/dev/null; then
    print_error "git installation failed"
    exit 1
  fi

  print_success "git installed successfully"
}

# Compare two git remote URLs for the same repository.
# Handles equivalent spellings of the same remote: scp-like SSH syntax
# (git@host:user/repo.git) vs ssh:// vs HTTPS, userinfo, letter case,
# trailing slashes, and a trailing ".git", so an existing checkout cloned
# over SSH still matches an HTTPS edition URL.
normalize_git_url() {
  local url="$1"
  local host=""
  local path=""

  # Trim surrounding whitespace.
  url="${url#"${url%%[![:space:]]*}"}"
  url="${url%"${url##*[![:space:]]}"}"

  if [[ "$url" == ssh://* ]]; then
    # ssh://[user@]host[:port]/path -> host/path (drop user, port).
    url="${url#ssh://}"
    url="${url#*@}"
    if [[ "$url" == *"/"* ]]; then
      host="${url%%/*}"
      path="${url#*/}"
      host="${host%%:*}"
    else
      host="${url%%:*}"
      path=""
    fi
    url="$host/$path"
  elif [[ "$url" == *"://"* ]]; then
    # scheme://[user@]host[:port]/path -> scheme://host/path
    local scheme rest
    scheme="${url%%://*}"
    rest="${url#*://}"
    if [[ "$rest" == *"@"* ]]; then
      rest="${rest#*@}"
    fi
    if [[ "$rest" == *"/"* ]]; then
      host="${rest%%/*}"
      path="${rest#*/}"
      host="${host%%:*}"
    else
      host="${rest%%:*}"
      path=""
    fi
    url="$scheme://$host/$path"
  elif [[ "$url" == *":"* ]]; then
    # scp-like [user@]host:path -> ssh-equivalent host/path.
    local remainder
    remainder="$url"
    if [[ "$remainder" == *"@"* ]]; then
      remainder="${remainder#*@}"
    fi
    host="${remainder%%:*}"
    path="${remainder#*:}"
    url="ssh://$host/$path"
  fi

  # Lowercase for comparison (host and GitHub owner/repo are case-insensitive).
  url="$(printf '%s' "$url" | tr '[:upper:]' '[:lower:]')"

  # Drop trailing slashes, one trailing ".git", and any slashes it reveals.
  while [[ "$url" == */ ]]; do
    url="${url%/}"
  done
  if [[ "$url" == *.git ]]; then
    url="${url%.git}"
  fi
  while [[ "$url" == */ ]]; do
    url="${url%/}"
  done

  # Treat scp-like/ssh:// SSH and HTTPS spellings of the same host/path
  # as equal (ssh://github.com/x vs https://github.com/x).
  url="${url#ssh://}"
  url="${url#https://}"
  url="${url#http://}"

  printf '%s' "$url"
}

git_urls_match() {
  [ "$(normalize_git_url "$1")" = "$(normalize_git_url "$2")" ]
}

# Explain why a repository could not be reached without ever prompting.
# Takes the captured git output as $1.
report_repo_unreachable() {
  local detail="${1:-}"

  print_error "Could not reach the Aurora $DISTRO_FAMILY edition at $REPO_URL"
  if [ -n "$detail" ]; then
    echo -e "  ${DARK}$(printf '%s' "$detail" | head -n 1)${NC}"
  fi
  case "$detail" in
  *"could not read Username"* | *"Authentication failed"* | *"Permission denied"* | *"publickey"* | *"repository not found"* | *"not found"*)
    echo -e "  ${DARK}The repository may be private, renamed, or the URL mistyped.${NC}"
    echo -e "  ${DARK}Git runs non-interactively, so it cannot ask for a username.${NC}"
    echo -e "  ${DARK}If this repository truly needs credentials, allow prompts with:${NC}"
    echo -e "  ${DARK}  GIT_TERMINAL_PROMPT=1 ./install.sh ...   (HTTPS)${NC}"
    echo -e "  ${DARK}  GIT_SSH_COMMAND=\"ssh\" ./install.sh ...    (SSH remotes)${NC}"
    ;;
  *"Could not resolve host"* | *"unable to connect"* | *"Connection refused"* | *"Network is unreachable"* | *"Temporary failure in name resolution"* | *"Could not resolve hostname"*)
    echo -e "  ${DARK}This looks like a network problem; check your connection and try again.${NC}"
    ;;
  *)
    echo -e "  ${DARK}Check that the URL is correct and reachable, then try again.${NC}"
    ;;
  esac
  exit 1
}

# Pre-flight check so a bad URL fails here with a clear message instead of
# stalling at a credential prompt (or a cryptic error) halfway through cloning.
verify_repo_reachable() {
  local output=""
  local output_tmp=""

  log_info "Checking that $REPO_URL is reachable"
  if [ -n "$REPO_BRANCH" ]; then
    output_tmp="$(mktemp)"
    if ! git ls-remote --heads "$REPO_URL" "$REPO_BRANCH" >"$output_tmp" 2>&1; then
      cat "$output_tmp" 2>/dev/null || true
      output="$(cat "$output_tmp" 2>/dev/null || true)"
      rm -f "$output_tmp"
      report_repo_unreachable "$output"
    fi
    output="$(cat "$output_tmp" 2>/dev/null || true)"
    rm -f "$output_tmp"
    if [ -z "$output" ]; then
      print_error "Branch '$REPO_BRANCH' was not found in $REPO_URL"
      echo -e "  ${DARK}Check --branch and that the repository URL is correct.${NC}"
      exit 1
    fi
  else
    output_tmp="$(mktemp)"
    if ! git ls-remote "$REPO_URL" HEAD >"$output_tmp" 2>&1; then
      cat "$output_tmp" 2>/dev/null || true
      output="$(cat "$output_tmp" 2>/dev/null || true)"
      rm -f "$output_tmp"
      report_repo_unreachable "$output"
    fi
    rm -f "$output_tmp"
  fi

  log_debug "Repository is reachable: $REPO_URL"
}

clone_or_update_repository() {
  next_step "Fetching the Aurora $DISTRO_FAMILY edition"

  mkdir -p "$(dirname "$CLONE_DIR")"

  if [ -d "$CLONE_DIR/.git" ]; then
    if [ "$FORCE_CLONE" = true ]; then
      log_warn "--force given; removing the existing checkout at $CLONE_DIR"
      rm -rf "$CLONE_DIR"
    else
      local existing_origin
      existing_origin="$(git -C "$CLONE_DIR" remote get-url origin 2>/dev/null || true)"
      if [ -n "$existing_origin" ] && ! git_urls_match "$existing_origin" "$REPO_URL"; then
        print_error "The checkout at $CLONE_DIR belongs to a different repository:"
        echo -e "  ${DARK}existing origin: $existing_origin${NC}"
        echo -e "  ${DARK}expected ($DISTRO_FAMILY edition): $REPO_URL${NC}"
        echo -e "  ${DARK}Pulling it would fetch the wrong project and may ask for credentials.${NC}"
        echo -e "  ${DARK}Rerun with --force to re-clone, --dir <path> for another location,${NC}"
        echo -e "  ${DARK}or --repo <url> matching the existing checkout.${NC}"
        exit 1
      fi
      if [ -n "$existing_origin" ]; then
        log_debug "Existing origin matches the $DISTRO_FAMILY edition"
      else
        log_warn "Existing checkout has no 'origin' remote; pulling its current upstream"
      fi
      log_info "Existing Aurora checkout found at $CLONE_DIR; updating it"
      if ! git -C "$CLONE_DIR" pull --ff-only; then
        print_error "Failed to update the Aurora checkout at $CLONE_DIR"
        echo -e "  ${DARK}Git runs non-interactively, so no username was requested.${NC}"
        echo -e "  ${DARK}This usually means the repository is unreachable, private, or SSH has no access.${NC}"
        echo -e "  ${DARK}Resolve the git error above, allow prompts with GIT_TERMINAL_PROMPT=1${NC}"
        echo -e "  ${DARK}(and GIT_SSH_COMMAND=\"ssh\" for SSH remotes), or rerun with --force to re-clone.${NC}"
        exit 1
      fi
      print_success "Aurora checkout updated"
      return 0
    fi
  elif [ -d "$CLONE_DIR" ] && [ -n "$(ls -A "$CLONE_DIR" 2>/dev/null || true)" ]; then
    if [ "$FORCE_CLONE" = true ]; then
      log_warn "--force given; removing the existing directory $CLONE_DIR"
      rm -rf "$CLONE_DIR"
    else
      print_error "$CLONE_DIR already exists and is not an Aurora git checkout"
      echo -e "  ${DARK}Move it away, choose another location with --dir, or rerun with --force.${NC}"
      exit 1
    fi
  fi

  verify_repo_reachable

  local -a clone_cmd=(git clone --depth 1)
  if [ -n "$REPO_BRANCH" ]; then
    clone_cmd+=(--branch "$REPO_BRANCH")
  fi
  clone_cmd+=("$REPO_URL" "$CLONE_DIR")

  log_info "Cloning $REPO_URL into $CLONE_DIR"
  log_debug "Running: ${clone_cmd[*]}"

  if ! "${clone_cmd[@]}"; then
    print_warning "Shallow clone failed; retrying with a full clone"
    rm -rf "$CLONE_DIR"

    local -a full_clone_cmd=(git clone)
    if [ -n "$REPO_BRANCH" ]; then
      full_clone_cmd+=(--branch "$REPO_BRANCH")
    fi
    full_clone_cmd+=("$REPO_URL" "$CLONE_DIR")

    log_debug "Running: ${full_clone_cmd[*]}"
    local full_clone_output=""
    local full_clone_tmp=""
    full_clone_tmp="$(mktemp)"
    if ! "${full_clone_cmd[@]}" >"$full_clone_tmp" 2>&1; then
      # Show the real git output, then explain it: with prompts disabled the
      # first failure mode is a terse auth error instead of a username prompt.
      cat "$full_clone_tmp" 2>/dev/null || true
      full_clone_output="$(cat "$full_clone_tmp" 2>/dev/null || true)"
      rm -f "$full_clone_tmp"
      report_repo_unreachable "$full_clone_output"
    fi
    rm -f "$full_clone_tmp"
  fi

  print_success "Aurora $DISTRO_FAMILY edition fetched into $CLONE_DIR"
}

locate_installer() {
  local dir="$1"
  local candidate
  local -a candidates

  if [ "$DISTRO_FAMILY" = "arch" ]; then
    candidates=("$EXPECTED_SCRIPT" "install.sh" "arch-install.sh")
  else
    candidates=("$EXPECTED_SCRIPT" "install.sh" "fedora-install.sh")
  fi

  for candidate in "${candidates[@]}"; do
    if [ ! -f "$dir/$candidate" ]; then
      continue
    fi
    if [ "$dir/$candidate" = "$SELF_PATH" ]; then
      continue
    fi
    if grep -q "AURORA_UNIVERSAL_BOOTSTRAP" "$dir/$candidate" 2>/dev/null; then
      log_debug "Skipping $candidate: it is the universal bootstrap, not an edition installer"
      continue
    fi
    echo "$dir/$candidate"
    return 0
  done

  # Last resort: any installer script in the checkout, never this bootstrap itself
  while IFS= read -r candidate; do
    if [ "$candidate" = "$SELF_PATH" ]; then
      continue
    fi
    if grep -q "AURORA_UNIVERSAL_BOOTSTRAP" "$candidate" 2>/dev/null; then
      continue
    fi
    echo "$candidate"
    return 0
  done < <(find "$dir" -maxdepth 2 -type f -name 'install*.sh' 2>/dev/null | LC_ALL=C sort)

  return 1
}

patch_self_update_reference() {
  local script_path="$1"
  local original_name="$2"

  if [ "$original_name" = "$EXPECTED_SCRIPT" ]; then
    return 0
  fi

  if grep -q -F "exec \"\$SCRIPT_DIR/$original_name\"" "$script_path" 2>/dev/null; then
    sed -i "s|exec \"\$SCRIPT_DIR/$original_name\"|exec \"\$SCRIPT_DIR/$EXPECTED_SCRIPT\"|g" "$script_path"
    log_info "Pointed the self-update of $EXPECTED_SCRIPT to the renamed script"
  fi
}

move_installer_into_aurora_dir() {
  next_step "Preparing the installer inside $CLONE_DIR"

  local source_path=""
  if ! source_path="$(locate_installer "$CLONE_DIR")"; then
    print_error "Could not find $EXPECTED_SCRIPT in $CLONE_DIR"
    echo -e "  ${DARK}$REPO_URL does not look like a valid Aurora edition.${NC}"
    exit 1
  fi

  local target_path="$CLONE_DIR/$EXPECTED_SCRIPT"

  if [ "$source_path" != "$target_path" ]; then
    log_info "Moving $(basename "$source_path") to $target_path"
    mv -f "$source_path" "$target_path"
    patch_self_update_reference "$target_path" "$(basename "$source_path")"
  else
    log_debug "$EXPECTED_SCRIPT is already in place"
  fi

  chmod +x "$target_path"
  INSTALLER_PATH="$target_path"

  print_success "Installer ready: $INSTALLER_PATH"
}

run_installer() {
  if [ "$RUN_INSTALLER" = false ]; then
    print_header "Aurora is ready"
    echo -e "  ${WHITE}Edition folder:${NC} ${CYAN}$CLONE_DIR${NC}"
    echo -e "  ${WHITE}Installer:${NC} ${CYAN}$INSTALLER_PATH${NC}"
    echo -e "  ${DARK}--no-run was given, so the installer was not executed.${NC}"
    echo ""
    echo -e "  ${DARK}Run it later with: $INSTALLER_PATH${NC}"
    echo ""
    return 0
  fi

  print_header "Launching the Aurora $DISTRO_FAMILY installer"

  cd "$CLONE_DIR" || exit 1

  log_info "Running: $INSTALLER_PATH"

  if [ "${#PASSTHROUGH_ARGS[@]}" -gt 0 ]; then
    log_info "Forwarding options: ${PASSTHROUGH_ARGS[*]}"
    exec "$INSTALLER_PATH" "${PASSTHROUGH_ARGS[@]}"
  fi

  exec "$INSTALLER_PATH"
}

# Main bootstrap flow
main() {
  parse_args "$@"

  clear_screen
  render_banner

  if [ "$EUID" -eq 0 ]; then
    print_warning "Running as root; the Aurora installers expect to run as a regular user"
  fi

  detect_distro
  ensure_git
  clone_or_update_repository
  move_installer_into_aurora_dir
  run_installer
}

# Run main function
main "$@"