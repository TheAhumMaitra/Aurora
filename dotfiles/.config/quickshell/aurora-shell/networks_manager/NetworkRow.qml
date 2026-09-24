import QtQuick
import QtQuick.Layouts
import "helpers.js" as H

Item {
    id: root

    required property var network

    signal clicked(var network)

    implicitHeight: Config.rowHeight
    width: parent ? parent.width : Config.popupWidth

    readonly property bool connecting: root.network && root.network.stateChanging
    readonly property bool isConnected: root.network && root.network.connected
    readonly property int strengthPct: root.network ? Math.round(root.network.signalStrength * 100) : 0

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
            text: root.isConnected ? Config.gCircle : Config.gCircleO
            color: root.isConnected ? Config.ok : Config.fgDim
        }

        ColumnLayout {
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignVCenter
            spacing: 2

            Text {
                Layout.fillWidth: true
                text: root.network ? root.network.name : ""
                font.family: Config.fontName
                font.pixelSize: Config.textSize
                font.bold: root.isConnected
                color: root.isConnected ? Config.ok : Config.fg
                elide: Text.ElideRight
            }

            Text {
                Layout.fillWidth: true
                text: root.isConnected
                      ? "Connected  \u00b7  " + strengthPct + "%"
                      : H.securityLabel(root.network ? root.network.security : 0)
                        + "  \u00b7  " + strengthPct + "%"
                font.family: Config.fontName
                font.pixelSize: 9
                color: root.isConnected ? Config.ok : Config.fgDim
                elide: Text.ElideRight
            }
        }

        Text {
            Layout.alignment: Qt.AlignVCenter
            visible: root.connecting
            text: Config.gSpinner
            font.family: Config.glyphFont
            font.pixelSize: 11
            color: Config.accent

            NumberAnimation on rotation {
                running: root.connecting
                from: 0
                to: 360
                duration: 900
                loops: Animation.Infinite
            }
        }

        Text {
            Layout.alignment: Qt.AlignVCenter
            visible: !root.connecting && root.network && !H.isOpen(root.network.security) && !root.isConnected
            text: Config.gLock
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
        onClicked: root.clicked(root.network)
    }
}