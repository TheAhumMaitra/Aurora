// Config.qml

pragma Singleton
import QtQuick
import "."

QtObject {
    readonly property string appName: "Aurora Display Manager"
    readonly property string configVersion: "0.1"

    // Placement ---------------------------------------------------------------
    readonly property int screenIndex: 0
    readonly property int popupMargin: 12
    readonly property int popupMarginTop: 54

    // Popup geometry ---------------------------------------------------------
    readonly property int popupWidth: 400
    readonly property int sliderHeight: 42
    readonly property int tileRadius: 10
    readonly property int controlRadius: 8
    readonly property int radius: 14
    readonly property int radiusSm: 9

    // Palette (colors.qml) ---------------------------------------------------
    readonly property color bg: Colors.background
    readonly property color bgElevated: Colors.surface
    readonly property color bgHover: Colors.activeBackground
    readonly property color fg: Colors.foreground
    readonly property color fgSub: Qt.alpha(Colors.foreground, 0.65)
    readonly property color fgDim: Colors.muted
    readonly property color border: Colors.border
    readonly property color accent: Colors.accent
    readonly property color accentActive: Colors.activeAccent
    readonly property color accentSoft: Qt.alpha(Colors.accent, 0.16)
    readonly property color ok: Colors.success
    readonly property color warn: Colors.warning
    readonly property color danger: Colors.urgentBackground
    readonly property color warmAmber: "#f5a97f"
    readonly property color coolBlue: "#8bd5ca"

    // Typography -------------------------------------------------------------
    readonly property int textSize: 11
    readonly property int titleSize: 13
    readonly property int subSize: 9
    readonly property int glyphSize: 13

    // Fonts ------------------------------------------------------------------
    readonly property string fontName: "JetBrainsMono NL Nerd Font"
    readonly property string glyphFont: "JetBrainsMono NL Nerd Font"

    // Glyphs (Nerd Font Font-Awesome range) -----------------------------------
    readonly property string gMonitor: "\uf108"
    readonly property string gLaptop: "\uf109"
    readonly property string gBrightness: "\uf185"
    readonly property string gMoon: "\uf186"
    readonly property string gThermometer: "\uf2c9"
    readonly property string gSync: "\uf021"
    readonly property string gClock: "\uf017"
    readonly property string gPower: "\uf011"
    readonly property string gTimes: "\uf00d"
    readonly property string gCheck: "\uf00c"
    readonly property string gRefresh: "\uf021"
    readonly property string gSliders: "\uf1de"
    readonly property string gEye: "\uf06e"
    readonly property string gChevronRight: "\uf054"
}
