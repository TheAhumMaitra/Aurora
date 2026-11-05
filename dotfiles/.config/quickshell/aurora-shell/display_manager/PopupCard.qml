import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Quickshell
import Quickshell.Io
import "helpers.js" as H
import "."

Rectangle {
    id: root

    signal requestClose()

    property bool active: false

    implicitWidth: Config.popupWidth
    implicitHeight: body.implicitHeight + 32
    width: Config.popupWidth
    radius: Config.radius
    color: Config.bg
    border.color: Config.border
    border.width: 1

    // -------------------------------------------------------- monitor detection
    property var monitorList: []
    readonly property int monitorsCount: root.monitorList.length

    Process {
        id: monitorsProc
        command: ["sh", "-c", "hyprctl monitors all -j | jq -c ."]
        stdout: SplitParser {
            onRead: function(data) {
                try {
                    var parsed = JSON.parse(String(data).trim())
                    if (Array.isArray(parsed)) root.monitorList = parsed
                } catch(e) {}
            }
        }
    }

    Timer {
        id: monitorPoll
        interval: 3000
        running: root.active
        repeat: true
        onTriggered: root.rescanMonitors()
    }

    function rescanMonitors() {
        monitorsProc.running = false
        monitorsProc.running = true
    }

    // -------------------------------------------------------- brightness state
    property bool hasBrightness: false
    property int brightnessPct: 90

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
        interval: 2000
        running: root.active
        repeat: true
        onTriggered: root.pollBrightness()
    }

    function applyBrightness(data) {
        var line = String(data).trim()
        if (line === "") return
        var p = line.split(",")
        if (p.length < 4) return
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

    // --------------------------------------------------- night light / warmth
    property bool nightLightOn: false
    property int warmthPct: 50 // 0% = 6500K (Daylight), 100% = 2500K (Amber)

    Process {
        id: nightStatusProc
        command: ["sh", "-c", "pgrep -x wlsunset >/dev/null && echo true || echo false"]
        stdout: SplitParser {
            onRead: function(data) {
                root.nightLightOn = String(data).trim() === "true"
            }
        }
    }

    Process {
        id: warmthReadProc
        command: ["sh", "-c", "cat \"$HOME/.local/share/Aurora/warmth.txt\" 2>/dev/null || echo 50"]
        stdout: SplitParser {
            onRead: function(data) {
                var v = parseInt(String(data).trim(), 10)
                if (!isNaN(v)) root.warmthPct = H.clamp(v, 0, 100)
            }
        }
    }

    Process {
        id: appSpawn
    }

    function pollNightLight() {
        nightStatusProc.running = false
        nightStatusProc.running = true
    }

    function applyWarmth(pct) {
        root.warmthPct = H.clamp(Math.round(pct), 0, 100)
        var kelvin = H.warmthToKelvin(root.warmthPct)
        appSpawn.exec(["sh", "-c", "mkdir -p \"$HOME/.local/share/Aurora\" && echo " + root.warmthPct + " > \"$HOME/.local/share/Aurora/warmth.txt\""])

        if (root.warmthPct <= 0) {
            root.nightLightOn = false
            appSpawn.exec(["pkill", "-x", "wlsunset"])
        } else {
            root.nightLightOn = true
            appSpawn.exec(["sh", "-c", "pkill -x wlsunset; wlsunset -t " + kelvin + " >/dev/null 2>&1 &"])
        }
        pollNightSoonTimer.restart()
    }

    function toggleNightLight() {
        if (root.nightLightOn) {
            root.nightLightOn = false
            appSpawn.exec(["pkill", "-x", "wlsunset"])
        } else {
            var p = root.warmthPct > 0 ? root.warmthPct : 50
            root.applyWarmth(p)
        }
        pollNightSoonTimer.restart()
    }

    Timer {
        id: pollNightSoonTimer
        interval: 400
        onTriggered: root.pollNightLight()
    }

    // --------------------------------------------------- vrr / adaptive sync
    property bool vrrOn: false

    Process {
        id: vrrStatusProc
        command: ["sh", "-c", "hyprctl getoption misc:vrr -j | jq -r '.int'"]
        stdout: SplitParser {
            onRead: function(data) {
                var v = parseInt(String(data).trim(), 10)
                root.vrrOn = (!isNaN(v) && v > 0)
            }
        }
    }

    function pollVrr() {
        vrrStatusProc.running = false
        vrrStatusProc.running = true
    }

    function toggleVrr() {
        var target = root.vrrOn ? 0 : 1
        root.vrrOn = (target === 1)
        appSpawn.exec(["sh", "-c", "hyprctl eval \"hl.config({ misc = { vrr = " + target + " } })\""])
        pollVrrSoonTimer.restart()
    }

    Timer {
        id: pollVrrSoonTimer
        interval: 400
        onTriggered: {
            root.pollVrr()
            root.rescanMonitors()
        }
    }

    // --------------------------------------------------- display sleep timeout
    property string activePreset: "5m"

    Process {
        id: presetReadProc
        command: ["sh", "-c", "cat \"$HOME/.local/share/Aurora/sleep_preset.txt\" 2>/dev/null || echo 5m"]
        stdout: SplitParser {
            onRead: function(data) {
                var s = String(data).trim()
                if (s.length > 0) root.activePreset = s
            }
        }
    }

    function setSleepPreset(preset) {
        root.activePreset = preset
        appSpawn.exec(["sh", "-c", "mkdir -p \"$HOME/.local/share/Aurora\" && echo " + preset + " > \"$HOME/.local/share/Aurora/sleep_preset.txt\""])

        if (preset === "Never") {
            appSpawn.exec(["pkill", "-STOP", "hypridle"])
        } else {
            appSpawn.exec(["pkill", "-CONT", "hypridle"])
        }
    }

    function sleepDisplaysNow() {
        appSpawn.exec(["hyprctl", "dispatch", "dpms", "off"])
    }

    function toggleMonitorPower(name) {
        if (!name) return
        appSpawn.exec(["hyprctl", "dispatch", "dpms", "toggle", name])
        monitorPollTimerSoon.restart()
    }

    Timer {
        id: monitorPollTimerSoon
        interval: 500
        onTriggered: root.rescanMonitors()
    }

    // ------------------------------------------------------------- lifecycle
    Component.onCompleted: {
        root.rescanMonitors()
        root.pollBrightness()
        root.pollNightLight()
        root.pollVrr()
        warmthReadProc.running = true
        presetReadProc.running = true
    }

    onActiveChanged: {
        if (root.active) {
            root.rescanMonitors()
            root.pollBrightness()
            root.pollNightLight()
            root.pollVrr()
        }
    }

    // ------------------------------------------------------------------- UI
    ColumnLayout {
        id: body
        anchors.fill: parent
        anchors.margins: 16
        spacing: 10

        // Header ------------------------------------------------------------
        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Rectangle {
                Layout.preferredWidth: 32
                Layout.preferredHeight: 32
                radius: 8
                color: Config.accentSoft

                Text {
                    anchors.centerIn: parent
                    text: Config.gMonitor
                    font.family: Config.glyphFont
                    font.pixelSize: 15
                    color: Config.accentActive
                }
            }

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 0

                Text {
                    text: "Display & Visual Comfort"
                    font.family: Config.fontName
                    font.pixelSize: Config.titleSize
                    font.bold: true
                    color: Config.fg
                }

                Text {
                    text: root.monitorsCount + " connected display" + (root.monitorsCount > 1 ? "s" : "")
                    font.family: Config.fontName
                    font.pixelSize: Config.subSize
                    color: Config.fgDim
                }
            }

            // Rescan Button
            Rectangle {
                Layout.preferredWidth: 28
                Layout.preferredHeight: 28
                radius: 7
                color: refHover.hovered ? Config.bgHover : "transparent"

                Text {
                    anchors.centerIn: parent
                    text: Config.gRefresh
                    font.family: Config.glyphFont
                    font.pixelSize: 12
                    color: Config.fgDim
                }

                MouseArea {
                    id: refHover
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        root.rescanMonitors()
                        root.pollBrightness()
                        root.pollNightLight()
                        root.pollVrr()
                    }
                }
            }

            // Close Button
            Rectangle {
                Layout.preferredWidth: 28
                Layout.preferredHeight: 28
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

        // Section: Visual Comfort (Brightness & Warmth Sliders) --------------
        Text {
            text: "VISUAL COMFORT"
            font.family: Config.fontName
            font.pixelSize: 9
            font.bold: true
            color: Config.fgDim
        }

        DisplaySlider {
            Layout.fillWidth: true
            glyph: Config.gBrightness
            iconColor: root.brightnessPct > 0 ? Config.warn : Config.fgDim
            iconBg: Qt.alpha(Config.warn, 0.16)
            value: root.brightnessPct / 100
            from: 0
            to: 1
            fillColor: Config.warn
            valueText: root.brightnessPct + "%"
            onUserSet: function(v) { root.setBrightness(v * 100) }
            onIconClicked: root.setBrightness(root.brightnessPct === 0 ? 100 : 0)
        }

        DisplaySlider {
            Layout.fillWidth: true
            glyph: Config.gMoon
            iconColor: root.nightLightOn ? Config.warmAmber : Config.coolBlue
            iconBg: Qt.alpha(root.nightLightOn ? Config.warmAmber : Config.coolBlue, 0.16)
            value: root.nightLightOn ? (root.warmthPct / 100) : 0
            from: 0
            to: 1
            fillColor: Config.warmAmber
            valueText: H.warmthLabel(root.nightLightOn ? root.warmthPct : 0)
            onUserSet: function(v) { root.applyWarmth(v * 100) }
            onIconClicked: root.toggleNightLight()
        }

        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: Config.border
            opacity: 0.6
        }

        // Section: Attached Monitors -----------------------------------------
        Text {
            text: "ATTACHED MONITORS"
            font.family: Config.fontName
            font.pixelSize: 9
            font.bold: true
            color: Config.fgDim
        }

        ColumnLayout {
            Layout.fillWidth: true
            spacing: 8

            Repeater {
                model: root.monitorList

                MonitorCard {
                    Layout.fillWidth: true
                    monitor: modelData
                    onTogglePower: root.toggleMonitorPower(modelData.name)
                }
            }

            // Fallback when no monitors detected yet
            Rectangle {
                visible: root.monitorList.length === 0
                Layout.fillWidth: true
                Layout.preferredHeight: 52
                radius: Config.tileRadius
                color: Config.bgElevated
                border.width: 1
                border.color: Config.border

                Text {
                    anchors.centerIn: parent
                    text: "Scanning monitors..."
                    font.family: Config.fontName
                    font.pixelSize: 10
                    color: Config.fgDim
                }
            }
        }

        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: Config.border
            opacity: 0.6
        }

        // Section: Toggles & Actions -----------------------------------------
        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            // VRR Toggle Tile
            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 46
                radius: Config.tileRadius
                color: vrrHover.hovered
                    ? (root.vrrOn ? Qt.darker(Config.accentActive, 1.15) : Config.bgHover)
                    : (root.vrrOn ? Config.accentActive : Config.bgElevated)
                border.width: 1
                border.color: root.vrrOn
                    ? Qt.alpha("#ffffff", 0.3)
                    : (vrrHover.hovered ? Qt.alpha(Config.accent, 0.6) : Config.border)

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 8
                    spacing: 8

                    Rectangle {
                        Layout.preferredWidth: 28
                        Layout.preferredHeight: 28
                        radius: 7
                        color: root.vrrOn ? Qt.alpha("#ffffff", 0.22) : Config.accentSoft

                        Text {
                            anchors.centerIn: parent
                            text: Config.gSync
                            font.family: Config.glyphFont
                            font.pixelSize: 12
                            color: root.vrrOn ? "#ffffff" : Config.accentActive
                        }
                    }

                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 0

                        Text {
                            text: "Adaptive Sync"
                            font.family: Config.fontName
                            font.pixelSize: 10
                            font.bold: true
                            color: root.vrrOn ? "#ffffff" : Config.fg
                        }

                        Text {
                            text: root.vrrOn ? "VRR Enabled" : "Off"
                            font.family: Config.fontName
                            font.pixelSize: 8
                            color: root.vrrOn ? Qt.alpha("#ffffff", 0.75) : Config.fgDim
                        }
                    }
                }

                MouseArea {
                    id: vrrHover
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: root.toggleVrr()
                }
            }

            // Sleep Displays Now
            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 46
                radius: Config.tileRadius
                color: sleepHover.hovered ? Config.bgHover : Config.bgElevated
                border.width: 1
                border.color: sleepHover.hovered ? Qt.alpha(Config.danger, 0.6) : Config.border

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 8
                    spacing: 8

                    Rectangle {
                        Layout.preferredWidth: 28
                        Layout.preferredHeight: 28
                        radius: 7
                        color: Qt.alpha(Config.danger, 0.16)

                        Text {
                            anchors.centerIn: parent
                            text: Config.gPower
                            font.family: Config.glyphFont
                            font.pixelSize: 12
                            color: Config.danger
                        }
                    }

                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 0

                        Text {
                            text: "Sleep Displays"
                            font.family: Config.fontName
                            font.pixelSize: 10
                            font.bold: true
                            color: Config.fg
                        }

                        Text {
                            text: "Turn off screens"
                            font.family: Config.fontName
                            font.pixelSize: 8
                            color: Config.fgDim
                        }
                    }
                }

                MouseArea {
                    id: sleepHover
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: root.sleepDisplaysNow()
                }
            }
        }

        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: Config.border
            opacity: 0.6
        }

        // Section: Display Sleep Timeout Presets -----------------------------
        RowLayout {
            Layout.fillWidth: true

            Text {
                text: "SLEEP TIMEOUT PRESETS"
                font.family: Config.fontName
                font.pixelSize: 9
                font.bold: true
                color: Config.fgDim
            }

            Item { Layout.fillWidth: true }

            Text {
                text: root.activePreset === "Never" ? "Always Awake" : (root.activePreset + " timeout")
                font.family: Config.fontName
                font.pixelSize: 9
                color: Config.accentActive
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 6

            PresetButton {
                Layout.fillWidth: true
                label: "Never"
                checked: root.activePreset === "Never"
                onClicked: root.setSleepPreset("Never")
            }

            PresetButton {
                Layout.fillWidth: true
                label: "2m"
                checked: root.activePreset === "2m"
                onClicked: root.setSleepPreset("2m")
            }

            PresetButton {
                Layout.fillWidth: true
                label: "5m"
                checked: root.activePreset === "5m"
                onClicked: root.setSleepPreset("5m")
            }

            PresetButton {
                Layout.fillWidth: true
                label: "10m"
                checked: root.activePreset === "10m"
                onClicked: root.setSleepPreset("10m")
            }

            PresetButton {
                Layout.fillWidth: true
                label: "15m"
                checked: root.activePreset === "15m"
                onClicked: root.setSleepPreset("15m")
            }

            PresetButton {
                Layout.fillWidth: true
                label: "30m"
                checked: root.activePreset === "30m"
                onClicked: root.setSleepPreset("30m")
            }
        }
    }
}
