import QtQuick
import QtQuick.Layouts
import Quickshell
import Quickshell.Io
import Quickshell.Services.UPower
import "."

Item {
    id: root

    signal clicked()

    property int capacity: 0
    property string status: ""
    property bool isPresent: false
    readonly property bool isCharging: root.status === "Charging" || root.status === "Full"

    implicitWidth: pill.width
    implicitHeight: 28
    visible: root.isPresent

    FileView {
        id: capFile
        path: "/sys/class/power_supply/BAT0/capacity"
        preload: true
        watchChanges: true
        onLoaded: {
            var val = parseInt(text().trim(), 10)
            if (!isNaN(val)) {
                root.capacity = val
                root.isPresent = true
            }
        }
    }

    FileView {
        id: statusFile
        path: "/sys/class/power_supply/BAT0/status"
        preload: true
        watchChanges: true
        onLoaded: {
            var st = text().trim()
            if (st.length > 0) root.status = st
        }
    }

    Timer {
        interval: 5000
        running: true
        repeat: true
        onTriggered: {
            capFile.reload()
            statusFile.reload()
        }
    }

    readonly property color stateColor: {
        if (root.isCharging) return Config.ok
        if (root.capacity <= 15) return Config.danger
        if (root.capacity <= 30) return Config.warn
        return Config.accentActive
    }

    readonly property string iconGlyph: {
        if (root.isCharging) return Config.gBolt
        if (root.capacity >= 80) return Config.gBatteryFull
        if (root.capacity >= 55) return Config.gBatteryThreeQuarters
        if (root.capacity >= 30) return Config.gBatteryHalf
        if (root.capacity >= 15) return Config.gBatteryQuarter
        return Config.gBatteryEmpty
    }

    Rectangle {
        id: pill
        height: 28
        width: contentRow.implicitWidth + 18
        radius: 8
        color: pillHover.hovered ? Qt.alpha(root.stateColor, 0.24) : Qt.alpha(root.stateColor, 0.12)
        border.width: 1
        border.color: Qt.alpha(root.stateColor, 0.45)

        Behavior on color { ColorAnimation { duration: 120 } }

        RowLayout {
            id: contentRow
            anchors.centerIn: parent
            spacing: 5

            Text {
                text: root.iconGlyph
                font.family: Config.glyphFont
                font.pixelSize: 11
                color: root.stateColor
            }

            Text {
                text: root.capacity + "%"
                font.family: Config.fontName
                font.pixelSize: 10
                font.bold: true
                color: Config.fg
            }
        }

        MouseArea {
            id: pillHover
            anchors.fill: parent
            hoverEnabled: true
            cursorShape: Qt.PointingHandCursor
            onClicked: root.clicked()
        }
    }
}
