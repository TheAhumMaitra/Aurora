import QtQuick
import QtQuick.Layouts
import "."
import "helpers.js" as H

Item {
    id: root

    property int percentage: 0
    property bool charging: false
    property bool acOnline: false
    property color activeColor: H.levelColor(root.percentage, root.charging, root.acOnline)

    implicitWidth: 140
    implicitHeight: 46

    RowLayout {
        anchors.fill: parent
        spacing: 3

        // Main Battery Body
        Rectangle {
            id: batteryBody
            Layout.fillWidth: true
            Layout.fillHeight: true
            radius: 8
            color: Config.bgElevated
            border.color: root.charging
                ? Qt.alpha(root.activeColor, 0.7)
                : Qt.alpha(Config.border, 0.6)
            border.width: 1.5
            clip: true

            Behavior on border.color { ColorAnimation { duration: 250 } }

            // Inner Fill
            Rectangle {
                id: fill
                anchors.left: parent.left
                anchors.top: parent.top
                anchors.bottom: parent.bottom
                anchors.margins: 3
                width: Math.max(root.percentage > 0 ? 6 : 0,
                                Math.min(parent.width - 6, (parent.width - 6) * (root.percentage / 100.0)))
                radius: 5
                color: root.activeColor

                Behavior on width {
                    NumberAnimation {
                        duration: 350
                        easing.type: Easing.OutCubic
                    }
                }

                Behavior on color {
                    ColorAnimation {
                        duration: 300
                    }
                }

                // Charging pulse glow
                Rectangle {
                    anchors.fill: parent
                    radius: parent.radius
                    color: "#ffffff"
                    opacity: root.charging ? pulseAnim.pulseValue : 0

                    SequentialAnimation {
                        id: pulseAnim
                        property real pulseValue: 0.15
                        running: root.charging
                        loops: Animation.Infinite

                        NumberAnimation {
                            target: pulseAnim
                            property: "pulseValue"
                            from: 0.1
                            to: 0.35
                            duration: 1000
                            easing.type: Easing.InOutQuad
                        }
                        NumberAnimation {
                            target: pulseAnim
                            property: "pulseValue"
                            from: 0.35
                            to: 0.1
                            duration: 1000
                            easing.type: Easing.InOutQuad
                        }
                    }
                }
            }

            // Segment markers (25%, 50%, 75%)
            Row {
                anchors.fill: parent
                anchors.margins: 4
                spacing: 0

                Item { Layout.fillWidth: true; width: (parent.width) / 4 }
                Rectangle {
                    width: 1
                    height: parent.height
                    color: Qt.alpha(Config.bg, 0.45)
                }
                Item { Layout.fillWidth: true; width: (parent.width) / 4 }
                Rectangle {
                    width: 1
                    height: parent.height
                    color: Qt.alpha(Config.bg, 0.45)
                }
                Item { Layout.fillWidth: true; width: (parent.width) / 4 }
                Rectangle {
                    width: 1
                    height: parent.height
                    color: Qt.alpha(Config.bg, 0.45)
                }
                Item { Layout.fillWidth: true; width: (parent.width) / 4 }
            }

            // Bolt icon overlay inside fill when charging
            Text {
                anchors.centerIn: fill
                visible: root.charging && fill.width > 30
                text: Config.gBolt
                font.family: Config.glyphFont
                font.pixelSize: 14
                font.bold: true
                color: "#1e1e2e"
                z: 5
            }
        }

        // Terminal Nub
        Rectangle {
            Layout.preferredWidth: 4
            Layout.preferredHeight: 16
            Layout.alignment: Qt.AlignVCenter
            radius: 2
            color: root.charging
                ? Qt.alpha(root.activeColor, 0.8)
                : Qt.alpha(Config.border, 0.6)

            Behavior on color { ColorAnimation { duration: 250 } }
        }
    }
}
