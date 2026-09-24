import QtQuick
import "helpers.js" as H

// Minimal draggable slider. `value` stays bound to an external source; dragging
// never clobbers that binding, it only emits `userSet` so the owner writes the
// new value back and optionally tracks a local drag value for smooth tracking.
Item {
    id: root

    property real value: 0
    property real from: 0
    property real to: 1.5

    property color trackColor: Qt.alpha(Config.fg, 0.18)
    property color fillColor: Config.accentActive
    property color fillBoostColor: Config.warn
    property color handleColor: Config.fg
    property color tickColor: Config.fgSub

    // Visual fraction of the full range the current value maps to.
    readonly property real fraction: root.from >= root.to ? 0 : H.clamp((root.value - root.from) / (root.to - root.from), 0, 1)
    // Fraction of the full range at value == 1.0 (the alpha-100% boundary).
    readonly property real unitFraction: root.from >= root.to ? 0 : H.clamp((1 - root.from) / (root.to - root.from), 0, 1)
    readonly property bool boosted: root.value > 1.001
    readonly property bool pressed: drag.pressed

    signal userSet(real value)

    implicitHeight: 24
    implicitWidth: 220

    function valueFromMouse(x) {
        var usable = root.width - handle.width
        var t = (x - handle.width / 2) / usable
        t = H.clamp(t, 0, 1)
        return root.from + t * (root.to - root.from)
    }

    Rectangle {
        id: track
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter
        height: 4
        radius: 2
        color: root.trackColor
    }

    Rectangle {
        id: fill
        x: track.x
        y: track.y
        width: (track.x + root.fraction * track.width) - track.x
        height: track.height
        radius: 2
        color: root.boosted ? root.fillBoostColor : root.fillColor

        Behavior on color { ColorAnimation { duration: 150 } }
    }

    Rectangle {
        id: unitTick
        visible: root.unitFraction > 0 && root.unitFraction < 1
        x: track.x + root.unitFraction * track.width
        y: track.y - 3
        width: 1
        height: track.height + 6
        color: root.tickColor
    }

    Rectangle {
        id: handle
        width: 13
        height: 19
        radius: 5
        y: (root.height - height) / 2
        x: track.x + root.fraction * track.width - width / 2
        color: root.pressed ? Config.accentActive : root.handleColor

        Behavior on color { ColorAnimation { duration: 120 } }
    }

    MouseArea {
        id: drag
        anchors.fill: parent
        cursorShape: Qt.PointingHandCursor
        onPressed: root.userSet(root.valueFromMouse(mouse.x))
        onPositionChanged: if (pressed) root.userSet(root.valueFromMouse(mouse.x))
    }
}