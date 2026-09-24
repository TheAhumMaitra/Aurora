import QtQuick
import QtQuick.Controls
import QtQuick.Effects
import QtQuick.Layouts
import Quickshell
import Quickshell.Io
import Quickshell.Networking
import "."

Scope {
    id: root

    // Popup state. The window is only mapped while the popup is open: a mapped
    // full screen layer surface keeps capturing pointer input.
    property bool popupOpen: false

    // ----------------------------------------------------------------- device discovery
    property var wifiDevice: null

    function refreshDevices() {
        var devices = Networking.devices.values
        var w = null
        for (var i = 0; i < devices.length; ++i) {
            var d = devices[i]
            if (!w && d.type === DeviceType.Wifi) w = d
        }
        root.wifiDevice = w
    }

    Component.onCompleted: {
        root.refreshDevices()
    }

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

    Connections {
        target: Networking.devices
        function onObjectInsertedPost(object, index) { root.refreshDevices() }
        function onObjectRemovedPost(object, index) { root.refreshDevices() }
    }

    property string connectedNetwork: {
        if (!root.wifiDevice) return ""
        var nets = root.wifiDevice.networks.values
        for (var i = 0; i < nets.length; ++i) {
            if (nets[i].connected) return nets[i].name
        }
        return ""
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
                anchors.leftMargin: 0
                anchors.rightMargin: 0

                shadowEnabled: true
                shadowBlur: 0.85
                shadowVerticalOffset: 2
                shadowColor: "#cc000000"

                opacity: root.popupOpen ? 1 : 0
                scale: root.popupOpen ? 1 : 0.97

                Behavior on opacity { NumberAnimation { duration: 150; easing.type: Easing.OutCubic } }
                Behavior on scale { NumberAnimation { duration: 150; easing.type: Easing.OutCubic } }

                PopupCard {
                    id: card
                    width: parent.width
                    height: card.implicitHeight

                    wifiDevice: root.wifiDevice

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
        target: "wifi"

        property bool popupOpen: root.popupOpen
        property bool wifiOn: Networking.wifiEnabled
        property bool airplaneMode: card.airplaneMode
        property int networksVisible: root.wifiDevice ? root.wifiDevice.networks.values.length : 0
        property string currentNetwork: root.connectedNetwork
        property int cardWidth: card.implicitWidth
        property int cardHeight: card.implicitHeight
        property int cardRows: card.rowCount

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
            if (root.wifiDevice) {
                root.wifiDevice.scannerEnabled = false
                root.wifiDevice.scannerEnabled = true
            }
        }

        function toggleAirplane(): void {
            card.toggleAirplane()
        }

        function setWifi(enabled: bool): void {
            card.setWifi(enabled)
        }
    }
}