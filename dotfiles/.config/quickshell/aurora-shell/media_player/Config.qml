pragma Singleton
import QtQuick
import "."

QtObject {
    readonly property string appName: "Aurora Media Hub"
    readonly property string configVersion: "0.1"

    // Placement ---------------------------------------------------------------
    readonly property int screenIndex: 0
    readonly property int popupMargin: 12
    readonly property int popupMarginTop: 64

    // Popup geometry ---------------------------------------------------------
    readonly property int popupWidth: 410
    readonly property int artSize: 108

    // Palette (colors.qml) ---------------------------------------------------
    readonly property color bg: Colors.background
    readonly property color bgElevated: Colors.surface
    readonly property color bgHover: Colors.activeBackground
    readonly property color fg: Colors.foreground
    readonly property color fgSub: Qt.alpha(Colors.foreground, 0.72)
    readonly property color fgDim: Colors.muted
    readonly property color border: Colors.border
    readonly property color accent: Colors.accent
    readonly property color accentActive: Colors.activeAccent
    readonly property color accentSoft: Qt.alpha(Colors.accent, 0.16)
    readonly property color ok: Colors.success
    readonly property color warn: Colors.warning
    readonly property color danger: Colors.urgentBackground

    // Shapes / text ----------------------------------------------------------
    readonly property int radius: 18
    readonly property int radiusSm: 10
    readonly property int textSize: 12
    readonly property int titleSize: 14
    readonly property int glyphSize: 13

    // Fonts ------------------------------------------------------------------
    readonly property string fontName: "JetBrainsMono NL Nerd Font"
    readonly property string glyphFont: "JetBrainsMono NL Nerd Font"

    // Glyphs (Nerd Font Font-Awesome 5 range) --------------------------------
    readonly property string gMusic: "\uf001"
    readonly property string gPlay: "\uf04b"
    readonly property string gPause: "\uf04c"
    readonly property string gPrev: "\uf048"
    readonly property string gNext: "\uf051"
    readonly property string gStop: "\uf04d"
    readonly property string gShuffle: "\uf074"
    readonly property string gRepeat: "\uf01e"
    readonly property string gVolumeOff: "\uf026"
    readonly property string gVolumeLow: "\uf027"
    readonly property string gVolumeHigh: "\uf028"
    readonly property string gTimes: "\uf00d"
    readonly property string gExternalLink: "\uf08e"
    readonly property string gDisc: "\uf10c"
    readonly property string gSpotify: "\uf1bc"
    readonly property string gBrowser: "\uf269"
    readonly property string gVideo: "\uf008"
    readonly property string gCheck: "\uf00c"
    readonly property string gArrowLeft: "\uf060"
}
