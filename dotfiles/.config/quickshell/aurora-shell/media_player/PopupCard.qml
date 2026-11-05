import QtQuick
import QtQuick.Controls
import QtQuick.Effects
import QtQuick.Layouts
import Quickshell
import Quickshell.Io
import Quickshell.Services.Mpris
import "helpers.js" as H
import "."

Item {
    id: root

    signal requestClose()

    implicitWidth: Config.popupWidth
    implicitHeight: body.implicitHeight + 36
    width: Config.popupWidth

    // ------------------------------------------------------------- player state
    property var manualPlayer: null

    readonly property var playersList: Mpris.players ? Mpris.players.values : []

    readonly property var activePlayer: {
        var list = root.playersList
        if (!list || list.length === 0) return null

        // Check if manual selection is still valid
        if (root.manualPlayer) {
            for (var m = 0; m < list.length; m++) {
                if (list[m] && list[m].uniqueId === root.manualPlayer.uniqueId) {
                    return list[m]
                }
            }
        }

        // Prefer currently playing player
        for (var i = 0; i < list.length; i++) {
            if (list[i] && list[i].playbackState === MprisPlaybackState.Playing) {
                return list[i]
            }
        }

        // Otherwise pick first player with track information
        for (var j = 0; j < list.length; j++) {
            if (list[j] && list[j].trackTitle && list[j].trackTitle !== "") {
                return list[j]
            }
        }

        return list[0]
    }

    readonly property bool hasPlayer: root.activePlayer !== null
    readonly property bool isPlaying: root.activePlayer ? root.activePlayer.playbackState === MprisPlaybackState.Playing : false
    readonly property string title: root.activePlayer && root.activePlayer.trackTitle !== "" ? root.activePlayer.trackTitle : "No Media Playing"
    readonly property string artist: {
        if (!root.activePlayer) return ""
        if (root.activePlayer.trackArtists && root.activePlayer.trackArtists.length > 0) return root.activePlayer.trackArtists.join(", ")
        if (root.activePlayer.trackArtist) return root.activePlayer.trackArtist
        return root.activePlayer.identity || ""
    }
    readonly property string album: root.activePlayer && root.activePlayer.trackAlbum ? root.activePlayer.trackAlbum : ""
    readonly property string artUrl: root.activePlayer && root.activePlayer.trackArtUrl ? H.cleanArtUrl(root.activePlayer.trackArtUrl) : ""
    readonly property string identity: root.activePlayer && root.activePlayer.identity ? root.activePlayer.identity : "Media"

    // ------------------------------------------------------ position tracking
    property real currentPosition: root.activePlayer && root.activePlayer.position ? root.activePlayer.position : 0
    readonly property real currentLength: root.activePlayer && root.activePlayer.length ? root.activePlayer.length : 0

    onActivePlayerChanged: {
        if (root.activePlayer && root.activePlayer.position) {
            root.currentPosition = root.activePlayer.position
        } else {
            root.currentPosition = 0
        }
    }

    Connections {
        target: root.activePlayer
        function onPositionChanged() {
            if (root.activePlayer && !scrubber.isDragging) {
                root.currentPosition = root.activePlayer.position
            }
        }
    }

    // Smooth position interpolation while playing
    Timer {
        id: posTimer
        interval: 500
        running: root.isPlaying && !scrubber.isDragging
        repeat: true
        onTriggered: {
            if (root.activePlayer && root.activePlayer.positionSupported) {
                root.currentPosition = root.activePlayer.position
            } else if (root.isPlaying && root.currentLength > 0 && root.currentPosition < root.currentLength) {
                root.currentPosition += 0.5
            }
        }
    }

    Process {
        id: playerctlCmd
    }

    function seekTo(targetSec) {
        if (!root.activePlayer) return
        var delta = targetSec - root.currentPosition
        if (typeof root.activePlayer.seek === "function") {
            try {
                root.activePlayer.seek(delta)
            } catch(e) {
                playerctlCmd.exec(["playerctl", "-p", root.activePlayer.identity, "position", Math.round(targetSec).toString()])
            }
        } else {
            playerctlCmd.exec(["playerctl", "-p", root.activePlayer.identity, "position", Math.round(targetSec).toString()])
        }
        root.currentPosition = targetSec
    }

    function togglePlayPause() {
        if (!root.activePlayer) return
        if (typeof root.activePlayer.togglePlaying === "function") {
            root.activePlayer.togglePlaying()
        } else if (typeof root.activePlayer.playPause === "function") {
            root.activePlayer.playPause()
        } else {
            playerctlCmd.exec(["playerctl", "-p", root.activePlayer.identity, "play-pause"])
        }
    }

    function goNext() {
        if (!root.activePlayer) return
        if (typeof root.activePlayer.next === "function") {
            root.activePlayer.next()
        } else {
            playerctlCmd.exec(["playerctl", "-p", root.activePlayer.identity, "next"])
        }
    }

    function goPrevious() {
        if (!root.activePlayer) return
        if (typeof root.activePlayer.previous === "function") {
            root.activePlayer.previous()
        } else {
            playerctlCmd.exec(["playerctl", "-p", root.activePlayer.identity, "previous"])
        }
    }

    function toggleShuffle() {
        if (!root.activePlayer) return
        if (root.activePlayer.shuffleSupported) {
            root.activePlayer.shuffle = !root.activePlayer.shuffle
        } else {
            playerctlCmd.exec(["playerctl", "-p", root.activePlayer.identity, "shuffle", "Toggle"])
        }
    }

    function cycleLoop() {
        if (!root.activePlayer || !root.activePlayer.loopSupported) return
        var s = root.activePlayer.loopState
        if (s === MprisLoopState.None) root.activePlayer.loopState = MprisLoopState.Playlist
        else if (s === MprisLoopState.Playlist) root.activePlayer.loopState = MprisLoopState.Track
        else root.activePlayer.loopState = MprisLoopState.None
    }

    function raisePlayer() {
        if (!root.activePlayer) return
        if (typeof root.activePlayer.raise === "function") {
            root.activePlayer.raise()
        } else {
            playerctlCmd.exec(["playerctl", "-p", root.activePlayer.identity, "raise"])
        }
    }

    // ------------------------------------------------------------- blur backdrop
    Rectangle {
        id: cardBackdrop
        anchors.fill: parent
        radius: Config.radius
        color: Config.bg
        clip: true

        // Image layer to be blurred
        Image {
            id: bgArt
            anchors.fill: parent
            source: root.artUrl
            fillMode: Image.PreserveAspectCrop
            visible: false
            asynchronous: true
        }

        // MultiEffect GPU-accelerated blur effect
        MultiEffect {
            anchors.fill: parent
            source: bgArt
            visible: root.artUrl !== "" && bgArt.status === Image.Ready
            blurEnabled: true
            blur: 0.95
            blurMax: 64
            saturation: 0.4
            brightness: -0.42

            Behavior on opacity { NumberAnimation { duration: 300 } }
        }

        // Glass scrim overlay
        Rectangle {
            anchors.fill: parent
            radius: Config.radius
            color: root.artUrl !== "" ? Qt.alpha(Config.bg, 0.78) : Config.bgElevated
            border.color: Qt.alpha(Config.accent, 0.32)
            border.width: 1

            Behavior on color { ColorAnimation { duration: 250 } }
            Behavior on border.color { ColorAnimation { duration: 250 } }
        }
    }

    // ------------------------------------------------------------------- UI
    ColumnLayout {
        id: body
        anchors.fill: parent
        anchors.margins: 18
        spacing: 14

        // 1. Header with Player Pills ----------------------------------------
        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Text {
                text: Config.gMusic
                font.family: Config.glyphFont
                font.pixelSize: 13
                color: Config.accent
            }

            Text {
                text: "Media Hub"
                font.family: Config.fontName
                font.pixelSize: Config.titleSize
                font.bold: true
                color: Config.fg
            }

            Item { Layout.fillWidth: true }

            // Multi-Player Switcher Tabs
            RowLayout {
                spacing: 6
                visible: root.playersList.length > 1

                Repeater {
                    model: root.playersList

                    Rectangle {
                        required property var modelData
                        readonly property bool isSelected: root.activePlayer && root.activePlayer.uniqueId === modelData.uniqueId
                        readonly property bool isThisPlaying: modelData && modelData.playbackState === MprisPlaybackState.Playing

                        width: pillRow.width + 16
                        height: 24
                        radius: 12
                        color: isSelected ? Config.accent : (pillHover.hovered ? Config.bgHover : Qt.alpha(Config.fg, 0.1))

                        Behavior on color { ColorAnimation { duration: 140 } }

                        Row {
                            id: pillRow
                            anchors.centerIn: parent
                            spacing: 5

                            Text {
                                anchors.verticalCenter: parent.verticalCenter
                                text: H.playerGlyph(modelData.identity)
                                font.family: Config.glyphFont
                                font.pixelSize: 10
                                color: isSelected ? "#ffffff" : (isThisPlaying ? Config.ok : Config.fgSub)
                            }

                            Text {
                                anchors.verticalCenter: parent.verticalCenter
                                text: modelData.identity || "Player"
                                font.family: Config.fontName
                                font.pixelSize: 10
                                font.bold: isSelected
                                color: isSelected ? "#ffffff" : Config.fgSub
                            }

                            // Green playing indicator dot
                            Rectangle {
                                anchors.verticalCenter: parent.verticalCenter
                                width: 5
                                height: 5
                                radius: 2.5
                                color: isSelected ? "#ffffff" : Config.ok
                                visible: isThisPlaying
                            }
                        }

                        MouseArea {
                            id: pillHover
                            anchors.fill: parent
                            hoverEnabled: true
                            cursorShape: Qt.PointingHandCursor
                            onClicked: {
                                root.manualPlayer = modelData
                            }
                        }
                    }
                }
            }

            // Raise Player App button
            Rectangle {
                width: 26
                height: 26
                radius: 7
                color: raiseHover.hovered ? Config.bgHover : "transparent"
                visible: root.hasPlayer

                Text {
                    anchors.centerIn: parent
                    text: Config.gExternalLink
                    font.family: Config.glyphFont
                    font.pixelSize: 11
                    color: Config.fgDim
                }

                MouseArea {
                    id: raiseHover
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: root.raisePlayer()
                }
            }

            // Close button
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

        // 2. Active Player View or Empty State -------------------------------
        Item {
            Layout.fillWidth: true
            Layout.preferredHeight: root.hasPlayer ? centerContent.implicitHeight : emptyState.implicitHeight

            Behavior on Layout.preferredHeight { NumberAnimation { duration: 180; easing.type: Easing.OutCubic } }

            // Active Track View
            RowLayout {
                id: centerContent
                anchors.fill: parent
                spacing: 16
                visible: root.hasPlayer

                // Album Art
                AlbumArt {
                    id: artItem
                    Layout.alignment: Qt.AlignVCenter
                    artUrl: root.artUrl
                    isPlaying: root.isPlaying
                    size: Config.artSize
                }

                // Metadata Column
                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignVCenter
                    spacing: 4

                    // Player tag
                    RowLayout {
                        spacing: 6

                        Rectangle {
                            Layout.preferredWidth: 6
                            Layout.preferredHeight: 6
                            radius: 3
                            color: root.isPlaying ? Config.ok : Config.fgDim
                        }

                        Text {
                            text: root.identity + (root.isPlaying ? " \u00b7 Playing" : " \u00b7 Paused")
                            font.family: Config.fontName
                            font.pixelSize: 10
                            font.bold: true
                            color: root.isPlaying ? Config.ok : Config.fgDim
                        }
                    }

                    // Track title
                    Text {
                        Layout.fillWidth: true
                        text: root.title
                        font.family: Config.fontName
                        font.pixelSize: 15
                        font.bold: true
                        color: Config.fg
                        elide: Text.ElideRight
                        maximumLineCount: 2
                        wrapMode: Text.Wrap
                    }

                    // Artist
                    Text {
                        Layout.fillWidth: true
                        text: root.artist !== "" ? root.artist : "Unknown Artist"
                        font.family: Config.fontName
                        font.pixelSize: 12
                        color: Config.accent
                        elide: Text.ElideRight
                    }

                    // Album
                    Text {
                        Layout.fillWidth: true
                        text: root.album !== "" ? root.album : ""
                        visible: root.album !== ""
                        font.family: Config.fontName
                        font.pixelSize: 10
                        color: Config.fgDim
                        elide: Text.ElideRight
                    }
                }
            }

            // Empty State
            ColumnLayout {
                id: emptyState
                anchors.centerIn: parent
                spacing: 8
                visible: !root.hasPlayer

                Rectangle {
                    Layout.alignment: Qt.AlignHCenter
                    width: 56
                    height: 56
                    radius: 28
                    color: Qt.alpha(Config.accent, 0.12)
                    border.color: Qt.alpha(Config.accent, 0.25)
                    border.width: 1

                    Text {
                        anchors.centerIn: parent
                        text: Config.gMusic
                        font.family: Config.glyphFont
                        font.pixelSize: 22
                        color: Config.accent
                    }
                }

                Text {
                    Layout.alignment: Qt.AlignHCenter
                    text: "No Media Playing"
                    font.family: Config.fontName
                    font.pixelSize: Config.titleSize
                    font.bold: true
                    color: Config.fg
                }

                Text {
                    Layout.alignment: Qt.AlignHCenter
                    text: "Start playback in Spotify, browser, or media player"
                    font.family: Config.fontName
                    font.pixelSize: 10
                    color: Config.fgDim
                }
            }
        }

        // 3. Scrubber Bar ----------------------------------------------------
        Scrubber {
            id: scrubber
            Layout.fillWidth: true
            visible: root.hasPlayer
            position: root.currentPosition
            length: root.currentLength
            seekable: root.activePlayer ? root.activePlayer.canSeek : false
            onSeekRequested: function(targetSec) {
                root.seekTo(targetSec)
            }
        }

        // 4. Playback Controls Deck ------------------------------------------
        RowLayout {
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignHCenter
            visible: root.hasPlayer
            spacing: 20

            // Shuffle Button
            Rectangle {
                width: 32
                height: 32
                radius: 16
                color: shufHover.hovered ? Config.bgHover : "transparent"
                opacity: root.activePlayer && root.activePlayer.shuffleSupported ? 1.0 : 0.45

                Text {
                    anchors.centerIn: parent
                    text: Config.gShuffle
                    font.family: Config.glyphFont
                    font.pixelSize: 13
                    color: root.activePlayer && root.activePlayer.shuffle ? Config.accent : Config.fgDim
                }

                MouseArea {
                    id: shufHover
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    enabled: root.activePlayer && root.activePlayer.shuffleSupported
                    onClicked: root.toggleShuffle()
                }
            }

            // Previous Button
            Rectangle {
                width: 38
                height: 38
                radius: 19
                color: prevHover.hovered ? Config.bgHover : Qt.alpha(Config.fg, 0.08)
                opacity: root.activePlayer && root.activePlayer.canGoPrevious ? 1.0 : 0.45

                Text {
                    anchors.centerIn: parent
                    text: Config.gPrev
                    font.family: Config.glyphFont
                    font.pixelSize: 14
                    color: Config.fg
                }

                MouseArea {
                    id: prevHover
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    enabled: root.activePlayer && root.activePlayer.canGoPrevious
                    onClicked: root.goPrevious()
                }
            }

            // Central Elevated Play / Pause Button
            Rectangle {
                width: 48
                height: 48
                radius: 24
                color: playBtnHover.hovered ? Qt.darker(Config.accent, 1.12) : Config.accent
                scale: playBtnHover.pressed ? 0.94 : (playBtnHover.hovered ? 1.06 : 1.0)

                Behavior on scale { NumberAnimation { duration: 140; easing.type: Easing.OutCubic } }
                Behavior on color { ColorAnimation { duration: 140 } }

                Text {
                    anchors.centerIn: parent
                    // Slight right offset for play triangle to look optically centered
                    x: parent.width / 2 - width / 2 + (root.isPlaying ? 0 : 1.5)
                    text: root.isPlaying ? Config.gPause : Config.gPlay
                    font.family: Config.glyphFont
                    font.pixelSize: 16
                    font.bold: true
                    color: "#ffffff"
                }

                MouseArea {
                    id: playBtnHover
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: root.togglePlayPause()
                }
            }

            // Next Button
            Rectangle {
                width: 38
                height: 38
                radius: 19
                color: nextHover.hovered ? Config.bgHover : Qt.alpha(Config.fg, 0.08)
                opacity: root.activePlayer && root.activePlayer.canGoNext ? 1.0 : 0.45

                Text {
                    anchors.centerIn: parent
                    text: Config.gNext
                    font.family: Config.glyphFont
                    font.pixelSize: 14
                    color: Config.fg
                }

                MouseArea {
                    id: nextHover
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    enabled: root.activePlayer && root.activePlayer.canGoNext
                    onClicked: root.goNext()
                }
            }

            // Loop / Repeat Button
            Rectangle {
                width: 32
                height: 32
                radius: 16
                color: loopHover.hovered ? Config.bgHover : "transparent"
                opacity: root.activePlayer && root.activePlayer.loopSupported ? 1.0 : 0.45

                Text {
                    anchors.centerIn: parent
                    text: Config.gRepeat
                    font.family: Config.glyphFont
                    font.pixelSize: 13
                    color: {
                        if (!root.activePlayer || !root.activePlayer.loopSupported) return Config.fgDim
                        if (root.activePlayer.loopState === MprisLoopState.Track) return Config.accentActive
                        if (root.activePlayer.loopState === MprisLoopState.Playlist) return Config.accent
                        return Config.fgDim
                    }
                }

                MouseArea {
                    id: loopHover
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    enabled: root.activePlayer && root.activePlayer.loopSupported
                    onClicked: root.cycleLoop()
                }
            }
        }

        // 5. Bottom Volume Row -----------------------------------------------
        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: Config.border
            opacity: 0.4
            visible: root.hasPlayer
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 12
            visible: root.hasPlayer && root.activePlayer && root.activePlayer.volumeSupported

            Text {
                text: {
                    var v = root.activePlayer ? root.activePlayer.volume : 0
                    if (v <= 0.02) return Config.gVolumeOff
                    if (v < 0.5) return Config.gVolumeLow
                    return Config.gVolumeHigh
                }
                font.family: Config.glyphFont
                font.pixelSize: 13
                color: Config.fgDim

                MouseArea {
                    anchors.fill: parent
                    cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        if (root.activePlayer) {
                            root.activePlayer.volume = root.activePlayer.volume > 0 ? 0.0 : 0.8
                        }
                    }
                }
            }

            // Volume mini slider
            Item {
                id: volContainer
                Layout.fillWidth: true
                height: 16

                Rectangle {
                    anchors.left: parent.left
                    anchors.right: parent.right
                    anchors.verticalCenter: parent.verticalCenter
                    height: 4
                    radius: 2
                    color: Qt.alpha(Config.fg, 0.15)

                    Rectangle {
                        anchors.left: parent.left
                        anchors.top: parent.top
                        anchors.bottom: parent.bottom
                        width: Math.max(0, parent.width * (root.activePlayer ? H.clamp(root.activePlayer.volume, 0, 1) : 0))
                        radius: 2
                        color: Config.accent
                    }
                }

                MouseArea {
                    id: volArea
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor

                    function setVol(mouse) {
                        var val = H.clamp(mouse.x / volContainer.width, 0, 1)
                        if (root.activePlayer) root.activePlayer.volume = val
                    }

                    onPressed: function(mouse) { setVol(mouse) }
                    onPositionChanged: function(mouse) { if (pressed) setVol(mouse) }
                }
            }

            Text {
                text: Math.round((root.activePlayer ? root.activePlayer.volume : 0) * 100) + "%"
                font.family: Config.fontName
                font.pixelSize: 10
                color: Config.fgDim
                Layout.preferredWidth: 32
                horizontalAlignment: Text.AlignRight
            }
        }
    }
}
