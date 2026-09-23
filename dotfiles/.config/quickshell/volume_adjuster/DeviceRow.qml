import QtQuick
import QtQuick.Layouts
import Quickshell.Services.Pipewire
import "helpers.js" as H

Item {
    id: root

    required property var node
    required property bool isSource

    signal requestDefault()
    signal toggleMute()

    implicitHeight: Config.rowHeight
    width: parent ? parent.width : Config.popupWidth

    readonly property var nodeAudio: root.node ? root.node.audio : null
    readonly property bool present: !!root.nodeAudio
    readonly property bool muted: root.present && root.nodeAudio.muted
    readonly property bool boosted: root.present && root.nodeAudio.volume > 1.001
    readonly property bool isDefault: {
        if (!root.node) return false
        return root.isSource
            ? (Pipewire.defaultAudioSource === root.node)
            : (Pipewire.defaultAudioSink === root.node)
    }

    // While the user drags the slider, keep a local value so the handle tracks
    // without waiting on a pipewire round-trip. The node volume is written live
    // during the drag and committed on release (the binding then resumes).
    property bool sliding: false
    property real slideValue: 0

    readonly property real sliderValue: root.sliding
        ? root.slideValue
        : (root.present ? root.nodeAudio.volume : 0)

    function commit(value) {
        root.slideValue = value
        root.sliding = true
        if (root.present) root.nodeAudio.volume = value
    }

    function stopSlide() {
        root.sliding = false
        if (root.present) root.nodeAudio.volume = root.slideValue
    }

    function iconGlyph() {
        if (root.muted) return root.isSource ? Config.gMicMute : Config.gVolumeOff
        if (root.isSource) return Config.gMic
        if (root.sliderValue >= 1) return Config.gVolumeHigh
        if (root.sliderValue > 0.01) return Config.gVolumeLow
        return Config.gVolumeOff
    }

    function iconColor() {
        if (root.muted) return Config.fgDim
        return Config.fgSub
    }

    Rectangle {
        id: bg
        anchors.fill: parent
        anchors.margins: 2
        radius: Config.radiusSm
        color: root.isDefault ? Config.accentSoft
              : rowHover.hovered ? Config.bgHover
                                  : "transparent"

        Behavior on color { ColorAnimation { duration: 120 } }
    }

    RowLayout {
        anchors.fill: parent
        anchors.leftMargin: 14
        anchors.rightMargin: 12
        spacing: 12
        z: 1

        Text {
            Layout.alignment: Qt.AlignVCenter
            font.family: Config.glyphFont
            font.pixelSize: 13
            text: root.iconGlyph()
            color: root.iconColor()

            Behavior on color { ColorAnimation { duration: 120 } }
        }

        ColumnLayout {
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignVCenter
            spacing: 7

            RowLayout {
                Layout.fillWidth: true
                spacing: 6

                Text {
                    Layout.fillWidth: true
                    text: H.deviceLabel(root.node)
                    font.family: Config.fontName
                    font.pixelSize: Config.textSize
                    font.bold: root.isDefault
                    color: Config.fg
                    elide: Text.ElideRight
                }

                Text {
                    visible: root.isDefault
                    text: Config.gCheck
                    font.family: Config.glyphFont
                    font.pixelSize: 9
                    color: Config.ok
                }
            }

            VolumeSlider {
                id: slider
                Layout.fillWidth: true
                from: 0
                to: Config.maxVolume
                value: root.sliderValue
                onUserSet: function (value) { root.commit(value) }
                onPressedChanged: if (!pressed) root.stopSlide()
            }
        }

        ColumnLayout {
            Layout.alignment: Qt.AlignVCenter
            spacing: 6

            Text {
                Layout.alignment: Qt.AlignHCenter
                text: H.pctText(root.sliderValue)
                font.family: Config.fontName
                font.pixelSize: 10
                font.bold: root.boosted
                color: Config.fg
                horizontalAlignment: Text.AlignHCenter
            }

            RowLayout {
                Layout.alignment: Qt.AlignHCenter
                spacing: 6

                Rectangle {
                    width: 26
                    height: 22
                    radius: 6
                    color: defHover.hovered ? Config.bgHover
                          : root.isDefault ? Qt.alpha(Config.accentActive, 0.18)
                                           : Config.bgElevated
                    border.color: root.isDefault ? Config.accentActive : Config.border
                    border.width: 1

                    Text {
                        anchors.centerIn: parent
                        text: root.isDefault ? Config.gDotCircle : Config.gCircle
                        font.family: Config.glyphFont
                        font.pixelSize: 10
                        color: root.isDefault ? Config.accentActive
                              : defHover.hovered ? Config.fg
                                                 : Config.fgDim
                    }

                    MouseArea {
                        id: defHover
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: {
                            if (root.present && !root.isDefault) root.requestDefault()
                        }
                    }
                }

                Rectangle {
                    width: 26
                    height: 22
                    radius: 6
                    color: muteHover.hovered ? Config.bgHover
                          : root.muted ? Qt.alpha(Config.danger, 0.22)
                                       : Config.bgElevated
                    border.color: root.muted ? Config.danger : Config.border
                    border.width: 1

                    Text {
                        anchors.centerIn: parent
                        text: root.isSource ? Config.gMicMute : Config.gVolumeOff
                        font.family: Config.glyphFont
                        font.pixelSize: 11
                        color: root.muted ? Config.danger : Config.fgDim
                    }

                    MouseArea {
                        id: muteHover
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: root.toggleMute()
                    }
                }
            }
        }
    }

    // Row-wide hit area for "make this the default device". It sits below the
    // slider / mute button (declared after), so those keep their own handlers.
    MouseArea {
        id: rowHover
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: {
            if (root.present && !root.isDefault) root.requestDefault()
        }
    }
}