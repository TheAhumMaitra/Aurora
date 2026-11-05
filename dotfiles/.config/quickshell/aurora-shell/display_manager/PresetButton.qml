import QtQuick
import "."

Item {
    id: root

    property string label: ""
    property bool checked: false

    signal clicked()

    implicitHeight: 28
    implicitWidth: 46

    Rectangle {
        id: pill
        anchors.fill: parent
        radius: 7
        color: btnMouse.hovered
            ? (root.checked ? Qt.darker(Config.accentActive, 1.15) : Config.bgHover)
            : (root.checked ? Config.accentActive : Config.bgElevated)
        border.width: 1
        border.color: root.checked
            ? Qt.alpha("#ffffff", 0.3)
            : (btnMouse.hovered ? Qt.alpha(Config.accent, 0.6) : Config.border)

        Behavior on color { ColorAnimation { duration: 120 } }
        Behavior on border.color { ColorAnimation { duration: 120 } }

        Text {
            anchors.centerIn: parent
            text: root.label
            font.family: Config.fontName
            font.pixelSize: 10
            font.bold: root.checked
            color: root.checked ? "#ffffff" : Config.fgSub
        }

        MouseArea {
            id: btnMouse
            anchors.fill: parent
            hoverEnabled: true
            cursorShape: Qt.PointingHandCursor
            onClicked: root.clicked()
        }
    }
}
