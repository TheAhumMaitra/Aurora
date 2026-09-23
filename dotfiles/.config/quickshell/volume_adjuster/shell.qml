import QtQuick
import QtQuick.Controls
import QtQuick.Effects
import QtQuick.Layouts
import Quickshell
import Quickshell.Io
import Quickshell.Services.Pipewire
import "."

ShellRoot {
    id: root

    // ------------------------------------------------------- live color reload
    // Watch colors.qml and push changes into the Colors singleton so every
    // Config binding re-evaluates without reloading the whole shell.
    FileView {
        id: colorFile
        path: Quickshell.configPath("colors.qml")
        preload: true
        watchChanges: true

        onFileChanged: reload()
        onLoaded: root.applyColors()
    }

    function applyColors() {
        var text = colorFile.text()
        if (!text) return
        var known = ["accent", "activeBackground", "activeAccent",
                     "urgentBackground", "border", "surface", "surfaceAlt",
                     "muted", "background", "foreground", "success", "warning"]
        var re = /(?:readonly\s+)?property\s+color\s+(\w+)\s*:\s*(['"])([\s\S]*?)\2/g
        var m
        while ((m = re.exec(text)) !== null) {
            if (known.indexOf(m[1]) !== -1) {
                Colors[m[1]] = m[3]
            }
        }
    }

    // -------------------------------------------------------------- popup window
    PanelWindow {
        id: menuWindow
        visible: true
        screen: Quickshell.screens[Config.screenIndex] || Quickshell.screens[0]

        anchors {
            top: true
            bottom: true
            left: true
            right: true
        }

        aboveWindows: true
        focusable: true
        exclusionMode: ExclusionMode.Ignore
        exclusiveZone: 0
        color: "transparent"

        onVisibleChanged: {
            if (visible) {
                popupContent.forceActiveFocus()
            }
        }

        Item {
            id: popupContent
            anchors.fill: parent

            focus: true
            Keys.onEscapePressed: root.closePopup()

            MouseArea {
                anchors.fill: parent
                z: 1
                onClicked: root.closePopup()
            }

            MultiEffect {
                id: cardFx
                z: 2
                x: popupContent.width - Config.popupWidth - Config.popupMargin
                y: Config.popupMarginTop
                width: Config.popupWidth
                height: card.implicitHeight

                source: card
                anchors.leftMargin: 0
                anchors.rightMargin: 0

                shadowEnabled: true
                shadowBlur: 0.85
                shadowVerticalOffset: 2
                shadowColor: "#cc000000"

                opacity: menuWindow.visible ? 1 : 0
                scale: menuWindow.visible ? 1 : 0.97

                Behavior on opacity { NumberAnimation { duration: 150; easing.type: Easing.OutCubic } }
                Behavior on scale { NumberAnimation { duration: 150; easing.type: Easing.OutCubic } }

                PopupCard {
                    id: card
                    width: parent.width
                    height: card.implicitHeight

                    active: menuWindow.visible

                    onRequestClose: root.closePopup()
                }
            }
        }
    }

    // ------------------------------------------------------------------------- ipc
    function openPopup() {
        menuWindow.visible = true
    }

    function closePopup() {
        menuWindow.visible = false
    }

    function togglePopup() {
        menuWindow.visible = !menuWindow.visible
    }

    IpcHandler {
        target: "volume"

        property bool popupOpen: menuWindow.visible
        property int sinksCount: card.sinkCount
        property int micsCount: card.micCount
        property real sinkVolume: card.mainSink && card.mainSink.audio ? card.mainSink.audio.volume * 100 : 0
        property real micVolume: card.mainMic && card.mainMic.audio ? card.mainMic.audio.volume * 100 : 0
        property bool sinkMuted: card.mainSink && card.mainSink.audio ? card.mainSink.audio.muted : false
        property bool micMuted: card.mainMic && card.mainMic.audio ? card.mainMic.audio.muted : false
        property int cardWidth: card.implicitWidth
        property int cardHeight: card.implicitHeight

        function toggle(): void {
            root.togglePopup()
        }

        function open(): void {
            root.openPopup()
        }

        function close(): void {
            root.closePopup()
        }

        function toggleSinkMute(): void {
            card.toggleMute(card.mainSink)
        }

        function toggleMicMute(): void {
            card.toggleMute(card.mainMic)
        }

        function setSinkVolume(percent: real): void {
            if (card.mainSink && card.mainSink.audio) {
                card.mainSink.audio.volume = Math.max(0, Math.min(Config.maxVolume, percent / 100))
            }
        }

        function setMicVolume(percent: real): void {
            if (card.mainMic && card.mainMic.audio) {
                card.mainMic.audio.volume = Math.max(0, Math.min(Config.maxVolume, percent / 100))
            }
        }
    }
}