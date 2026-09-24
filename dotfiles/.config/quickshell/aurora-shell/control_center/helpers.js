.pragma library

function clamp(v, lo, hi) {
    return Math.max(lo, Math.min(hi, v));
}

function pctText(value) {
    if (typeof value !== "number" || !isFinite(value)) return "0%";
    return Math.round(clamp(value, 0, 9.99) * 100) + "%";
}