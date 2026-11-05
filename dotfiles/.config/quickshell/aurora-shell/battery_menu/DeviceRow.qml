import QtQuick
import QtQuick.Layouts
import "."
import "helpers.js" as H

Rectangle {
    id: root

    property string modelName: ""
    property int percentage: 0
    property string deviceType: "device"

    implicitHeight: 38
    Layout.fillWidth: true
    radius: 7
    color: Config.bgElevated
    border.color: Config.border
    border.width: 1

    RowLayout {
        anchors.fill: parent
        anchors.leftMargin: 10
        anchors.rightMargin: 10
        spacing: 8

        Text {
            text: H.peripheralGlyph(root.deviceType)
            font.family: Config.glyphFont
            font.pixelSize: 13
            color: Config.accentActive
        }

        Text {
            Layout.fillWidth: true
            text: root.modelName
            font.family: Config.fontName
            font.pixelSize: 11
            color: Config.fg
            elide: Text.ElideRight
        }

        // Percentage Badge
        Rectangle {
            Layout.preferredWidth: 44
            Layout.preferredHeight: 22
            radius: 5
            color: Qt.alpha(H.levelColor(root.percentage, false, false), 0.18)
            border.color: Qt.alpha(H.levelColor(root.percentage, false, false), 0.5)
            border.width: 1

            Text {
                anchors.centerIn: parent
                text: root.percentage + "%"
                font.family: Config.fontName
                font.pixelSize: 10
                font.bold: true
                color: H.levelColor(root.percentage, false, false)
            }
        }
    }
}
