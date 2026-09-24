.pragma library

function clamp(v, lo, hi) {
    return Math.max(lo, Math.min(hi, v));
}

function pct(battery) {
    if (typeof battery !== "number" || !isFinite(battery)) return 0;
    return Math.round(clamp(battery, 0, 100));
}

// BluetoothDevice.icon -> Font-Awesome 5 glyph codepoint.
function deviceGlyph(icon) {
    if (!icon) return "\uf293";
    icon = String(icon).toLowerCase();
    if (icon.indexOf("audio") !== -1) return "\uf025";
    if (icon.indexOf("keyboard") !== -1) return "\uf11c";
    if (icon.indexOf("mouse") !== -1) return "\uf245";
    if (icon.indexOf("phone") !== -1) return "\uf10b";
    if (icon.indexOf("tablet") !== -1) return "\uf3fa";
    if (icon.indexOf("computer") !== -1 || icon.indexOf("laptop") !== -1) return "\uf109";
    if (icon.indexOf("gamepad") !== -1) return "\uf11b";
    return "\uf293";
}

// BluetoothDeviceState: Disconnected=0, Connected=1, Disconnecting=2, Connecting=3
function deviceSubLabel(device) {
    if (!device) return "";
    if (device.connected) return "Connected";
    if (device.pairing) return "Pairing\u2026";
    switch (device.state) {
    case 3: return "Connecting\u2026";
    case 2: return "Disconnecting\u2026";
    case 1: return "Connected";
    default: break;
    }
    if (device.paired) return "Paired";
    return "Discovered";
}

// BluetoothAdapterState: Disabled=0, Enabled=1, Enabling=2, Disabling=3, Blocked=4
function adapterSuffix(state) {
    switch (state) {
    case 1: return "On";
    case 2: return "Enabling\u2026";
    case 3: return "Disabling\u2026";
    case 4: return "Blocked";
    case 0:
    default: return "Off";
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