.pragma library

function clamp(v, lo, hi) {
    return Math.max(lo, Math.min(hi, v));
}

function pctText(value) {
    if (typeof value !== "number" || !isFinite(value)) return "0%";
    return Math.round(clamp(value, 0, 1) * 100) + "%";
}

function formatRate(rate) {
    if (typeof rate !== "number" || !isFinite(rate)) return "60Hz";
    return Math.round(rate) + "Hz";
}

function warmthToKelvin(warmthPct) {
    var p = clamp(warmthPct, 0, 100);
    return Math.round(6500 - (p / 100) * (6500 - 2500));
}

function kelvinToWarmth(kelvin) {
    var k = clamp(kelvin, 2500, 6500);
    return Math.round(((6500 - k) / (6500 - 2500)) * 100);
}

function warmthLabel(warmthPct) {
    if (warmthPct <= 0) return "6500K (Off)";
    var k = warmthToKelvin(warmthPct);
    var desc = "Warm";
    if (warmthPct <= 25) desc = "Mild";
    else if (warmthPct <= 50) desc = "Cozy";
    else if (warmthPct <= 75) desc = "Warm";
    else desc = "Amber";
    return k + "K \u00b7 " + desc;
}
