.pragma library

function clamp(v, lo, hi) {
    return Math.max(lo, Math.min(hi, v));
}

function pctText(value) {
    if (typeof value !== "number" || !isFinite(value)) return "0%";
    return Math.round(clamp(value, 0, 9.99) * 100) + "%";
}

function formatDuration(seconds) {
    if (!seconds || seconds <= 0) return "";
    var m = Math.floor(seconds / 60);
    var h = Math.floor(m / 60);
    m = m % 60;
    if (h > 0) return h + "h " + m + "m";
    return m + "m";
}
