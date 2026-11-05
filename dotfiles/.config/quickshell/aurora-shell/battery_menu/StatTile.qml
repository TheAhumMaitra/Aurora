import QtQuick
import QtQuick.Layouts
import "."

Rectangle {
    id: root

    property string icon: ""
    property string label: ""
    property string value: ""
    property string sub: ""
    property color iconColor: Config.accentActive
    property color iconBg: Qt.alpha(iconColor, 0.14)
    property color valueColor: Config.fg

    implicitHeight: 54
    implicitWidth: 165
    radius: Config.radiusSm
    color: tileMouse.hovered ? Config.bgHover : Config.bgElevated
    border.color: tileMouse.hovered ? Qt.alpha(Config.accent, 0.5) : Config.border
    border.width: 1

    Behavior on color { ColorAnimation { duration: 120 } }
    Behavior on border.color { ColorAnimation { duration: 120 } }

    MouseArea {
        id: tileMouse
        anchors.fill: parent
        hoverEnabled: true
    }

    RowLayout {
        anchors.fill: parent
        anchors.margins: 8
        spacing: 9

        // Icon Box
        Rectangle {
            Layout.preferredWidth: 32
            Layout.preferredHeight: 32
            Layout.alignment: Qt.AlignVCenter
            radius: 8
            color: root.iconBg

            Text {
                anchors.centerIn: parent
                text: root.icon
                font.family: Config.glyphFont
                font.pixelSize: 13
                color: root.iconColor
            }
        }

        // Labels
        ColumnLayout {
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignVCenter
            spacing: 1

            Text {
                text: root.label
                font.family: Config.fontName
                font.pixelSize: 9
                font.bold: true
                color: Config.fgDim
                elide: Text.ElideRight
                Layout.fillWidth: true
            }

            Text {
                text: root.value
                font.family: Config.fontName
                font.pixelSize: 12
                font.bold: true
                color: root.valueColor
                elide: Text.ElideRight
                Layout.fillWidth: true
            }

            Text {
                visible: root.sub.length > 0
                text: root.sub
                font.family: Config.fontName
                font.pixelSize: 9
                color: Config.fgSub
                elide: Text.ElideRight
                Layout.fillWidth: true
            }
        }
    }
}
