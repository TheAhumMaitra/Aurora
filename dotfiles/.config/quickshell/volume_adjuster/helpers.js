.pragma library

// PwNodeType::Flag values (quickshell pipewire service). Enum singletons are
// not accessible from .pragma library files, so keep the numeric constants.
const NT_AUDIO = 1;   // 0b1
const NT_STREAM = 4;  // 0b100
const NT_SOURCE = 8;  // 0b1000
const NT_SINK = 16;   // 0b10000

function clamp(v, lo, hi) {
    return Math.max(lo, Math.min(hi, v));
}

// Normalize a pipewire "visual" volume (1.0 == 100%) to a percentage string.
function pctText(volume) {
    if (typeof volume !== "number" || !isFinite(volume)) return "0%";
    return Math.round(clamp(volume, 0, 9.99) * 100) + "%";
}

// Classify a pipewire node as a sink, a source or null. Streams (per-app
// playback/capture) and monitor sources are treated as non-devices.
function classify(node) {
    if (!node || !node.type) return null;
    if (node.type & NT_STREAM) return null;
    if (!(node.type & NT_AUDIO) || !node.audio) return null;

    if (node.properties && node.properties["media.category"] === "Monitor") return null;

    var isSink = (node.type & NT_SINK) !== 0;
    var isSource = (node.type & NT_SOURCE) !== 0;
    if (isSink && isSource) return "both";
    if (isSink) return "sink";
    if (isSource) return "source";
    return null;
}

// Friendly device label: description > nickname > node name.
function deviceLabel(node) {
    if (!node) return "";
    return node.description || node.nickname || node.name || "";
}