import QtQuick
import QtQuick.Controls
import QtQuick.Effects
import QtQuick.Layouts
import Quickshell
import Quickshell.Io
import Quickshell.Networking
import "."

ShellRoot {
    id: root

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
        visible: true
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

                opacity: menuWindow.visible ? 1 : 0
                scale: menuWindow.visible ? 1 : 0.97

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
        menuWindow.visible = true
    }

    function closePopup() {
        menuWindow.visible = false
    }

    function togglePopup() {
        menuWindow.visible = !menuWindow.visible
    }

    IpcHandler {
        target: "wifi"

        property bool popupOpen: menuWindow.visible
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