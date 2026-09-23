import QtQuick
import QtQuick.Controls
import QtQuick.Effects
import QtQuick.Layouts
import Quickshell
import Quickshell.Bluetooth
import Quickshell.Io
import "."

ShellRoot {
    id: root

    // ----------------------------------------------------------------- device discovery
    property var adapter: Bluetooth.defaultAdapter

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

                    adapter: root.adapter
                    active: menuWindow.visible

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
        target: "bluetooth"

        property bool popupOpen: menuWindow.visible
        property bool bluetoothOn: root.adapter && root.adapter.enabled
        property bool discovering: root.adapter && root.adapter.discovering
        property int devicesVisible: root.adapter && root.adapter.enabled ? card.rowCount : 0
        property int cardWidth: card.implicitWidth
        property int cardHeight: card.implicitHeight
        property int cardRows: card.rowCount

        function toggle(): void {
            if (root.adapter) root.adapter.enabled = !root.adapter.enabled
        }

        function togglePopup(): void {
            root.togglePopup()
        }

        function open(): void {
            root.openPopup()
        }

        function close(): void {
            root.closePopup()
        }

        function setEnabled(on: bool): void {
            if (root.adapter) root.adapter.enabled = on
        }

        function rescan(): void {
            if (root.adapter && root.adapter.enabled) {
                root.adapter.discovering = false
                root.adapter.discovering = true
            }
        }
    }
}