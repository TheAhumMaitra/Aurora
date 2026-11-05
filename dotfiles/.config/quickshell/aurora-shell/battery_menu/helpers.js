.pragma library

function clamp(v, lo, hi) {
    return Math.max(lo, Math.min(hi, v));
}

function batteryGlyph(pct, charging) {
    if (charging) return "\uf0e7";
    if (pct >= 85) return "\uf240";
    if (pct >= 60) return "\uf241";
    if (pct >= 35) return "\uf242";
    if (pct >= 15) return "\uf243";
    return "\uf244";
}

function levelColor(pct, charging, acOnline) {
    if (charging) return "#a6e3a1";
    if (acOnline && pct >= 95) return "#89dceb";
    if (pct >= 75) return "#a6e3a1";
    if (pct >= 45) return "#89b4fa";
    if (pct >= 25) return "#f9e2af";
    if (pct >= 12) return "#fab387";
    return "#f38ba8";
}

function healthColor(pct) {
    if (pct >= 80) return "#a6e3a1";
    if (pct >= 65) return "#f9e2af";
    if (pct >= 50) return "#fab387";
    return "#f38ba8";
}

function healthRating(pct) {
    if (pct >= 85) return "Excellent";
    if (pct >= 75) return "Good";
    if (pct >= 60) return "Fair";
    return "Degraded";
}

function peripheralGlyph(type) {
    switch (type) {
    case "mouse": return "\uf245";
    case "keyboard": return "\uf11c";
    case "audio": return "\uf025";
    case "gamepad": return "\uf11b";
    default: return "\uf108";
    }
}
