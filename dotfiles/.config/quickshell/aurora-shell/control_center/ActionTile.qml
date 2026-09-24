import QtQuick

// A compact square action button used for one-shot launcher tiles. Hovering
// lifts the background; `checked` flips it into the accent "on" state.
Item {
    id: root

    property string glyph: ""
    property bool checked: false

    signal clicked()

    implicitWidth: 52
    implicitHeight: 46

    Rectangle {
        id: tile
        anchors.fill: parent
        radius: Config.tileRadius
        color: tileMouse.hovered
            ? (root.checked ? Qt.darker(Config.accentActive, 1.15) : Config.bgHover)
            : (root.checked ? Config.accentActive : Config.bgElevated)
        border.width: 1
        border.color: root.checked
            ? Qt.alpha("#ffffff", 0.25)
            : (tileMouse.hovered ? Qt.alpha(Config.accent, 0.7) : Config.border)

        Behavior on color { ColorAnimation { duration: 130 } }
        Behavior on border.color { ColorAnimation { duration: 130 } }

        Text {
            anchors.centerIn: parent
            text: root.glyph
            font.family: Config.glyphFont
            font.pixelSize: 16
            color: root.checked ? "#ffffff" : Config.accentActive
        }

        MouseArea {
            id: tileMouse
            anchors.fill: parent
            hoverEnabled: true
            cursorShape: Qt.PointingHandCursor
            onClicked: root.clicked()
        }
    }
}