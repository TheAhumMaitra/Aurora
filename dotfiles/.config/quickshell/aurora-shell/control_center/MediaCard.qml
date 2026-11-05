import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Quickshell
import Quickshell.Services.Mpris
import "."

Rectangle {
    id: root

    signal expandClicked()

    readonly property var activePlayer: {
        var list = Mpris.players.values
        if (!list || list.length === 0) return null
        for (var i = 0; i < list.length; i++) {
            if (list[i].playbackState === MprisPlaybackState.Playing) return list[i]
        }
        return list[0]
    }

    readonly property bool hasPlayer: root.activePlayer !== null && (root.activePlayer.trackTitle !== "" || root.activePlayer.playbackState === MprisPlaybackState.Playing)
    readonly property bool isPlaying: root.activePlayer ? root.activePlayer.playbackState === MprisPlaybackState.Playing : false
    readonly property string title: root.activePlayer && root.activePlayer.trackTitle !== "" ? root.activePlayer.trackTitle : "Nothing playing"
    readonly property string artist: {
        if (!root.activePlayer) return ""
        if (root.activePlayer.trackArtists && root.activePlayer.trackArtists.length > 0) return root.activePlayer.trackArtists.join(", ")
        if (root.activePlayer.trackArtist) return root.activePlayer.trackArtist
        return root.activePlayer.identity || ""
    }
    readonly property string artUrl: root.activePlayer ? (root.activePlayer.trackArtUrl || "") : ""

    implicitHeight: root.hasPlayer ? 66 : 0
    visible: root.hasPlayer
    radius: Config.tileRadius
    color: Config.bgElevated
    border.width: 1
    border.color: Config.border

    Behavior on color { ColorAnimation { duration: 130 } }

    RowLayout {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        // Album Art and Metadata Clickable Area
        RowLayout {
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignVCenter
            spacing: 10

            // Album Art or Placeholder
            Rectangle {
                Layout.preferredWidth: 46
                Layout.preferredHeight: 46
                Layout.alignment: Qt.AlignVCenter
                radius: 8
                color: Config.accentSoft
                clip: true

                Image {
                    anchors.fill: parent
                    source: root.artUrl
                    fillMode: Image.PreserveAspectCrop
                    visible: root.artUrl !== ""
                }

                Text {
                    anchors.centerIn: parent
                    visible: root.artUrl === ""
                    text: Config.gMusic
                    font.family: Config.glyphFont
                    font.pixelSize: 18
                    color: Config.accentActive
                }
            }

            // Title and Artist
            Item {
                Layout.fillWidth: true
                Layout.alignment: Qt.AlignVCenter
                implicitHeight: trackCol.implicitHeight

                ColumnLayout {
                    id: trackCol
                    anchors.fill: parent
                    spacing: 2

                    Text {
                        Layout.fillWidth: true
                        text: root.title
                        font.family: Config.fontName
                        font.pixelSize: Config.textSize
                        font.bold: true
                        color: Config.fg
                        elide: Text.ElideRight
                    }

                    Text {
                        Layout.fillWidth: true
                        text: root.artist
                        font.family: Config.fontName
                        font.pixelSize: 9
                        color: Config.fgDim
                        elide: Text.ElideRight
                    }
                }

                MouseArea {
                    anchors.fill: parent
                    cursorShape: Qt.PointingHandCursor
                    onClicked: root.expandClicked()
                }
            }
        }

        // Playback Controls
        RowLayout {
            Layout.alignment: Qt.AlignVCenter
            spacing: 6

            // Prev
            Rectangle {
                Layout.preferredWidth: 28
                Layout.preferredHeight: 28
                radius: 7
                color: prevHover.hovered ? Config.bgHover : "transparent"

                Text {
                    anchors.centerIn: parent
                    text: Config.gPrev
                    font.family: Config.glyphFont
                    font.pixelSize: 11
                    color: root.activePlayer && root.activePlayer.canGoPrevious ? Config.fg : Config.fgDim
                }

                MouseArea {
                    id: prevHover
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    enabled: root.activePlayer && root.activePlayer.canGoPrevious
                    onClicked: if (root.activePlayer) root.activePlayer.previous()
                }
            }

            // Play / Pause
            Rectangle {
                Layout.preferredWidth: 32
                Layout.preferredHeight: 32
                radius: 16
                color: playHover.hovered ? Qt.darker(Config.accentActive, 1.15) : Config.accentActive

                Behavior on color { ColorAnimation { duration: 120 } }

                Text {
                    anchors.centerIn: parent
                    text: root.isPlaying ? Config.gPause : Config.gPlay
                    font.family: Config.glyphFont
                    font.pixelSize: 12
                    color: "#ffffff"
                }

                MouseArea {
                    id: playHover
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        if (root.activePlayer) {
                            if (typeof root.activePlayer.togglePlaying === "function") root.activePlayer.togglePlaying()
                            else if (typeof root.activePlayer.playPause === "function") root.activePlayer.playPause()
                        }
                    }
                }
            }

            // Next
            Rectangle {
                Layout.preferredWidth: 28
                Layout.preferredHeight: 28
                radius: 7
                color: nextHover.hovered ? Config.bgHover : "transparent"

                Text {
                    anchors.centerIn: parent
                    text: Config.gNext
                    font.family: Config.glyphFont
                    font.pixelSize: 11
                    color: root.activePlayer && root.activePlayer.canGoNext ? Config.fg : Config.fgDim
                }

                MouseArea {
                    id: nextHover
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    enabled: root.activePlayer && root.activePlayer.canGoNext
                    onClicked: if (root.activePlayer) root.activePlayer.next()
                }
            }
        }
    }
}
