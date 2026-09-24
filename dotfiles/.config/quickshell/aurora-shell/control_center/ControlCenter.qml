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

    // The other Aurora managers, injected by the shell root. Used to open the
    // detailed popups and to keep at most one popup mapped at a time.
    property var bluetoothManager: null
    property var networksManager: null
    property var volumeManager: null

    // ------------------------------------------------------- live color reload
    // Watch colors.qml and push changes into the Colors singleton so every
    // Config binding re-evaluates without reloading the whole shell.
    FileView {
        id: colorFile
        path: Quickshell.configPath("colors.qml")
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

                source: card

                shadowEnabled: true
                shadowBlur: 0.85
                shadowVerticalOffset: 2
                shadowColor: "#cc000000"

                opacity: root.popupOpen ? 1 : 0
                scale: root.popupOpen ? 1 : 0.97

                Behavior on opacity { NumberAnimation { duration: 160; easing.type: Easing.OutCubic } }
                Behavior on scale { NumberAnimation { duration: 160; easing.type: Easing.OutCubic } }

                PopupCard {
                    id: card
                    width: parent.width
                    height: card.implicitHeight

                    networksManager: root.networksManager
                    bluetoothManager: root.bluetoothManager
                    volumeManager: root.volumeManager

                    onRequestClose: root.closePopup()
                }
            }
        }
    }

    // ------------------------------------------------------------------------- ipc
    function openPopup() {
        // Keep at most one Aurora popup on screen at a time.
        if (root.bluetoothManager) root.bluetoothManager.closePopup()
        if (root.networksManager) root.networksManager.closePopup()
        if (root.volumeManager) root.volumeManager.closePopup()
        root.popupOpen = true
    }

    function closePopup() {
        root.popupOpen = false
    }

    function togglePopup() {
        if (root.popupOpen) {
            root.closePopup()
        } else {
            root.openPopup()
        }
    }

    IpcHandler {
        target: "cc"

        property bool popupOpen: root.popupOpen
        property bool wifiOn: card.wifiOn
        property bool bluetoothOn: card.btOn
        property bool airplaneMode: card.airplaneMode
        property bool micMuted: card.micReady ? card.micMuted : false
        property real sinkVolume: card.volume * 100
        property int brightness: card.brightnessPct
        property string connectedNetwork: card.wifiName
        property string connectedDevice: card.btName
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

        function toggleWifi(): void {
            card.setWifi(!card.wifiOn)
        }

        function toggleBluetooth(): void {
            card.toggleBluetooth()
        }

        function toggleAirplane(): void {
            card.setAirplane(!card.airplaneMode)
        }

        function toggleMicMute(): void {
            card.toggleMicMute()
        }

        function setVolume(percent: real): void {
            card.setVolume(percent / 100)
        }

        function setBrightness(percent: real): void {
            card.setBrightness(percent)
        }

        function openWifi(): void {
            card.openDetail("wifi")
        }

        function openBluetooth(): void {
            card.openDetail("bluetooth")
        }

        function openVolume(): void {
            card.openDetail("volume")
        }
    }
}