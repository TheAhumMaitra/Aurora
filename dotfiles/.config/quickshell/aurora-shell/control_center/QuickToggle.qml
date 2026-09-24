import QtQuick
import QtQuick.Layouts

// A compact quick-toggle tile: icon, label, sub-label and an optional detail
// chevron. `checked` flips the tile into the accent "on" state; clicking the
// main tile emits `clicked`, clicking the chevron emits `expandClicked`.
Item {
    id: root

    property bool checked: false
    property string icon: ""
    property string label: ""
    property string sub: ""
    property bool expandable: false

    signal clicked()
    signal expandClicked()

    implicitHeight: Config.tileHeight
    implicitWidth: 160

    Rectangle {
        id: tile
        anchors.fill: parent
        radius: Config.tileRadius
        border.width: 1
        color: tileMouse.hovered
            ? (root.checked ? Qt.darker(Config.accentActive, 1.15) : Config.bgHover)
            : (root.checked ? Config.accentActive : Config.bgElevated)
        border.color: root.checked
            ? Qt.alpha("#ffffff", 0.25)
            : (tileMouse.hovered ? Qt.alpha(Config.accent, 0.7) : Config.border)

        Behavior on color { ColorAnimation { duration: 130 } }
        Behavior on border.color { ColorAnimation { duration: 130 } }

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 10
            anchors.rightMargin: 6
            spacing: 10

            Rectangle {
                Layout.preferredWidth: 32
                Layout.preferredHeight: 32
                Layout.alignment: Qt.AlignVCenter
                radius: 9
                color: root.checked ? Qt.alpha("#ffffff", 0.2) : Config.accentSoft

                Text {
                    anchors.centerIn: parent
                    text: root.icon
                    font.family: Config.glyphFont
                    font.pixelSize: 15
                    color: root.checked ? "#ffffff" : Config.accentActive
                }
            }

            ColumnLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true
                spacing: 1

                Text {
                    Layout.fillWidth: true
                    text: root.label
                    font.family: Config.fontName
                    font.pixelSize: Config.textSize
                    font.bold: true
                    color: root.checked ? "#ffffff" : Config.fg
                    elide: Text.ElideRight
                }

                Text {
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignLeft
                    text: root.sub
                    font.family: Config.fontName
                    font.pixelSize: 9
                    color: root.checked ? Qt.alpha("#ffffff", 0.75) : Config.fgDim
                    elide: Text.ElideRight
                }
            }

            Rectangle {
                id: chevron
                visible: root.expandable
                z: 2
                Layout.preferredWidth: 22
                Layout.preferredHeight: 22
                Layout.alignment: Qt.AlignVCenter
                radius: 7
                color: expandHover.hovered
                    ? (root.checked ? Qt.alpha("#ffffff", 0.35) : Config.accentSoft)
                    : (root.checked ? Qt.alpha("#ffffff", 0.18) : Qt.alpha(Config.fg, 0.06))

                Text {
                    anchors.centerIn: parent
                    text: Config.gChevronRight
                    font.family: Config.glyphFont
                    font.pixelSize: 9
                    color: root.checked ? Qt.alpha("#ffffff", 0.9) : Config.fgDim
                }

                MouseArea {
                    id: expandHover
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: root.expandClicked()
                }
            }
        }

        MouseArea {
            id: tileMouse
            anchors.fill: parent
            z: 1
            hoverEnabled: true
            cursorShape: Qt.PointingHandCursor
            onClicked: root.clicked()
        }
    }
}