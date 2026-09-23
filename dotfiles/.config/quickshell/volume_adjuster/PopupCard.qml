import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Quickshell.Services.Pipewire
import "helpers.js" as H

Rectangle {
    id: root

    signal requestClose()

    implicitWidth: Config.popupWidth
    implicitHeight: body.implicitHeight + 32
    width: Config.popupWidth
    radius: Config.radius
    color: Config.bg
    border.color: Config.border
    border.width: 1

    // Whether the popup window backing this card is visible.
    property bool active: false

    // Sorted device lists (audio sinks / sources, streams and monitors skipped).
    property var sinks: []
    property var mics: []
    readonly property int sinkCount: root.sinks.length
    readonly property int micCount: root.mics.length

    // Currently selected devices (for IPC quick actions).
    readonly property var mainSink: Pipewire.defaultAudioSink || null
    readonly property var mainMic: Pipewire.defaultAudioSource || null

    // All device nodes we want to observe. Tracking them force-binds the
    // pipewire nodes so their audio params (volume / mute) get loaded and
    // updates are delivered; without this they stay unbound and read 0.
    readonly property var trackedNodes: {
        var out = root.sinks.concat(root.mics)
        if (root.mainSink && out.indexOf(root.mainSink) === -1) out.push(root.mainSink)
        if (root.mainMic && out.indexOf(root.mainMic) === -1) out.push(root.mainMic)
        return out
    }

    PwObjectTracker {
        objects: root.trackedNodes
    }

    // ----------------------------------------------------------- device rebuild
    function rebuild() {
        var sinks = []
        var mics = []
        var nodes = Pipewire.nodes.values
        for (var i = 0; i < nodes.length; ++i) {
            var kind = H.classify(nodes[i])
            if (kind === "sink" || kind === "both") sinks.push(nodes[i])
            if (kind === "source" || kind === "both") mics.push(nodes[i])
        }
        sinks.sort(function (a, b) {
            var da = (a === Pipewire.defaultAudioSink) ? 0 : 1
            var db = (b === Pipewire.defaultAudioSink) ? 0 : 1
            if (da !== db) return da - db
            return H.deviceLabel(a).localeCompare(H.deviceLabel(b))
        })
        mics.sort(function (a, b) {
            var da = (a === Pipewire.defaultAudioSource) ? 0 : 1
            var db = (b === Pipewire.defaultAudioSource) ? 0 : 1
            if (da !== db) return da - db
            return H.deviceLabel(a).localeCompare(H.deviceLabel(b))
        })
        root.sinks = sinks
        root.mics = mics
    }

    Connections {
        target: Pipewire.nodes
        function onValuesChanged() { root.rebuild() }
        function onObjectInsertedPost() { root.rebuild() }
        function onObjectRemovedPost() { root.rebuild() }
    }

    Connections {
        target: Pipewire
        function onReadyChanged() { root.rebuild() }
    }

    // Periodic re-sync while the popup is open catches async name/property
    // updates (device hotplug, wireplumber quirks) and default switches.
    Timer {
        interval: 2500
        running: root.active
        onTriggered: root.rebuild()
    }

    Component.onCompleted: root.rebuild()

    // -------------------------------------------------------------- helpers/ipc
    function setDefaultSink(node) {
        if (node) Pipewire.preferredDefaultAudioSink = node
    }

    function setDefaultMic(node) {
        if (node) Pipewire.preferredDefaultAudioSource = node
    }

    function toggleMute(node) {
        if (!node || !node.audio) return
        node.audio.muted = !node.audio.muted
    }

    // ----------------------------------------------------------------------- UI
    ColumnLayout {
        id: body
        anchors.fill: parent
        anchors.margins: 16
        spacing: 10

        // header ------------------------------------------------------------
        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Text {
                text: Config.gMusic
                font.family: Config.glyphFont
                font.pixelSize: Config.glyphSize
                color: Config.accentActive
            }

            Text {
                text: "Volume"
                font.family: Config.fontName
                font.pixelSize: Config.titleSize
                font.bold: true
                color: Config.fg
            }

            Item {
                Layout.fillWidth: true
            }

            Rectangle {
                width: 26
                height: 26
                radius: 7
                color: closeHover.hovered ? Config.bgHover : "transparent"

                Text {
                    anchors.centerIn: parent
                    text: Config.gTimes
                    font.family: Config.glyphFont
                    font.pixelSize: 12
                    color: Config.fgDim
                }

                MouseArea {
                    id: closeHover
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: root.requestClose()
                }
            }
        }

        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: Config.border
            opacity: 0.6
        }

        // output ------------------------------------------------------------
        RowLayout {
            Layout.fillWidth: true
            spacing: 6

            Text {
                text: "Output"
                font.family: Config.fontName
                font.pixelSize: 10
                font.bold: true
                color: Config.fgSub
            }

            Item {
                Layout.fillWidth: true
            }

            Text {
                text: root.sinkCount + " device" + (root.sinkCount === 1 ? "" : "s")
                font.family: Config.fontName
                font.pixelSize: 9
                color: Config.fgDim
            }
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: root.sinkCount > 0
                ? Math.min(Config.maxListHeight, root.sinkCount * Config.rowHeight)
                : Config.emptyListHeight
            radius: Config.radiusSm
            color: Config.bgElevated
            border.color: Config.border
            border.width: 1
            clip: true

            ListView {
                id: sinkList
                anchors.fill: parent
                model: root.sinks

                ScrollBar.vertical: ScrollBar {
                    visible: sinkList.contentHeight > Config.maxListHeight
                }

                delegate: DeviceRow {
                    required property var modelData
                    node: modelData
                    isSource: false
                    width: sinkList.width
                    onRequestDefault: root.setDefaultSink(modelData)
                    onToggleMute: root.toggleMute(modelData)
                }
            }

            Text {
                anchors.centerIn: parent
                visible: root.sinkCount === 0
                text: "No output devices"
                font.family: Config.fontName
                font.pixelSize: Config.textSize
                color: Config.fgDim
            }
        }

        // input -------------------------------------------------------------
        RowLayout {
            Layout.fillWidth: true
            spacing: 6

            Text {
                text: "Input"
                font.family: Config.fontName
                font.pixelSize: 10
                font.bold: true
                color: Config.fgSub
            }

            Item {
                Layout.fillWidth: true
            }

            Text {
                text: root.micCount + " device" + (root.micCount === 1 ? "" : "s")
                font.family: Config.fontName
                font.pixelSize: 9
                color: Config.fgDim
            }
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: root.micCount > 0
                ? Math.min(Config.maxListHeight, root.micCount * Config.rowHeight)
                : Config.emptyListHeight
            radius: Config.radiusSm
            color: Config.bgElevated
            border.color: Config.border
            border.width: 1
            clip: true

            ListView {
                id: micList
                anchors.fill: parent
                model: root.mics

                ScrollBar.vertical: ScrollBar {
                    visible: micList.contentHeight > Config.maxListHeight
                }

                delegate: DeviceRow {
                    required property var modelData
                    node: modelData
                    isSource: true
                    width: micList.width
                    onRequestDefault: root.setDefaultMic(modelData)
                    onToggleMute: root.toggleMute(modelData)
                }
            }

            Text {
                anchors.centerIn: parent
                visible: root.micCount === 0
                text: "No input devices"
                font.family: Config.fontName
                font.pixelSize: Config.textSize
                color: Config.fgDim
            }
        }
    }
}