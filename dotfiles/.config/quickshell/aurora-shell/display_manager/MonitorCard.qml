import QtQuick
import QtQuick.Layouts
import "helpers.js" as H
import "."

Rectangle {
    id: root

    property var monitor: null
    signal togglePower()

    readonly property string monName: root.monitor ? String(root.monitor.name || "Display") : "Display"
    readonly property string monDesc: root.monitor ? String(root.monitor.description || root.monitor.model || "") : ""
    readonly property int monWidth: root.monitor ? parseInt(root.monitor.width || 0, 10) : 0
    readonly property int monHeight: root.monitor ? parseInt(root.monitor.height || 0, 10) : 0
    readonly property real monRate: root.monitor ? parseFloat(root.monitor.refreshRate || 60) : 60
    readonly property real monScale: root.monitor ? parseFloat(root.monitor.scale || 1.0) : 1.0
    readonly property bool isFocused: root.monitor ? (root.monitor.focused === true) : false
    readonly property bool isDpmsOn: root.monitor ? (root.monitor.dpmsStatus !== false) : true
    readonly property bool isVrr: root.monitor ? (root.monitor.vrr === true || root.monitor.vrr === 1) : false
    readonly property bool isLaptop: root.monName.indexOf("eDP") !== -1

    implicitHeight: col.implicitHeight + 20
    radius: Config.tileRadius
    color: cardMouse.hovered ? Config.bgHover : Config.bgElevated
    border.width: root.isFocused ? 2 : 1
    border.color: root.isFocused
        ? Config.accentActive
        : (cardMouse.hovered ? Qt.alpha(Config.accent, 0.6) : Config.border)

    Behavior on color { ColorAnimation { duration: 120 } }
    Behavior on border.color { ColorAnimation { duration: 120 } }

    ColumnLayout {
        id: col
        anchors.fill: parent
        anchors.margins: 10
        spacing: 8
        z: 1

        // Header: Icon + Name + Badges + Power Toggle
        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Rectangle {
                Layout.preferredWidth: 32
                Layout.preferredHeight: 32
                radius: 8
                color: root.isFocused ? Qt.alpha(Config.accentActive, 0.22) : Config.accentSoft

                Text {
                    anchors.centerIn: parent
                    text: root.isLaptop ? Config.gLaptop : Config.gMonitor
                    font.family: Config.glyphFont
                    font.pixelSize: 14
                    color: root.isFocused ? Config.accentActive : Config.fg
                }
            }

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 1

                RowLayout {
                    spacing: 6

                    Text {
                        text: root.monName
                        font.family: Config.fontName
                        font.pixelSize: Config.titleSize
                        font.bold: true
                        color: Config.fg
                    }

                    // Focused Badge
                    Rectangle {
                        visible: root.isFocused
                        Layout.preferredHeight: 16
                        Layout.preferredWidth: focusedText.implicitWidth + 8
                        radius: 4
                        color: Config.accentActive

                        Text {
                            id: focusedText
                            anchors.centerIn: parent
                            text: "MAIN"
                            font.family: Config.fontName
                            font.pixelSize: 8
                            font.bold: true
                            color: "#ffffff"
                        }
                    }

                    // DPMS Status Badge
                    Rectangle {
                        Layout.preferredHeight: 16
                        Layout.preferredWidth: dpmsText.implicitWidth + 8
                        radius: 4
                        color: root.isDpmsOn ? Qt.alpha(Config.ok, 0.2) : Qt.alpha(Config.danger, 0.2)

                        Text {
                            id: dpmsText
                            anchors.centerIn: parent
                            text: root.isDpmsOn ? "ON" : "STANDBY"
                            font.family: Config.fontName
                            font.pixelSize: 8
                            font.bold: true
                            color: root.isDpmsOn ? Config.ok : Config.danger
                        }
                    }
                }

                Text {
                    Layout.fillWidth: true
                    text: root.monDesc !== "" ? root.monDesc : "Connected display"
                    font.family: Config.fontName
                    font.pixelSize: 9
                    color: Config.fgDim
                    elide: Text.ElideRight
                }
            }

            // Power Toggle Button
            Rectangle {
                Layout.preferredWidth: 28
                Layout.preferredHeight: 28
                radius: 7
                color: pwrHover.hovered ? Qt.alpha(Config.accent, 0.25) : Qt.alpha(Config.fg, 0.06)

                Text {
                    anchors.centerIn: parent
                    text: Config.gPower
                    font.family: Config.glyphFont
                    font.pixelSize: 11
                    color: root.isDpmsOn ? Config.ok : Config.fgDim
                }

                MouseArea {
                    id: pwrHover
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: root.togglePower()
                }
            }
        }

        // Details Row: Resolution & Rate Pill + Scale Pill + VRR Pill
        RowLayout {
            Layout.fillWidth: true
            spacing: 6

            // Resolution & Refresh Rate
            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 22
                radius: 5
                color: Qt.alpha(Config.fg, 0.05)
                border.width: 1
                border.color: Qt.alpha(Config.border, 0.5)

                Text {
                    anchors.centerIn: parent
                    text: root.monWidth + "x" + root.monHeight + " @ " + H.formatRate(root.monRate)
                    font.family: Config.fontName
                    font.pixelSize: 9
                    font.bold: true
                    color: Config.fg
                }
            }

            // Scale
            Rectangle {
                Layout.preferredWidth: scaleText.implicitWidth + 12
                Layout.preferredHeight: 22
                radius: 5
                color: Qt.alpha(Config.fg, 0.05)
                border.width: 1
                border.color: Qt.alpha(Config.border, 0.5)

                Text {
                    id: scaleText
                    anchors.centerIn: parent
                    text: root.monScale.toFixed(2) + "x"
                    font.family: Config.fontName
                    font.pixelSize: 9
                    color: Config.fgSub
                }
            }

            // VRR
            Rectangle {
                visible: root.isVrr
                Layout.preferredWidth: vrrText.implicitWidth + 12
                Layout.preferredHeight: 22
                radius: 5
                color: Qt.alpha(Config.accentActive, 0.15)
                border.width: 1
                border.color: Qt.alpha(Config.accentActive, 0.4)

                Text {
                    id: vrrText
                    anchors.centerIn: parent
                    text: "VRR"
                    font.family: Config.fontName
                    font.pixelSize: 9
                    font.bold: true
                    color: Config.accentActive
                }
            }
        }
    }

    MouseArea {
        id: cardMouse
        anchors.fill: parent
        hoverEnabled: true
    }
}
