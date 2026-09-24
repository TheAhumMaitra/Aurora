import QtQuick
import "helpers.js" as H

Item {
    id: root

    property real strength: 0.0
    property color activeColor: Config.accent
    property color dimColor: Qt.alpha(Config.fg, 0.15)
    property int barCount: 4
    property real barWidth: 4
    property real barGap: 3
    property real barMaxHeight: 18

    readonly property int lit: H.litBars(root.strength, root.barCount)

    implicitWidth: barCount * barWidth + (barCount - 1) * barGap
    implicitHeight: barMaxHeight

    Repeater {
        model: root.barCount

        Rectangle {
            required property int index
            width: root.barWidth
            height: root.barMaxHeight * (0.35 + 0.65 * (index + 1) / root.barCount)
            y: root.barMaxHeight - height
            x: index * (root.barWidth + root.barGap)
            radius: Math.max(1, root.barWidth / 2)
            color: index < root.lit ? root.activeColor : root.dimColor
            opacity: index < root.lit ? 1 : 0.9
            Behavior on color { ColorAnimation { duration: 180 } }
        }
    }
}