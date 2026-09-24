import QtQuick
import QtQuick.Layouts
import "helpers.js" as H

// A full-width control row: a tappable glyph button, a draggable slider and a
// right-aligned value label. Dragging emits `userSet` but never clobbers the
// external `value` binding; the handle tracks the drag live for smoothness.
Item {
    id: root

    signal userSet(real value)
    signal iconClicked()

    property string glyph: ""
    property string valueText: ""
    property real value: 0
    property real from: 0
    property real to: 1
    property color iconColor: Config.fg
    property color iconBg: Config.accentSoft
    property color trackColor: Qt.alpha(Config.fg, 0.16)
    property color fillColor: Config.accentActive

    implicitHeight: Config.sliderHeight
    implicitWidth: 320

    RowLayout {
        anchors.fill: parent
        spacing: 10

        Rectangle {
            Layout.preferredWidth: 32
            Layout.preferredHeight: 32
            Layout.alignment: Qt.AlignVCenter
            radius: Config.controlRadius
            color: root.iconBg

            Text {
                anchors.centerIn: parent
                text: root.glyph
                font.family: Config.glyphFont
                font.pixelSize: 13
                color: root.iconColor
            }

            MouseArea {
                id: iconMouse
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: Qt.PointingHandCursor
                onClicked: root.iconClicked()
            }
        }

        SliderBar {
            id: bar
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignVCenter
            value: root.value
            from: root.from
            to: root.to
            trackColor: root.trackColor
            fillColor: root.fillColor
            onUserSet: function (v) { root.userSet(v) }
        }

        Text {
            Layout.alignment: Qt.AlignVCenter
            Layout.preferredWidth: 40
            text: root.valueText
            font.family: Config.fontName
            font.pixelSize: 10
            color: Config.fgSub
            horizontalAlignment: Text.AlignRight
        }
    }

    component SliderBar: Item {
        id: bar

        property real value: 0
        property real from: 0
        property real to: 1
        property color trackColor
        property color fillColor

        signal userSet(real value)

        // While dragging, track the finger/follow the cursor instead of the
        // externally bound value so the handle never lags behind.
        readonly property real dragValue: drag.fraction < 0 ? -1
            : bar.from + drag.fraction * (bar.to - bar.from)
        readonly property real effectiveValue: dragValue >= 0 ? dragValue : bar.value
        readonly property real fraction: bar.from >= bar.to ? 0
            : H.clamp((bar.effectiveValue - bar.from) / (bar.to - bar.from), 0, 1)

        implicitHeight: 22
        implicitWidth: 200

        Rectangle {
            id: track
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.verticalCenter: parent.verticalCenter
            height: 4
            radius: 2
            color: bar.trackColor
        }

        Rectangle {
            id: fill
            x: track.x
            width: bar.fraction * track.width
            height: track.height
            anchors.verticalCenter: track.verticalCenter
            radius: 2
            color: bar.fillColor

            Behavior on color { ColorAnimation { duration: 140 } }
        }

        Rectangle {
            id: handle
            width: 13
            height: 19
            radius: 5
            y: (bar.height - height) / 2
            x: track.x + bar.fraction * track.width - width / 2
            color: drag.pressed ? Config.accentActive : Config.fg

            Behavior on color { ColorAnimation { duration: 120 } }

            Rectangle {
                width: 5
                height: 5
                radius: 2.5
                anchors.centerIn: parent
                color: drag.pressed ? "#ffffff" : Config.bg
            }
        }

        MouseArea {
            id: drag
            anchors.fill: parent
            cursorShape: Qt.PointingHandCursor

            property real fraction: -1

            function valueFromMouse(x) {
                var usable = bar.width - handle.width
                return H.clamp((x - handle.width / 2) / usable, 0, 1)
            }

            onPressed: {
                drag.fraction = valueFromMouse(mouse.x)
                bar.userSet(bar.from + drag.fraction * (bar.to - bar.from))
            }

            onPositionChanged: {
                if (pressed) {
                    drag.fraction = valueFromMouse(mouse.x)
                    bar.userSet(bar.from + drag.fraction * (bar.to - bar.from))
                }
            }

            onReleased: drag.fraction = -1
        }
    }
}