pragma Singleton
import QtQuick
import "."

QtObject {
    readonly property string appName: "Aurora Bluetooth"
    readonly property string configVersion: "0.1"

    // Placement ---------------------------------------------------------------
    // Index into Quickshell.screens of the screen the popup lives on.
    readonly property int screenIndex: 0
    // Gap between the popup card and the screen edge.
    readonly property int popupMargin: 12
    // Extra gap from the top edge of the screen.
    readonly property int popupMarginTop: 64

    // Popup geometry ---------------------------------------------------------
    readonly property int popupWidth: 380
    readonly property int rowHeight: 52
    readonly property int maxListHeight: 364
    readonly property int emptyListHeight: 96

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

    // Shapes / text ----------------------------------------------------------
    readonly property int radius: 14
    readonly property int radiusSm: 9
    readonly property int textSize: 12
    readonly property int titleSize: 13
    readonly property int glyphSize: 13

    // Fonts ------------------------------------------------------------------
    readonly property string fontName: "JetBrainsMono NL Nerd Font"
    readonly property string glyphFont: "JetBrainsMono NL Nerd Font"

    // Glyphs (Nerd Font Font-Awesome 5 range) --------------------------------
    readonly property string gBluetooth: "\uf293"
    readonly property string gCheck: "\uf00c"
    readonly property string gRefresh: "\uf021"
    readonly property string gPower: "\uf011"
    readonly property string gTrash: "\uf1f8"
    readonly property string gTimes: "\uf00d"
    readonly property string gArrowLeft: "\uf060"
    readonly property string gSpinner: "\uf110"
    readonly property string gBookmark: "\uf02e"
}