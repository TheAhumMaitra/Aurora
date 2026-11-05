// Config.qml

pragma Singleton
import QtQuick
import "."

QtObject {
    readonly property string appName: "Aurora Battery"
    readonly property string configVersion: "0.1"

    // Placement ---------------------------------------------------------------
    readonly property int screenIndex: 0
    readonly property int popupMargin: 12
    readonly property int popupMarginTop: 64

    // Popup geometry ---------------------------------------------------------
    readonly property int popupWidth: 380
    readonly property int radius: 14
    readonly property int radiusSm: 9
    readonly property int textSize: 12
    readonly property int titleSize: 13
    readonly property int glyphSize: 13

    // Palette (colors.qml) ---------------------------------------------------
    readonly property color bg: Colors.background
    readonly property color bgElevated: Colors.surface
    readonly property color bgHover: Colors.activeBackground
    readonly property color fg: Colors.foreground
    readonly property color fgSub: Qt.alpha(Colors.foreground, 0.6)
    readonly property color fgDim: Colors.muted
    readonly property color border: Colors.border
    readonly property color accent: Colors.accent
    readonly property color accentActive: Colors.activeAccent
    readonly property color accentSoft: Qt.alpha(Colors.accent, 0.16)
    readonly property color ok: Colors.success
    readonly property color warn: Colors.warning
    readonly property color danger: Colors.urgentBackground
    readonly property color scrim: Qt.alpha(Colors.background, 0.22)

    // Dynamic Battery Colors
    readonly property color chargeColor: "#a6e3a1"      // Green
    readonly property color highColor: "#94e2d5"        // Teal
    readonly property color normalColor: "#89b4fa"      // Blue
    readonly property color mediumColor: "#f9e2af"      // Yellow
    readonly property color lowColor: "#fab387"         // Orange / Peach
    readonly property color criticalColor: "#f38ba8"    // Red

    // Fonts ------------------------------------------------------------------
    readonly property string fontName: "JetBrainsMono NL Nerd Font"
    readonly property string glyphFont: "JetBrainsMono NL Nerd Font"

    // Glyphs (Nerd Font Font-Awesome 5 range) --------------------------------
    readonly property string gBatteryFull: "\uf240"
    readonly property string gBatteryThreeQuarters: "\uf241"
    readonly property string gBatteryHalf: "\uf242"
    readonly property string gBatteryQuarter: "\uf243"
    readonly property string gBatteryEmpty: "\uf244"
    readonly property string gBolt: "\uf0e7"
    readonly property string gPlug: "\uf1e6"
    readonly property string gLeaf: "\uf06c"
    readonly property string gSun: "\uf185"
    readonly property string gGauge: "\uf0e4"
    readonly property string gHeart: "\uf004"
    readonly property string gCycle: "\uf1da"
    readonly property string gChip: "\uf2db"
    readonly property string gTimes: "\uf00d"
    readonly property string gRefresh: "\uf021"
    readonly property string gInfo: "\uf05a"
    readonly property string gCheck: "\uf00c"
    readonly property string gActivity: "\uf201"
    readonly property string gShield: "\uf132"
    readonly property string gBuilding: "\uf1ad"
    readonly property string gMouse: "\uf245"
    readonly property string gKeyboard: "\uf11c"
    readonly property string gHeadphones: "\uf025"
    readonly property string gDevice: "\uf108"
}
