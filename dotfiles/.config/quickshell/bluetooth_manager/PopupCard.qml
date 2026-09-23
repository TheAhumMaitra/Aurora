import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Quickshell
import Quickshell.Bluetooth
import "helpers.js" as H

Rectangle {
    id: root

    required property var adapter

    signal requestClose()

    implicitWidth: Config.popupWidth
    implicitHeight: body.implicitHeight + 32
    width: Config.popupWidth
    radius: Config.radius
    color: Config.bg
    border.color: Config.border
    border.width: 1

    readonly property bool adapterPresent: !!root.adapter
    readonly property bool adapterOn: root.adapter && root.adapter.enabled
    readonly property bool scanning: root.adapter && root.adapter.discovering

    // Whether the popup window backing this card is visible.
    property bool active: false

    // Devices: connected first, then paired, then by name.
    property var sorted: []

    function devKey(d) {
        return (d.connected ? "c" : d.paired ? "p" : "d")
            + d.state + "|" + (d.name || d.deviceName || d.address || "")
    }

    // Debounced + change-checked: bursts of model events (discovery) coalesce
    // into a single pass, and the list is only reassigned when it really
    // changed, so the ListView keeps its scroll position and delegates.
    Timer {
        id: rebuildTimer
        interval: 90
        onTriggered: root.flushRebuild()
    }

    function scheduleRebuild() {
        rebuildTimer.restart()
    }

    function flushRebuild() {
        rebuildTimer.stop()
        if (!root.adapterOn) {
            if (root.sorted.length > 0) root.sorted = []
            return
        }
        var devs = Bluetooth.devices.values.slice()
        var seen = {}
        for (var k = 0; k < devs.length; ++k) {
            if (devs[k]) seen[devs[k].address] = true
        }
        if (root.adapter) {
            var adv = root.adapter.devices.values
            for (var j = 0; j < adv.length; ++j) {
                var d = adv[j]
                if (d && !seen[d.address]) {
                    seen[d.address] = true
                    devs.push(d)
                }
            }
        }
        devs.sort(function (a, b) {
            if (a.connected !== b.connected) return a.connected ? -1 : 1
            if (a.paired !== b.paired) return a.paired ? -1 : 1
            return (a.name || "").localeCompare(b.name || "")
        })
        var cur = root.sorted
        if (cur.length === devs.length) {
            var same = true
            for (var i = 0; i < devs.length; ++i) {
                if (root.devKey(cur[i]) !== root.devKey(devs[i])) { same = false; break }
            }
            if (same) return
        }
        root.sorted = devs
    }

    Connections {
        id: devsConn
        target: Bluetooth.devices
        function onObjectInsertedPost() { root.scheduleRebuild() }
        function onObjectRemovedPost() { root.scheduleRebuild() }
        function onValuesChanged() { root.scheduleRebuild() }
    }

    Connections {
        id: adapterConn
        function onEnabledChanged() {
            root.scheduleRebuild()
            if (root.adapter && root.adapter.enabled) root.adapter.discovering = true
        }
        function onDiscoveringChanged() {
            root.scheduleRebuild()
        }
    }

    // While the popup is open, periodically re-sync from both device models so
    // nothing gets stuck missing if BlueZ's InterfacesAdded lagged behind.
    Timer {
        id: syncTimer
        interval: 2500
        running: root.active && root.adapterOn
        onTriggered: root.scheduleRebuild()
    }

    function rehookAdapter() {
        adapterConn.target = root.adapter
    }

    onAdapterChanged: { root.rehookAdapter(); root.scheduleRebuild() }
    onActiveChanged: root.scheduleRebuild()
    Component.onCompleted: {
        root.rehookAdapter()
        root.scheduleRebuild()
        root.initScanTimer.start()
    }

    readonly property int rowCount: root.adapterOn ? root.sorted.length : 0

    // Auto-discover on start: as soon as the adapter shows up and is enabled,
    // start scanning right away, popup visibility doesn't matter. Discovery
    // keeps running until toggled off with the "Discover devices" button.
    Timer {
        id: initScanTimer
        interval: 200
        repeat: true
        onTriggered: {
            if (!root.adapter) return
            initScanTimer.stop()
            if (root.adapter.enabled) root.adapter.discovering = true
        }
    }

    function toggleDiscovering() {
        if (!root.adapterOn || !root.adapter) return
        root.adapter.discovering = !root.adapter.discovering
    }

    // ------------------------------------------------------------ behavior
    property string view: "list" // "list" | "detail"
    property var pendingDevice: null

    function pick(device) {
        if (!device
            || device.state === BluetoothDeviceState.Connecting
            || device.state === BluetoothDeviceState.Disconnecting) return
        if (device.connected) {
            root.pendingDevice = device
            root.view = "detail"
        } else if (device.paired) {
            device.connect()
        } else {
            device.pair()
        }
    }

    function backToList() {
        root.view = "list"
        root.pendingDevice = null
    }

    function togglePower() {
        if (root.adapter) root.adapter.enabled = !root.adapter.enabled
    }

    // ------------------------------------------------------------------ UI
    ColumnLayout {
        id: body
        anchors.fill: parent
        anchors.margins: 16
        spacing: 10

        // header ------------------------------------------------------------
        RowLayout {
            Layout.fillWidth: true
            spacing: 10

            Text {
                text: "Bluetooth"
                font.family: Config.fontName
                font.pixelSize: Config.titleSize
                font.bold: true
                color: Config.fg
            }

            Item {
                Layout.fillWidth: true
            }

            Rectangle {
                id: powerBtn
                Layout.alignment: Qt.AlignVCenter
                width: 26
                height: 26
                radius: 7
                color: powerHover.hovered ? Config.bgHover : "transparent"
                opacity: root.adapterPresent ? 1 : 0.4

                Text {
                    anchors.centerIn: parent
                    text: Config.gBluetooth
                    font.family: Config.glyphFont
                    font.pixelSize: 12
                    color: root.adapterOn ? Config.accentActive : Config.fgDim
                }

                Rectangle {
                    anchors.right: parent.right
                    anchors.rightMargin: 3
                    anchors.bottom: parent.bottom
                    anchors.bottomMargin: 3
                    width: 6
                    height: 6
                    radius: 3
                    color: root.adapterOn ? Config.ok : Config.fgDim
                }

                MouseArea {
                    id: powerHover
                    anchors.fill: parent
                    enabled: root.adapterPresent
                    cursorShape: Qt.PointingHandCursor
                    onClicked: root.togglePower()
                }
            }

            Rectangle {
                width: 26
                height: 26
                radius: 7
                color: gearHover.hovered ? Config.bgHover : "transparent"

                Text {
                    anchors.centerIn: parent
                    text: Config.gTimes
                    font.family: Config.glyphFont
                    font.pixelSize: 12
                    color: Config.fgDim
                }

                MouseArea {
                    id: gearHover
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

        // device list -------------------------------------------------------
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: root.rowCount === 0
                ? Config.emptyListHeight
                : Math.min(Config.maxListHeight, root.rowCount * Config.rowHeight)
            radius: Config.radiusSm
            color: Config.bgElevated
            border.color: Config.border
            border.width: 1
            clip: true

            Behavior on Layout.preferredHeight {
                enabled: !root.scanning
                NumberAnimation { duration: 160; easing.type: Easing.OutCubic }
            }

            ListView {
                id: list
                anchors.fill: parent
                model: root.adapterOn ? root.sorted : []

                ScrollBar.vertical: ScrollBar {
                    visible: list.contentHeight > Config.maxListHeight
                }

                delegate: DeviceRow {
                    required property var modelData
                    device: modelData
                    width: list.width
                    onClicked: root.pick(modelData)
                }
            }

            Text {
                anchors.centerIn: parent
                visible: root.rowCount === 0
                text: !root.adapterPresent
                          ? "No Bluetooth adapter found"
                          : (!root.adapterOn
                             ? "Bluetooth is off"
                             : (root.scanning
                                ? "Searching for devices\u2026"
                                : "No devices found"))
                font.family: Config.fontName
                font.pixelSize: Config.textSize
                color: Config.fgDim
            }
        }

        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: Config.border
            opacity: 0.6
            visible: root.rowCount > 0
        }

        // footer: discover --------------------------------------------------
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 38
            radius: Config.radiusSm
            color: scanHover.hovered && root.adapterOn ? Config.bgHover : Config.bgElevated
            border.color: Config.border
            opacity: root.adapterOn ? 1 : 0.55

            Row {
                anchors.centerIn: parent
                spacing: 9

                Text {
                    anchors.verticalCenter: parent.verticalCenter
                    text: Config.gRefresh
                    font.family: Config.glyphFont
                    font.pixelSize: 12
                    color: root.scanning ? Config.accentActive : Config.accent

                    NumberAnimation on rotation {
                        running: root.scanning
                        from: 0
                        to: 360
                        duration: 900
                        loops: Animation.Infinite
                    }
                }

                Text {
                    anchors.verticalCenter: parent.verticalCenter
                    text: root.scanning ? "Discovering\u2026" : "Discover devices"
                    font.family: Config.fontName
                    font.pixelSize: Config.textSize
                    color: Config.fgSub
                }
            }

            MouseArea {
                id: scanHover
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: Qt.PointingHandCursor
                enabled: root.adapterOn
                onClicked: root.toggleDiscovering()
            }
        }
    }

    // connected device dialog ---------------------------------------------
    Item {
        anchors.fill: parent
        visible: root.view === "detail"
        z: 10

        Rectangle {
            anchors.fill: parent
            radius: Config.radius
            color: Config.bg
            border.color: Config.border
            border.width: 1

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 18
                spacing: 12

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 8

                    Text {
                        Layout.fillWidth: true
                        text: root.pendingDevice
                              ? (root.pendingDevice.name || root.pendingDevice.deviceName || root.pendingDevice.address)
                              : ""
                        font.family: Config.fontName
                        font.pixelSize: Config.titleSize
                        font.bold: true
                        color: Config.ok
                        elide: Text.ElideRight
                    }

                    Text {
                        text: Config.gCheck
                        font.family: Config.glyphFont
                        font.pixelSize: 13
                        color: Config.ok
                    }
                }

                Text {
                    Layout.fillWidth: true
                    text: {
                        var d = root.pendingDevice
                        if (!d) return ""
                        if (d.batteryAvailable) return "Connected  \u00b7  " + H.pct(d.battery) + "% battery"
                        return "Connected  \u00b7  " + (d.address || "")
                    }
                    font.family: Config.fontName
                    font.pixelSize: 10
                    color: Config.fgSub
                    elide: Text.ElideRight
                }

                Item {
                    Layout.fillHeight: true
                }

                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 38
                    radius: Config.radiusSm
                    color: discHover.hovered ? Qt.darker(Config.danger, 1.15) : Config.danger

                    Text {
                        anchors.centerIn: parent
                        text: Config.gPower + "  Disconnect"
                        font.family: Config.glyphFont
                        font.pixelSize: 11
                        font.bold: true
                        color: "#ffffff"
                    }

                    MouseArea {
                        id: discHover
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: {
                            var d = root.pendingDevice
                            root.backToList()
                            if (d) d.disconnect()
                        }
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 38
                    radius: Config.radiusSm
                    color: forgetHover.hovered ? Config.bgHover : Config.bgElevated
                    border.color: Config.border
                    border.width: 1

                    Text {
                        anchors.centerIn: parent
                        text: Config.gTrash + "  Forget device"
                        font.family: Config.glyphFont
                        font.pixelSize: 11
                        font.bold: true
                        color: Config.danger
                    }

                    MouseArea {
                        id: forgetHover
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: {
                            var d = root.pendingDevice
                            root.backToList()
                            if (d) {
                                d.disconnect()
                                d.forget()
                            }
                        }
                    }
                }

                Text {
                    Layout.fillWidth: true
                    text: Config.gArrowLeft + "  Back"
                    font.family: Config.glyphFont
                    font.pixelSize: 11
                    horizontalAlignment: Text.AlignHCenter
                    color: Config.fgSub

                    MouseArea {
                        anchors.fill: parent
                        anchors.margins: -6
                        cursorShape: Qt.PointingHandCursor
                        onClicked: root.backToList()
                    }
                }
            }
        }
    }
}