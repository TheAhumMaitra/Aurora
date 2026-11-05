import QtQuick
import QtQuick.Layouts
import "helpers.js" as H
import "."

Item {
    id: root

    property real position: 0.0
    property real length: 0.0
    property bool seekable: true

    signal seekRequested(real targetSec)

    implicitHeight: 26
    Layout.fillWidth: true

    property bool isDragging: false
    property real dragFraction: 0.0

    readonly property real currentFraction: {
        if (root.isDragging) return root.dragFraction
        if (root.length <= 0) return 0
        return H.clamp(root.position / root.length, 0, 1)
    }

    readonly property real displaySeconds: {
        if (root.isDragging) return root.dragFraction * root.length
        return root.position
    }

    RowLayout {
        anchors.fill: parent
        spacing: 10

        Text {
            text: H.formatTime(root.displaySeconds)
            font.family: Config.fontName
            font.pixelSize: 10
            color: Config.fgDim
            Layout.preferredWidth: 40
            horizontalAlignment: Text.AlignLeft
        }

        Item {
            id: trackContainer
            Layout.fillWidth: true
            height: 18

            Rectangle {
                id: trackBg
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.verticalCenter: parent.verticalCenter
                height: trackHover.hovered || root.isDragging ? 6 : 4
                radius: 3
                color: Qt.alpha(Config.fg, 0.15)

                Behavior on height { NumberAnimation { duration: 120 } }

                // Filled portion
                Rectangle {
                    anchors.left: parent.left
                    anchors.top: parent.top
                    anchors.bottom: parent.bottom
                    width: Math.max(0, parent.width * root.currentFraction)
                    radius: 3
                    color: Config.accent

                    Behavior on color { ColorAnimation { duration: 150 } }
                }

                // Handle / thumb knob
                Rectangle {
                    id: handle
                    width: 12
                    height: 12
                    radius: 6
                    anchors.verticalCenter: parent.verticalCenter
                    x: Math.max(0, Math.min(parent.width - width, (parent.width * root.currentFraction) - (width / 2)))
                    color: "#ffffff"
                    border.color: Qt.alpha("#000000", 0.2)
                    border.width: 1
                    opacity: trackHover.hovered || root.isDragging ? 1.0 : 0.0
                    scale: trackHover.hovered || root.isDragging ? 1.0 : 0.6

                    Behavior on opacity { NumberAnimation { duration: 120 } }
                    Behavior on scale { NumberAnimation { duration: 120 } }
                }
            }

            MouseArea {
                id: trackHover
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: root.seekable && root.length > 0 ? Qt.PointingHandCursor : Qt.ArrowCursor
                enabled: root.seekable && root.length > 0

                function updatePos(mouse) {
                    var frac = H.clamp(mouse.x / trackContainer.width, 0, 1)
                    root.dragFraction = frac
                }

                onPressed: function(mouse) {
                    root.isDragging = true
                    updatePos(mouse)
                }

                onPositionChanged: function(mouse) {
                    if (root.isDragging) {
                        updatePos(mouse)
                    }
                }

                onReleased: function(mouse) {
                    if (root.isDragging) {
                        updatePos(mouse)
                        var target = root.dragFraction * root.length
                        root.seekRequested(target)
                        root.isDragging = false
                    }
                }

                onCanceled: {
                    root.isDragging = false
                }
            }
        }

        Text {
            text: root.length > 0 ? H.formatTime(root.length) : "--:--"
            font.family: Config.fontName
            font.pixelSize: 10
            color: Config.fgDim
            Layout.preferredWidth: 40
            horizontalAlignment: Text.AlignRight
        }
    }
}
