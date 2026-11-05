import QtQuick
import QtQuick.Controls
import QtQuick.Effects
import QtQuick.Layouts
import Quickshell
import Quickshell.Io
import "."

Scope {
    id: root

    // Popup state. The window is only mapped while the popup is open: a mapped
    // full screen layer surface keeps capturing pointer input.
    property bool popupOpen: false

    // ------------------------------------------------------- live color reload
    FileView {
        id: colorFile
        path: Quickshell.shellPath("colors.qml")
        preload: true
        watchChanges: true

        onFileChanged: reload()
        onLoaded: root.applyColors()
    }

    function applyColors() {
        var text = colorFile.text()
        if (!text) return
        var known = ["accent", "activeBackground", "activeAccent",
                     "urgentBackground", "border", "surface", "surfaceAlt",
                     "muted", "background", "foreground", "success", "warning"]
        var re = /(?:readonly\s+)?property\s+color\s+(\w+)\s*:\s*(['"])([\s\S]*?)\2/g
        var m
        while ((m = re.exec(text)) !== null) {
            if (known.indexOf(m[1]) !== -1) {
                Colors[m[1]] = m[3]
            }
        }
    }

    // -------------------------------------------------------------- popup window
    PanelWindow {
        id: menuWindow
        visible: root.popupOpen
        screen: Quickshell.screens[Config.screenIndex] || Quickshell.screens[0]

        anchors {
            top: true
            bottom: true
            left: true
            right: true
        }

        aboveWindows: true
        focusable: true
        exclusionMode: ExclusionMode.Ignore
        exclusiveZone: 0
        color: "transparent"

        onVisibleChanged: {
            if (visible) {
                popupContent.forceActiveFocus()
            }
        }

        Item {
            id: popupContent
            anchors.fill: parent

            focus: true
            Keys.onEscapePressed: root.closePopup()

            MouseArea {
                anchors.fill: parent
                z: 1
                onClicked: root.closePopup()
            }

            MultiEffect {
                id: cardFx
                z: 2
                x: popupContent.width - Config.popupWidth - Config.popupMargin
                y: Config.popupMarginTop
                width: Config.popupWidth
                height: card.implicitHeight
                transformOrigin: Item.TopRight

                source: card

                shadowEnabled: true
                shadowBlur: 0.85
                shadowVerticalOffset: 2
                shadowColor: "#cc000000"

                opacity: root.popupOpen ? 1 : 0
                scale: root.popupOpen ? 1 : 0.97

                Behavior on opacity { NumberAnimation { duration: 160; easing.type: Easing.OutCubic } }
                Behavior on scale { NumberAnimation { duration: 160; easing.type: Easing.OutCubic } }
                Behavior on height { NumberAnimation { duration: 180; easing.type: Easing.OutCubic } }

                PopupCard {
                    id: card
                    width: parent.width
                    height: card.implicitHeight
                    active: root.popupOpen

                    onRequestClose: root.closePopup()
                }
            }
        }
    }

    // ------------------------------------------------------------------------- ipc
    function openPopup() {
        root.popupOpen = true
    }

    function closePopup() {
        root.popupOpen = false
    }

    function togglePopup() {
        root.popupOpen = !root.popupOpen
    }

    IpcHandler {
        target: "display"

        property bool popupOpen: root.popupOpen
        property int monitorsCount: card.monitorsCount
        property int brightness: card.brightnessPct
        property int warmth: card.warmthPct
        property bool nightLightOn: card.nightLightOn
        property bool vrrOn: card.vrrOn
        property string activePreset: card.activePreset
        property int cardWidth: card.implicitWidth
        property int cardHeight: card.implicitHeight

        function toggle(): void {
            root.togglePopup()
        }

        function open(): void {
            root.openPopup()
        }

        function close(): void {
            root.closePopup()
        }

        function rescan(): void {
            card.rescanMonitors()
        }

        function setBrightness(percent: real): void {
            card.setBrightness(percent)
        }

        function setWarmth(percent: real): void {
            card.applyWarmth(percent)
        }

        function toggleNightLight(): void {
            card.toggleNightLight()
        }

        function toggleVrr(): void {
            card.toggleVrr()
        }

        function sleepDisplays(): void {
            card.sleepDisplaysNow()
        }

        function setSleepPreset(preset: string): void {
            card.setSleepPreset(preset)
        }
    }
}
