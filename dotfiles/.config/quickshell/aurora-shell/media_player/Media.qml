import QtQuick
import QtQuick.Controls
import QtQuick.Effects
import QtQuick.Layouts
import Quickshell
import Quickshell.Io
import "."

Scope {
    id: root

    // Popup state. The window is only mapped while the popup is open: a mapped
    // full screen layer surface keeps capturing pointer input.
    property bool popupOpen: false

    // ------------------------------------------------------- live color reload
    // Watch colors.qml and push changes into the Colors singleton so every
    // Config binding re-evaluates without reloading the whole shell.
    FileView {
        id: colorFile
        path: Quickshell.shellPath("colors.qml")
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
        visible: root.popupOpen
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

                shadowEnabled: true
                shadowBlur: 0.85
                shadowVerticalOffset: 3
                shadowColor: "#dd000000"

                opacity: root.popupOpen ? 1 : 0
                scale: root.popupOpen ? 1 : 0.96

                Behavior on opacity { NumberAnimation { duration: 160; easing.type: Easing.OutCubic } }
                Behavior on scale { NumberAnimation { duration: 160; easing.type: Easing.OutCubic } }

                PopupCard {
                    id: card
                    width: parent.width
                    height: card.implicitHeight

                    onRequestClose: root.closePopup()
                }
            }
        }
    }

    // ------------------------------------------------------------------------- ipc
    function openPopup() {
        root.popupOpen = true
    }

    function closePopup() {
        root.popupOpen = false
    }

    function togglePopup() {
        if (root.popupOpen) {
            root.closePopup()
        } else {
            root.openPopup()
        }
    }

    IpcHandler {
        target: "media"

        property bool popupOpen: root.popupOpen
        property bool isPlaying: card.isPlaying
        property bool hasPlayer: card.hasPlayer
        property string title: card.title
        property string artist: card.artist
        property string album: card.album
        property string player: card.identity
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

        function togglePlayPause(): void {
            card.togglePlayPause()
        }

        function next(): void {
            card.goNext()
        }

        function previous(): void {
            card.goPrevious()
        }

        function raisePlayer(): void {
            card.raisePlayer()
        }
    }
}
