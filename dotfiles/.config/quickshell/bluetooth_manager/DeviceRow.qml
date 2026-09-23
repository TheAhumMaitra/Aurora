import QtQuick
import QtQuick.Layouts
import Quickshell.Bluetooth
import "helpers.js" as H

Item {
    id: root

    required property var device

    signal clicked(var device)

    implicitHeight: Config.rowHeight
    width: parent ? parent.width : Config.popupWidth

    readonly property bool isConnected: root.device && root.device.connected
    readonly property bool busy: root.device &&
        (root.device.pairing
         || root.device.state === BluetoothDeviceState.Connecting
         || root.device.state === BluetoothDeviceState.Disconnecting)
    readonly property bool isPaired: root.device && root.device.paired && !root.device.connected

    readonly property string subtitle: root.device
        ? (root.device.connected && root.device.batteryAvailable
           ? "Connected  \u00b7  " + H.pct(root.device.battery) + "%"
           : H.deviceSubLabel(root.device))
        : ""

    Rectangle {
        id: bg
        anchors.fill: parent
        anchors.margins: 2
        radius: Config.radiusSm
        color: root.isConnected ? Qt.alpha(Config.ok, 0.12)
              : rootMouse.hovered ? Config.bgHover
                                   : "transparent"

        Behavior on color { ColorAnimation { duration: 120 } }
    }

    RowLayout {
        anchors.fill: parent
        anchors.leftMargin: 14
        anchors.rightMargin: 14
        spacing: 12

        Text {
            Layout.alignment: Qt.AlignVCenter
            font.family: Config.glyphFont
            font.pixelSize: 9
            text: H.deviceGlyph(root.device ? root.device.icon : "")
            color: root.isConnected ? Config.ok : Config.fgDim
        }

        ColumnLayout {
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignVCenter
            spacing: 2

            Text {
                Layout.fillWidth: true
                text: root.device ? (root.device.name || root.device.deviceName || root.device.address) : ""
                font.family: Config.fontName
                font.pixelSize: Config.textSize
                font.bold: root.isConnected
                color: root.isConnected ? Config.ok : Config.fg
                elide: Text.ElideRight
            }

            Text {
                Layout.fillWidth: true
                text: root.subtitle
                font.family: Config.fontName
                font.pixelSize: 9
                color: root.isConnected ? Config.ok : Config.fgDim
                elide: Text.ElideRight
            }
        }

        Text {
            Layout.alignment: Qt.AlignVCenter
            visible: root.busy
            text: Config.gSpinner
            font.family: Config.glyphFont
            font.pixelSize: 11
            color: Config.accent

            NumberAnimation on rotation {
                running: root.busy
                from: 0
                to: 360
                duration: 900
                loops: Animation.Infinite
            }
        }

        Text {
            Layout.alignment: Qt.AlignVCenter
            visible: !root.busy && root.isPaired
            text: Config.gBookmark
            font.family: Config.glyphFont
            font.pixelSize: 11
            color: Config.fgDim
        }

        Text {
            Layout.alignment: Qt.AlignVCenter
            font.family: Config.glyphFont
            font.pixelSize: 12
            visible: root.isConnected
            text: Config.gCheck
            color: Config.ok
        }
    }

    MouseArea {
        id: rootMouse
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: root.clicked(root.device)
    }
}