import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Quickshell
import Quickshell.Io
import Quickshell.Networking
import Quickshell.Bluetooth
import Quickshell.Services.Pipewire
import "helpers.js" as H

Rectangle {
    id: root

    signal requestClose()

    // Manager instances injected by the shell, used to open the detailed popups.
    property var networksManager: null
    property var bluetoothManager: null
    property var volumeManager: null

    implicitWidth: Config.popupWidth
    implicitHeight: body.implicitHeight + 32
    width: Config.popupWidth
    radius: Config.radius
    color: Config.bg
    border.color: Config.border
    border.width: 1

    // ------------------------------------------------------------------- clock
    property string headerTime: ""

    Timer {
        interval: 1000
        running: true
        repeat: true
        onTriggered: {
            root.headerTime = Qt.formatTime(new Date(), "hh:mm")
            root.updateGreeting()
        }
        Component.onCompleted: {
            root.headerTime = Qt.formatTime(new Date(), "hh:mm")
            root.updateGreeting()
        }
    }

    // -------------------------------------------------------- identity
    property string userName: ""
    property string homeDir: ""
    property string activeThemeDir: ""
    property string greetingPart: "Good Morning"

    function updateGreeting() {
        var h = new Date().getHours()
        root.greetingPart =
            h >= 5 && h < 12 ? "Good Morning" :
            h >= 12 && h < 17 ? "Good Afternoon" :
            h >= 17 && h < 21 ? "Good Evening" : "Good Night"
    }

    // ----------------------------------------------------------- wifi state
    property var wifiDevice: null

    function refreshDevices() {
        var devices = Networking.devices.values
        var w = null
        for (var i = 0; i < devices.length; ++i) {
            if (!w && devices[i].type === DeviceType.Wifi) w = devices[i]
        }
        root.wifiDevice = w
    }

    Connections {
        target: Networking.devices
        function onObjectInsertedPost() { root.refreshDevices() }
        function onObjectRemovedPost() { root.refreshDevices() }
    }

    Component.onCompleted: {
        root.refreshDevices()
        userQuery.running = true
        homeQuery.running = true
        themeQuery.running = true
    }

    readonly property bool wifiOn: Networking.wifiEnabled

    readonly property string wifiName: {
        if (!root.wifiOn) return "Off"
        if (!root.wifiDevice) return "Not connected"
        var nets = root.wifiDevice.networks.values
        for (var i = 0; i < nets.length; ++i) {
            if (nets[i].connected) return nets[i].name
        }
        return "Not connected"
    }

    // --------------------------------------------------- airplane / radio state
    property bool wifiRadio: true
    property bool wwanRadio: true
    readonly property bool airplaneMode: !root.wifiRadio && !root.wwanRadio

    Process {
        id: radioStatus
        command: ["nmcli", "-t", "-f", "WIFI,WWAN", "radio"]
        stdout: SplitParser {
            onRead: function(data) {
                var parts = String(data).trim().split(":")
                if (parts.length < 2) return
                if (parts[0] === "enabled") root.wifiRadio = true
                else if (parts[0] === "disabled") root.wifiRadio = false
                if (parts[1] === "enabled") root.wwanRadio = true
                else if (parts[1] === "disabled") root.wwanRadio = false
            }
        }
    }

    Process {
        id: radioCmd
    }

    Timer {
        id: radioPoll
        interval: 2000
        running: true
        repeat: true
        onTriggered: root.pollRadio()
    }

    Timer {
        id: pollSoonTimer
        interval: 600
        onTriggered: root.pollRadio()
    }

    function pollRadio() {
        radioStatus.running = false
        radioStatus.running = true
    }

    function pollSoon() {
        pollSoonTimer.restart()
    }

    function setAirplane(on) {
        radioCmd.exec(["nmcli", "radio", "all", on ? "off" : "on"])
        var adapter = Bluetooth.defaultAdapter
        if (adapter) adapter.enabled = !on
        root.pollSoon()
    }

    function setWifi(on) {
        if (on && root.airplaneMode) root.setAirplane(false)
        Networking.wifiEnabled = on
        root.pollSoon()
    }

    // --------------------------------------------------- bluetooth state
    readonly property var btAdapter: Bluetooth.defaultAdapter

    readonly property bool btOn: root.btAdapter ? root.btAdapter.enabled : false

    readonly property string btName: {
        if (!root.btOn || !root.btAdapter) return "Off"
        var devs = root.btAdapter.devices.values
        for (var i = 0; i < devs.length; ++i) {
            if (devs[i].connected) return devs[i].name
        }
        return "On"
    }

    function toggleBluetooth() {
        var adapter = root.btAdapter
        if (!adapter) return
        var target = !adapter.enabled
        if (target && root.airplaneMode) root.setAirplane(false)
        adapter.enabled = target
        root.pollSoon()
    }

    // ------------------------------------------------------------- audio state
    readonly property var mainSink: Pipewire.defaultAudioSink || null
    readonly property var mainMic: Pipewire.defaultAudioSource || null

    readonly property var trackedNodes: {
        var out = []
        if (root.mainSink) out.push(root.mainSink)
        if (root.mainMic) out.push(root.mainMic)
        return out
    }

    PwObjectTracker {
        objects: root.trackedNodes
    }

    readonly property bool sinkReady: root.mainSink && root.mainSink.audio ? true : false
    readonly property bool micReady: root.mainMic && root.mainMic.audio ? true : false

    readonly property real volume: root.sinkReady ? root.mainSink.audio.volume : 0
    readonly property bool sinkMuted: root.sinkReady ? root.mainSink.audio.muted : false
    readonly property bool micMuted: root.micReady ? root.mainMic.audio.muted : false

    readonly property string volumeGlyph: {
        if (!root.sinkReady || root.sinkMuted || root.volume <= 0.02) return Config.gVolumeOff
        if (root.volume < 0.5) return Config.gVolumeLow
        return Config.gVolumeHigh
    }

    function setVolume(v) {
        if (root.mainSink && root.mainSink.audio) {
            root.mainSink.audio.volume = H.clamp(v, 0, Config.maxVolume)
        }
    }

    function toggleSinkMute() {
        if (root.mainSink && root.mainSink.audio) {
            root.mainSink.audio.muted = !root.mainSink.audio.muted
        }
    }

    function toggleMicMute() {
        if (root.mainMic && root.mainMic.audio) {
            root.mainMic.audio.muted = !root.mainMic.audio.muted
        }
    }

    // -------------------------------------------------------- brightness state
    property bool hasBrightness: false
    property int brightnessPct: 0

    Process {
        id: brightnessStatus
        command: ["brightnessctl", "-m"]
        stdout: SplitParser {
            onRead: function(data) { root.applyBrightness(data) }
        }
    }

    Process {
        id: brightnessSet
    }

    Timer {
        id: brightPoll
        interval: 1500
        running: true
        repeat: true
        onTriggered: root.pollBrightness()
    }

    function applyBrightness(data) {
        var line = String(data).trim()
        if (line === "") return
        var p = line.split(",")
        if (p.length < 4) return
        // Skip keyboard/other class devices unless we have no backlight yet.
        if (p[1] !== "backlight" && root.hasBrightness) return
        var pct = parseInt(p[3], 10)
        if (isNaN(pct)) return
        root.brightnessPct = H.clamp(pct, 0, 100)
        root.hasBrightness = true
    }

    function pollBrightness() {
        brightnessStatus.running = false
        brightnessStatus.running = true
    }

    function setBrightness(pct) {
        var v = H.clamp(Math.round(pct), 0, 100)
        root.brightnessPct = v
        brightnessSet.exec(["brightnessctl", "set", v + "%"])
    }

    // --------------------------------------------------- identity probes
    Process {
        id: userQuery
        command: ["whoami"]
        stdout: SplitParser {
            onRead: function(data) {
                var name = String(data).trim()
                if (name !== "") root.userName = name
            }
        }
    }

    Process {
        id: homeQuery
        command: ["sh", "-c", "echo $HOME"]
        stdout: SplitParser {
            onRead: function(data) {
                var dir = String(data).trim()
                if (dir !== "") root.homeDir = dir
            }
        }
    }

    Process {
        id: themeQuery
        command: ["sh", "-c", "cat \"$HOME/.local/share/Aurora/theme_name.log\""]
        stdout: SplitParser {
            onRead: function(data) {
                var dir = String(data).trim()
                if (dir !== "") root.activeThemeDir = dir
            }
        }
    }

    Process {
        id: appSpawn
    }

    Process {
        id: themeApply
    }

    Timer {
        id: themeRefreshTimer
        interval: 3500
        onTriggered: {
            themeQuery.running = false
            themeQuery.running = true
        }
    }

    function applyTheme(dir) {
        if (!dir) return
        root.activeThemeDir = dir
        themeApply.exec(["aurora", "apply-theme", dir])
        themeRefreshTimer.restart()
    }

    // ------------------------------------------------------------- detail nav
    function openDetail(which) {
        var target = null
        if (which === "wifi") target = root.networksManager
        else if (which === "bluetooth") target = root.bluetoothManager
        else if (which === "volume") target = root.volumeManager
        if (!target) return
        root.requestClose()
        target.openPopup()
    }

    // ------------------------------------------------------------------- UI
    ColumnLayout {
        id: body
        anchors.fill: parent
        anchors.margins: 16
        spacing: 10

        // header ------------------------------------------------------------
        RowLayout {
            Layout.fillWidth: true
            spacing: 10

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 0

                Text {
                    text: root.greetingPart + (root.userName !== "" ? ", " + root.userName : "")
                    font.family: Config.fontName
                    font.pixelSize: Config.titleSize
                    font.bold: true
                    color: Config.fg
                }

                Text {
                    text: root.headerTime && root.headerTime.length
                        ? Qt.formatDate(new Date(), "ddd, MMM d") + "  \u00b7  " + root.headerTime
                        : Qt.formatDate(new Date(), "ddd, MMM d")
                    font.family: Config.fontName
                    font.pixelSize: 9
                    color: Config.fgDim
                }
            }

            Rectangle {
                Layout.preferredWidth: 28
                Layout.preferredHeight: 28
                radius: 8
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

        // quick toggles -----------------------------------------------------
        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            QuickToggle {
                id: wifiTile
                Layout.fillWidth: true
                Layout.preferredHeight: Config.tileHeight
                checked: root.wifiOn
                icon: Config.gWifi
                label: "Wi-Fi"
                sub: root.wifiName
                expandable: true
                onClicked: root.setWifi(!root.wifiOn)
                onExpandClicked: root.openDetail("wifi")
            }

            QuickToggle {
                id: btTile
                Layout.fillWidth: true
                Layout.preferredHeight: Config.tileHeight
                checked: root.btOn
                icon: Config.gBluetooth
                label: "Bluetooth"
                sub: root.btName
                expandable: true
                onClicked: root.toggleBluetooth()
                onExpandClicked: root.openDetail("bluetooth")
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            QuickToggle {
                id: planeTile
                Layout.fillWidth: true
                Layout.preferredHeight: Config.tileHeight
                checked: root.airplaneMode
                icon: Config.gPlane
                label: "Airplane"
                sub: root.airplaneMode ? "On" : "Off"
                onClicked: root.setAirplane(!root.airplaneMode)
            }

            QuickToggle {
                id: micTile
                Layout.fillWidth: true
                Layout.preferredHeight: Config.tileHeight
                checked: root.micReady ? !root.micMuted : false
                icon: root.micMuted ? Config.gMicMute : Config.gMic
                label: "Microphone"
                sub: !root.micReady ? "No device" : (root.micMuted ? "Muted" : "On")
                onClicked: root.toggleMicMute()
            }
        }

        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: Config.border
            opacity: 0.6
            visible: root.hasBrightness || root.sinkReady
        }

        // sliders -----------------------------------------------------------
        SliderRow {
            Layout.fillWidth: true
            Layout.preferredHeight: Config.sliderHeight
            visible: root.hasBrightness
            glyph: Config.gBrightness
            iconColor: root.brightnessPct > 0 ? Config.warn : Config.fgDim
            value: root.brightnessPct / 100
            from: 0
            to: 1
            valueText: root.brightnessPct + "%"
            onUserSet: function(v) { root.setBrightness(v * 100) }
            onIconClicked: root.setBrightness(root.brightnessPct === 0 ? 100 : 0)
        }

        SliderRow {
            Layout.fillWidth: true
            Layout.preferredHeight: Config.sliderHeight
            glyph: root.volumeGlyph
            iconColor: root.sinkMuted ? Config.fgDim : (root.volume > 1.001 ? Config.warn : Config.accentActive)
            value: root.volume
            from: 0
            to: Config.maxVolume
            valueText: H.pctText(root.volume)
            onUserSet: function(v) { root.setVolume(v) }
            onIconClicked: root.toggleSinkMute()
        }

        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: Config.border
            opacity: 0.6
        }

        // quick actions ------------------------------------------------------
        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            ActionTile {
                Layout.fillWidth: true
                Layout.preferredHeight: Config.tileHeight
                glyph: Config.gSettings
                onClicked: appSpawn.exec(["settings"])
            }

            ActionTile {
                Layout.fillWidth: true
                Layout.preferredHeight: Config.tileHeight
                glyph: Config.gPower
                onClicked: appSpawn.exec(["system_menu"])
            }

            ActionTile {
                Layout.fillWidth: true
                Layout.preferredHeight: Config.tileHeight
                glyph: Config.gRefresh
                onClicked: appSpawn.exec(["sh", "-c", "aurora refresh && hyprctl reload"])
            }

            ActionTile {
                Layout.fillWidth: true
                Layout.preferredHeight: Config.tileHeight
                glyph: Config.gTerminal
                onClicked: appSpawn.exec(["command_palette"])
            }

            ActionTile {
                Layout.fillWidth: true
                Layout.preferredHeight: Config.tileHeight
                glyph: Config.gVideo
                onClicked: appSpawn.exec(["screenrecorder"])
            }
        }

        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: Config.border
            opacity: 0.6
        }

        // theme switcher -----------------------------------------------------
        ThemeSection {
            Layout.fillWidth: true
            homeDir: root.homeDir
            activeThemeDir: root.activeThemeDir
            onApplyTheme: function(dir) { root.applyTheme(dir) }
        }
    }
}