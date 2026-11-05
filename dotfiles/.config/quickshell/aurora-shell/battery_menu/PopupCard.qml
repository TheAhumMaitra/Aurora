import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Quickshell
import Quickshell.Io
import "."
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

    property bool active: false

    // ------------------------------------------------------------- state model
    property bool batPresent: true
    property string batName: "BAT0"
    property int percentage: 0
    property string status: "Unknown"
    property real powerRate: 0.0
    property real powerSigned: 0.0
    property real voltage: 0.0
    property real voltageMinDesign: 0.0
    property real energyNow: 0.0
    property real energyFull: 0.0
    property real energyDesign: 0.0
    property real healthPct: 100.0
    property int cycles: 0
    property string modelName: ""
    property string vendor: ""
    property string technology: ""
    property string serialNumber: ""
    property bool acOnline: false
    property string activeProfile: "balanced"
    property string timeEstimate: ""
    property var peripherals: []
    property string lastUpdated: ""

    readonly property bool isCharging: root.status.toLowerCase() === "charging"
    readonly property color dynamicColor: H.levelColor(root.percentage, root.isCharging, root.acOnline)

    // ----------------------------------------------------------- battery probe
    Process {
        id: batQuery
        command: [
            "python3",
            Quickshell.shellPath("battery_menu/battery_info.py")
        ]
        stdout: SplitParser {
            onRead: function(data) {
                root.parseBatteryData(data)
            }
        }
    }

    Process {
        id: profileSetProc
    }

    Timer {
        id: pollTimer
        interval: 2000
        repeat: true
        running: root.active
        onTriggered: root.refresh()
    }

    onActiveChanged: {
        if (root.active) {
            root.refresh()
        }
    }

    Component.onCompleted: {
        root.refresh()
    }

    function refresh() {
        batQuery.running = false
        batQuery.running = true
    }

    function setPowerProfile(prof) {
        root.activeProfile = prof
        profileSetProc.exec(["powerprofilesctl", "set", prof])
        pollTimer.restart()
    }

    function parseBatteryData(raw) {
        var str = String(raw).trim()
        if (!str || str.length === 0) return
        try {
            var data = JSON.parse(str)
            root.batPresent = data.present !== undefined ? data.present : true
            root.batName = data.name || "BAT0"
            root.percentage = data.percentage || 0
            root.status = data.status || "Unknown"
            root.powerRate = data.power_w || 0.0
            root.powerSigned = data.power_signed_w || 0.0
            root.voltage = data.voltage_v || 0.0
            root.voltageMinDesign = data.voltage_min_design_v || 0.0
            root.energyNow = data.energy_now_wh || 0.0
            root.energyFull = data.energy_full_wh || 0.0
            root.energyDesign = data.energy_full_design_wh || 0.0
            root.healthPct = data.health_pct || 100.0
            root.cycles = data.cycles || 0
            root.modelName = data.model || ""
            root.vendor = data.vendor || ""
            root.technology = data.technology || ""
            root.serialNumber = data.serial || ""
            root.acOnline = !!data.ac_online
            root.activeProfile = data.profile || "balanced"
            root.timeEstimate = data.time_str || ""
            root.peripherals = data.peripherals || []
            root.lastUpdated = Qt.formatTime(new Date(), "hh:mm:ss")
        } catch (e) {
            console.warn("Battery data parse error:", e)
        }
    }

    // ---------------------------------------------------------------------- UI
    ColumnLayout {
        id: body
        anchors.fill: parent
        anchors.margins: 16
        spacing: 12

        // Header
        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Rectangle {
                Layout.preferredWidth: 26
                Layout.preferredHeight: 26
                radius: 7
                color: Qt.alpha(root.dynamicColor, 0.16)

                Text {
                    anchors.centerIn: parent
                    text: root.isCharging ? Config.gBolt : Config.gBatteryFull
                    font.family: Config.glyphFont
                    font.pixelSize: 13
                    color: root.dynamicColor
                }
            }

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 1

                Text {
                    text: "Battery & Power"
                    font.family: Config.fontName
                    font.pixelSize: Config.titleSize
                    font.bold: true
                    color: Config.fg
                }

                Text {
                    text: (root.modelName ? root.modelName + " (" + root.batName + ")" : root.batName)
                          + (root.acOnline ? " \u00b7 AC Connected" : " \u00b7 On Battery")
                    font.family: Config.fontName
                    font.pixelSize: 10
                    color: Config.fgDim
                    elide: Text.ElideRight
                }
            }

            // Refresh Button
            Rectangle {
                Layout.preferredWidth: 26
                Layout.preferredHeight: 26
                radius: 7
                color: refreshHover.hovered ? Config.bgHover : "transparent"

                Text {
                    anchors.centerIn: parent
                    text: Config.gRefresh
                    font.family: Config.glyphFont
                    font.pixelSize: 11
                    color: Config.fgDim
                }

                MouseArea {
                    id: refreshHover
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: root.refresh()
                }
            }

            // Close Button
            Rectangle {
                Layout.preferredWidth: 26
                Layout.preferredHeight: 26
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

        // Divider
        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: Config.border
            opacity: 0.5
        }

        // Hero Battery Status Section
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 88
            radius: Config.radiusSm
            color: Config.bgElevated
            border.color: Config.border
            border.width: 1

            RowLayout {
                anchors.fill: parent
                anchors.margins: 12
                spacing: 14

                // Gauge
                BatteryGauge {
                    Layout.preferredWidth: 120
                    Layout.preferredHeight: 52
                    Layout.alignment: Qt.AlignVCenter
                    percentage: root.percentage
                    charging: root.isCharging
                    acOnline: root.acOnline
                }

                // Status info column
                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignVCenter
                    spacing: 3

                    RowLayout {
                        spacing: 8

                        Text {
                            text: root.percentage + "%"
                            font.family: Config.fontName
                            font.pixelSize: 26
                            font.bold: true
                            color: root.dynamicColor
                        }

                        // Rate / Wattage Pill
                        Rectangle {
                            Layout.preferredHeight: 22
                            Layout.preferredWidth: rateText.implicitWidth + 14
                            radius: 6
                            color: root.isCharging
                                ? Qt.alpha(Config.ok, 0.18)
                                : (root.powerRate > 0 ? Qt.alpha(Config.accentActive, 0.18) : Qt.alpha(Config.fgDim, 0.15))
                            border.color: root.isCharging
                                ? Qt.alpha(Config.ok, 0.4)
                                : (root.powerRate > 0 ? Qt.alpha(Config.accentActive, 0.4) : "transparent")
                            border.width: 1

                            RowLayout {
                                anchors.centerIn: parent
                                spacing: 4

                                Text {
                                    visible: root.isCharging
                                    text: Config.gBolt
                                    font.family: Config.glyphFont
                                    font.pixelSize: 9
                                    color: Config.ok
                                }

                                Text {
                                    id: rateText
                                    text: (root.isCharging ? "+" : (root.powerRate > 0 ? "-" : ""))
                                          + root.powerRate.toFixed(1) + " W"
                                    font.family: Config.fontName
                                    font.pixelSize: 10
                                    font.bold: true
                                    color: root.isCharging ? Config.ok : Config.accentActive
                                }
                            }
                        }
                    }

                    // Status and time subtitle
                    Text {
                        text: (root.status !== "" ? root.status : "Idle")
                              + (root.timeEstimate.length > 0 ? "  \u00b7  " + root.timeEstimate : "")
                        font.family: Config.fontName
                        font.pixelSize: 10
                        color: Config.fgSub
                        elide: Text.ElideRight
                        Layout.fillWidth: true
                    }
                }
            }
        }

        // Power Profiles Section
        ColumnLayout {
            Layout.fillWidth: true
            spacing: 6

            RowLayout {
                Layout.fillWidth: true

                Text {
                    text: "Performance Profile"
                    font.family: Config.fontName
                    font.pixelSize: 10
                    font.bold: true
                    color: Config.fgSub
                }

                Item { Layout.fillWidth: true }

                Text {
                    text: root.activeProfile.charAt(0).toUpperCase() + root.activeProfile.slice(1).replace("-", " ")
                    font.family: Config.fontName
                    font.pixelSize: 10
                    color: Config.accentActive
                    font.bold: true
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 6

                ProfileButton {
                    profileId: "power-saver"
                    label: "Power Saver"
                    icon: Config.gLeaf
                    active: root.activeProfile === "power-saver"
                    onSelected: function(id) { root.setPowerProfile(id) }
                }

                ProfileButton {
                    profileId: "balanced"
                    label: "Balanced"
                    icon: Config.gSun
                    active: root.activeProfile === "balanced"
                    onSelected: function(id) { root.setPowerProfile(id) }
                }

                ProfileButton {
                    profileId: "performance"
                    label: "Performance"
                    icon: Config.gGauge
                    active: root.activeProfile === "performance"
                    onSelected: function(id) { root.setPowerProfile(id) }
                }
            }
        }

        // Battery Health Card
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 74
            radius: Config.radiusSm
            color: Config.bgElevated
            border.color: Config.border
            border.width: 1

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 10
                spacing: 6

                RowLayout {
                    Layout.fillWidth: true

                    Text {
                        text: Config.gHeart
                        font.family: Config.glyphFont
                        font.pixelSize: 11
                        color: H.healthColor(root.healthPct)
                    }

                    Text {
                        text: "Battery Health"
                        font.family: Config.fontName
                        font.pixelSize: 11
                        font.bold: true
                        color: Config.fg
                    }

                    Item { Layout.fillWidth: true }

                    // Rating Pill
                    Rectangle {
                        Layout.preferredHeight: 18
                        Layout.preferredWidth: healthText.implicitWidth + 12
                        radius: 5
                        color: Qt.alpha(H.healthColor(root.healthPct), 0.16)
                        border.color: Qt.alpha(H.healthColor(root.healthPct), 0.4)
                        border.width: 1

                        Text {
                            id: healthText
                            anchors.centerIn: parent
                            text: root.healthPct.toFixed(1) + "% \u00b7 " + H.healthRating(root.healthPct)
                            font.family: Config.fontName
                            font.pixelSize: 9
                            font.bold: true
                            color: H.healthColor(root.healthPct)
                        }
                    }
                }

                // Health Bar
                Rectangle {
                    Layout.fillWidth: true
                    height: 6
                    radius: 3
                    color: Qt.alpha(Config.bg, 0.6)
                    clip: true

                    Rectangle {
                        anchors.left: parent.left
                        anchors.top: parent.top
                        anchors.bottom: parent.bottom
                        width: Math.max(4, Math.min(parent.width, parent.width * (root.healthPct / 100.0)))
                        radius: 3
                        color: H.healthColor(root.healthPct)
                    }
                }

                // Health Capacity Details
                RowLayout {
                    Layout.fillWidth: true

                    Text {
                        text: "Capacity: " + root.energyFull.toFixed(1) + " Wh / " + root.energyDesign.toFixed(1) + " Wh"
                        font.family: Config.fontName
                        font.pixelSize: 9
                        color: Config.fgDim
                    }

                    Item { Layout.fillWidth: true }

                    Text {
                        text: root.cycles > 0 ? root.cycles.toLocaleString() + " cycles" : "Cycles N/A"
                        font.family: Config.fontName
                        font.pixelSize: 9
                        font.bold: true
                        color: Config.accentActive
                    }
                }
            }
        }

        // Vast Details Section Title
        RowLayout {
            Layout.fillWidth: true

            Text {
                text: "Battery Metrics & Details"
                font.family: Config.fontName
                font.pixelSize: 10
                font.bold: true
                color: Config.fgSub
            }

            Item { Layout.fillWidth: true }

            Text {
                text: root.batName
                font.family: Config.fontName
                font.pixelSize: 9
                color: Config.fgDim
            }
        }

        // Vast Details 2-Column Grid
        GridLayout {
            Layout.fillWidth: true
            columns: 2
            rowSpacing: 6
            columnSpacing: 6

            // Row 1: Power Rate & Voltage
            StatTile {
                Layout.fillWidth: true
                icon: Config.gBolt
                iconColor: root.isCharging ? Config.ok : Config.accentActive
                label: "POWER DRAW"
                value: root.powerRate.toFixed(1) + " W"
                sub: root.isCharging ? "Charging wattage" : (root.powerRate > 0 ? "Discharge rate" : "Idle on AC")
            }

            StatTile {
                Layout.fillWidth: true
                icon: Config.gActivity
                iconColor: Config.highColor
                label: "VOLTAGE"
                value: root.voltage.toFixed(2) + " V"
                sub: root.voltageMinDesign > 0 ? "Min: " + root.voltageMinDesign.toFixed(2) + " V" : "Operating voltage"
            }

            // Row 2: Energy Stored & Full Capacity
            StatTile {
                Layout.fillWidth: true
                icon: Config.gBatteryHalf
                iconColor: root.dynamicColor
                label: "CURRENT ENERGY"
                value: root.energyNow.toFixed(1) + " Wh"
                sub: root.percentage + "% of capacity"
            }

            StatTile {
                Layout.fillWidth: true
                icon: Config.gShield
                iconColor: Config.accent
                label: "DESIGN CAPACITY"
                value: root.energyDesign.toFixed(1) + " Wh"
                sub: "Full: " + root.energyFull.toFixed(1) + " Wh"
            }

            // Row 3: Cycles & Technology
            StatTile {
                Layout.fillWidth: true
                icon: Config.gCycle
                iconColor: Config.accentActive
                label: "CHARGE CYCLES"
                value: root.cycles > 0 ? root.cycles.toLocaleString() : "N/A"
                sub: "Lifetime cycles"
            }

            StatTile {
                Layout.fillWidth: true
                icon: Config.gChip
                iconColor: Config.warn
                label: "CHEMISTRY"
                value: root.technology || "Li-ion"
                sub: "Technology type"
            }

            // Row 4: Model & Manufacturer
            StatTile {
                Layout.fillWidth: true
                icon: Config.gDevice
                iconColor: Config.normalColor
                label: "BATTERY MODEL"
                value: root.modelName || "Standard"
                sub: root.serialNumber ? "S/N: " + root.serialNumber : root.batName
            }

            StatTile {
                Layout.fillWidth: true
                icon: Config.gBuilding
                iconColor: Config.fgSub
                label: "MANUFACTURER"
                value: root.vendor || "OEM"
                sub: root.acOnline ? "Supply: AC Online" : "Supply: Battery"
            }
        }

        // Peripherals Section (if any devices connected)
        ColumnLayout {
            Layout.fillWidth: true
            spacing: 6
            visible: root.peripherals.length > 0

            Text {
                text: "Connected Devices"
                font.family: Config.fontName
                font.pixelSize: 10
                font.bold: true
                color: Config.fgSub
            }

            Repeater {
                model: root.peripherals
                delegate: DeviceRow {
                    Layout.fillWidth: true
                    modelName: modelData.model
                    percentage: modelData.percentage
                    deviceType: modelData.type
                }
            }
        }

        // Footer status line
        RowLayout {
            Layout.fillWidth: true
            spacing: 6

            Text {
                text: root.lastUpdated ? "Updated " + root.lastUpdated : "Live monitoring"
                font.family: Config.fontName
                font.pixelSize: 9
                color: Config.fgDim
            }

            Item { Layout.fillWidth: true }

            Text {
                text: "Press Esc to close"
                font.family: Config.fontName
                font.pixelSize: 9
                color: Config.fgDim
            }
        }
    }
}
