import QtQuick
import QtQuick.Effects
import "."

Item {
    id: root

    property string artUrl: ""
    property bool isPlaying: false
    property real size: Config.artSize

    width: root.size + (root.isPlaying ? 22 : 0)
    height: root.size
    implicitWidth: width
    implicitHeight: height

    Behavior on width { NumberAnimation { duration: 300; easing.type: Easing.OutCubic } }

    // Vinyl Record disc that slides out from behind the album art
    Rectangle {
        id: vinyl
        width: root.size - 6
        height: root.size - 6
        radius: width / 2
        anchors.verticalCenter: parent.verticalCenter
        x: root.isPlaying ? 22 : 4
        color: "#141419"
        border.color: "#2e2e38"
        border.width: 1
        z: 1

        Behavior on x { NumberAnimation { duration: 350; easing.type: Easing.OutCubic } }

        // Concentric vinyl grooves
        Rectangle {
            anchors.centerIn: parent
            width: parent.width * 0.78
            height: width
            radius: width / 2
            color: "transparent"
            border.color: Qt.alpha("#ffffff", 0.07)
            border.width: 1
        }
        Rectangle {
            anchors.centerIn: parent
            width: parent.width * 0.58
            height: width
            radius: width / 2
            color: "transparent"
            border.color: Qt.alpha("#ffffff", 0.09)
            border.width: 1
        }

        // Center vinyl label
        Rectangle {
            id: vinylCenter
            anchors.centerIn: parent
            width: parent.width * 0.36
            height: width
            radius: width / 2
            color: Config.accent

            Rectangle {
                anchors.centerIn: parent
                width: 8
                height: 8
                radius: 4
                color: "#141419"
            }
        }

        // Slow rotation while playing
        RotationAnimation on rotation {
            running: root.isPlaying
            from: 0
            to: 360
            duration: 9000
            loops: Animation.Infinite
        }
    }

    // Main Album Sleeve / Cover
    Rectangle {
        id: sleeve
        width: root.size
        height: root.size
        radius: Config.radiusSm + 2
        color: Config.bgElevated
        border.color: Qt.alpha(Config.border, 0.7)
        border.width: 1
        clip: true
        z: 2

        Image {
            id: coverImg
            anchors.fill: parent
            source: root.artUrl
            fillMode: Image.PreserveAspectCrop
            asynchronous: true
            visible: root.artUrl !== "" && status === Image.Ready
        }

        // Fallback when no cover is available or while loading
        Rectangle {
            anchors.fill: parent
            visible: root.artUrl === "" || coverImg.status !== Image.Ready
            color: Config.accentSoft

            Text {
                anchors.centerIn: parent
                text: Config.gMusic
                font.family: Config.glyphFont
                font.pixelSize: 32
                color: Config.accentActive
            }
        }
    }
}
