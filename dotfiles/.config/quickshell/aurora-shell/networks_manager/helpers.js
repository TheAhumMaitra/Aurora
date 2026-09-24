.pragma library

function clamp(v, lo, hi) {
    return Math.max(lo, Math.min(hi, v));
}

function pct(strength) {
    if (typeof strength !== "number" || !isFinite(strength)) return 0;
    return Math.round(clamp(strength, 0, 1) * 100);
}

function litBars(strength, bars) {
    if (typeof strength !== "number" || !isFinite(strength)) return 0;
    return Math.round(clamp(strength, 0, 1) * bars);
}

function securityLabel(id) {
    switch (id) {
    case 0: return "WPA3 192-bit";
    case 1: return "WPA3";
    case 2: return "WPA2 Enterprise";
    case 3: return "WPA2";
    case 4: return "WPA Enterprise";
    case 5: return "WPA";
    case 6: return "WEP";
    case 7: return "WEP";
    case 8: return "LEAP";
    case 9: return "WPA2 Transitional";
    case 10: return "Open";
    default: return "Unknown";
    }
}

function isOpen(id) {
    return id === 10;
}

function isEnterprise(id) {
    return id === 2 || id === 4;
}

function needsPsk(id) {
    return !isOpen(id) && !isEnterprise(id);
}

function failMessage(reason) {
    switch (reason) {
    case 1: return "Missing password or credentials.";
    case 2: return "The Wi-Fi client was disconnected.";
    case 3: return "Could not join the network.";
    case 4: return "Authentication failed. Check your password.";
    case 5: return "The network was lost while connecting.";
    case 0:
    default: return "Something went wrong while connecting.";
    }
}

function connectivityLabel(conn) {
    switch (conn) {
    case 1: return "Offline";
    case 2: return "Portal";
    case 3: return "Limited";
    case 4: return "Online";
    case 0:
    default: return "Checking\u2026";
    }
}

function stateLabel(state) {
    switch (state) {
    case 1: return "Connecting\u2026";
    case 3: return "Disconnecting\u2026";
    case 2: return "Connected";
    case 0: return "Unknown";
    default: return "Disconnected";
    }
}

function includesIgnoreCase(haystack, needle) {
    if (typeof haystack !== "string") return false;
    if (needle === "") return true;
    try {
        return haystack.toLowerCase().indexOf(needle.toLowerCase()) !== -1;
    } catch (e) {
        return false;
    }
}