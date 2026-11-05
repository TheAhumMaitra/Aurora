import QtQuick
import QtQuick.Layouts
import "."

Rectangle {
    id: root

    property string profileId: ""
    property string label: ""
    property string icon: ""
    property bool active: false

    signal selected(string id)

    implicitHeight: 38
    Layout.fillWidth: true
    radius: 8

    color: root.active
        ? Qt.alpha(Config.accentActive, 0.22)
        : (btnMouse.hovered ? Config.bgHover : Config.bgElevated)

    border.color: root.active
        ? Config.accentActive
        : (btnMouse.hovered ? Qt.alpha(Config.accent, 0.6) : Config.border)
    border.width: 1

    Behavior on color { ColorAnimation { duration: 120 } }
    Behavior on border.color { ColorAnimation { duration: 120 } }

    MouseArea {
        id: btnMouse
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: root.selected(root.profileId)
    }

    RowLayout {
        anchors.centerIn: parent
        spacing: 6

        Text {
            text: root.icon
            font.family: Config.glyphFont
            font.pixelSize: 12
            color: root.active ? Config.accentActive : Config.fgSub
        }

        Text {
            text: root.label
            font.family: Config.fontName
            font.pixelSize: 11
            font.bold: root.active
            color: root.active ? Config.fg : Config.fgSub
        }
    }
}
