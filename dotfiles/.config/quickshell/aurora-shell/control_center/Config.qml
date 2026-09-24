// Config.qml

pragma Singleton
import QtQuick
import "."

QtObject {
    readonly property string appName: "Aurora Control Center"
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
    readonly property int tileHeight: 66
    readonly property int tileRadius: 10
    readonly property int sliderHeight: 44
    readonly property int controlRadius: 9

    // Volume -----------------------------------------------------------------
    // 1.0 == 100 %. Allow amplification past 100 % (wireplumber goes to ~150 %).
    readonly property real maxVolume: 1.5

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

    // Theme grid --------------------------------------------------------------
    readonly property int themeCellWidth: 174
    readonly property int themeCellHeight: 96
    readonly property int themeImgHeight: 56
    readonly property int themeHeaderHeight: 52
    readonly property int themeLabelSize: 9
    readonly property int maxThemeGridHeight: 340

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
    readonly property string gWifi: "\uf1eb"
    readonly property string gBluetooth: "\uf293"
    readonly property string gPlane: "\uf072"
    readonly property string gMic: "\uf130"
    readonly property string gMicMute: "\uf131"
    readonly property string gVolumeHigh: "\uf028"
    readonly property string gVolumeLow: "\uf027"
    readonly property string gVolumeOff: "\uf026"
    readonly property string gBrightness: "\uf185"
    readonly property string gTimes: "\uf00d"
    readonly property string gChevronRight: "\uf054"
    readonly property string gChevronDown: "\uf078"
    readonly property string gCheck: "\uf00c"
    readonly property string gSettings: "\uf013"
    readonly property string gPower: "\uf011"
    readonly property string gRefresh: "\uf021"
    readonly property string gPalette: "\uf53f"
    readonly property string gTerminal: "\uf120"
    readonly property string gVideo: "\uf03d"
}