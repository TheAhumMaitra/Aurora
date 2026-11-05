.pragma library

function clamp(v, lo, hi) {
    return Math.max(lo, Math.min(hi, v));
}

function formatTime(sec) {
    if (typeof sec !== "number" || !isFinite(sec) || isNaN(sec) || sec < 0) {
        return "00:00";
    }
    var total = Math.floor(sec);
    var hours = Math.floor(total / 3600);
    var mins = Math.floor((total % 3600) / 60);
    var secs = total % 60;

    var sStr = secs < 10 ? "0" + secs : String(secs);
    if (hours > 0) {
        var mStr = mins < 10 ? "0" + mins : String(mins);
        return hours + ":" + mStr + ":" + sStr;
    }
    var mStr = mins < 10 ? "0" + mins : String(mins);
    return mStr + ":" + sStr;
}

function cleanArtUrl(url) {
    if (!url || typeof url !== "string") return "";
    var s = url.trim();
    if (s === "") return "";
    if (s.startsWith("/")) {
        return "file://" + s;
    }
    return s;
}

function playerGlyph(identity) {
    if (!identity) return "\uf001";
    var id = String(identity).toLowerCase();
    if (id.indexOf("spotify") !== -1) return "\uf1bc";
    if (id.indexOf("firefox") !== -1) return "\uf269";
    if (id.indexOf("chrome") !== -1 || id.indexOf("chromium") !== -1 || id.indexOf("brave") !== -1 || id.indexOf("zen") !== -1) return "\uf268";
    if (id.indexOf("vlc") !== -1 || id.indexOf("mpv") !== -1 || id.indexOf("celluloid") !== -1 || id.indexOf("video") !== -1) return "\uf008";
    return "\uf001";
}
