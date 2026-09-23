import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Quickshell
import Quickshell.Io
import Quickshell.Networking
import "helpers.js" as H

Rectangle {
    id: root

    required property var wifiDevice

    signal requestClose()

    implicitWidth: Config.popupWidth
    implicitHeight: body.implicitHeight + 32
    width: Config.popupWidth
    radius: Config.radius
    color: Config.bg
    border.color: Config.border
    border.width: 1

    // Styled toggle switch with a leading icon.
    component IconSwitch: Item {
        id: isw
        property bool checked: false
        property string icon: ""
        signal clicked()

        width: 62
        height: 22
        implicitWidth: 62
        implicitHeight: 22

        Text {
            id: isIcon
            width: 18
            anchors.left: parent.left
            anchors.verticalCenter: parent.verticalCenter
            text: isw.icon
            font.family: Config.glyphFont
            font.pixelSize: 12
            horizontalAlignment: Text.AlignHCenter
            color: isw.checked ? Config.accent : Config.fgDim

            Behavior on color { ColorAnimation { duration: 150 } }
        }

        Rectangle {
            id: track
            width: 38
            height: 22
            radius: 11
            anchors.right: parent.right
            anchors.verticalCenter: parent.verticalCenter
            color: isw.checked ? Config.accent : Qt.alpha(Config.fg, 0.12)
            border.width: 1
            border.color: isw.checked ? Config.accent : Qt.alpha(Config.fg, 0.25)

            Behavior on color { ColorAnimation { duration: 150 } }
            Behavior on border.color { ColorAnimation { duration: 150 } }

            Rectangle {
                id: knob
                width: 18
                height: 18
                radius: 9
                color: "#ffffff"
                border.color: Qt.alpha("#000000", 0.15)
                border.width: 1
                y: (track.height - knob.height) / 2
                x: isw.checked ? track.width - knob.width - 2 : 2

                Behavior on x { NumberAnimation { duration: 160; easing.type: Easing.OutCubic } }
            }
        }

        MouseArea {
            anchors.fill: parent
            cursorShape: Qt.PointingHandCursor
            onClicked: isw.clicked()
        }
    }

    // Networks, connected first, then by signal strength.
    property var sorted: []

    function rebuild() {
        if (!root.wifiDevice) {
            root.sorted = []
            return
        }
        var nets = root.wifiDevice.networks.values.slice()
        nets.sort(function (a, b) {
            if (a.connected !== b.connected) return a.connected ? -1 : 1
            var d = b.signalStrength - a.signalStrength
            if (d !== 0) return d
            return (a.name || "").localeCompare(b.name || "")
        })
        root.sorted = nets
    }

    Connections {
        id: netsConn
        function onObjectInsertedPost() { root.rebuild() }
        function onObjectRemovedPost() { root.rebuild() }
        function onValuesChanged() { root.rebuild() }
    }

    function rehook() {
        netsConn.target = root.wifiDevice ? root.wifiDevice.networks : null
        root.rebuild()
    }

    onWifiDeviceChanged: root.rehook()
    Component.onCompleted: { root.rehook(); root.pollRadio() }

    // ------------------------------------------------------------- radio state
    // Airplane mode is derived from NetworkManager's wifi + wwan radios.
    property bool airplaneMode: false
    property bool wwanEnabled: false

    function parseRadio(data) {
        if (typeof data !== "string") return
        var line = data.trim()
        if (line === "") return
        var parts = line.split(":")
        if (parts.length < 2) return
        var w = parts[0]
        var ww = parts[1]
        if (w !== "enabled" && w !== "disabled") return
        if (ww !== "enabled" && ww !== "disabled") return
        root.wwanEnabled = ww === "enabled"
        root.airplaneMode = w === "disabled" && ww === "disabled"
    }

    Process {
        id: radioStatus
        command: ["nmcli", "-t", "-f", "WIFI,WWAN", "radio"]
        stdout: SplitParser {
            onRead: function(data) { root.parseRadio(data) }
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

    function setRadioAll(enabled) {
        radioCmd.exec(["nmcli", "radio", "all", enabled ? "on" : "off"])
        root.pollSoon()
    }

    function toggleAirplane() {
        var target = !root.airplaneMode
        root.airplaneMode = target
        if (target) {
            root.wwanEnabled = false
            setRadioAll(false)
        } else {
            root.wwanEnabled = true
            setRadioAll(true)
            rescanSoon.restart()
        }
    }

    function setWifi(enabled) {
        Networking.wifiEnabled = enabled
        if (enabled && root.airplaneMode) {
            root.airplaneMode = false
            root.wwanEnabled = true
            setRadioAll(true)
            rescanSoon.restart()
        } else {
            root.pollSoon()
        }
    }

    Timer {
        id: rescanSoon
        interval: 1200
        onTriggered: root.rescan()
    }

    readonly property int rowCount: Networking.wifiEnabled ? root.sorted.length : 0

    // ------------------------------------------------------------ behavior
    property string view: "list" // "list" | "password" | "active"
    property var pendingNet: null

    readonly property var currentNet: root.sorted.length > 0 && root.sorted[0].connected ? root.sorted[0] : null

    function pick(net) {
        if (!net || net.stateChanging || root.view !== "list") return
        if (net.connected) {
            root.pendingNet = net
            root.view = "active"
        } else if (H.isOpen(net.security) || net.known) {
            net.connect()
        } else {
            root.pendingNet = net
            root.view = "password"
            pwdField.text = ""
            pwdField.forceActiveFocus()
        }
    }

    function submitPsk() {
        var psk = pwdField.text
        var net = root.pendingNet
        if (!net || psk === "") return
        root.pendingNet = null
        root.view = "list"
        pwdField.text = ""
        net.connectWithPsk(psk)
    }

    function backToList() {
        root.view = "list"
        root.pendingNet = null
    }

    function rescan() {
        if (!root.wifiDevice || !Networking.wifiEnabled) return
        root.scanning = true
        scanTimer.restart()
        root.wifiDevice.scannerEnabled = false
        root.wifiDevice.scannerEnabled = true
    }

    property bool scanning: false
    Timer {
        id: scanTimer
        interval: 2000
        onTriggered: root.scanning = false
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
                text: "Wi-Fi"
                font.family: Config.fontName
                font.pixelSize: Config.titleSize
                font.bold: true
                color: Config.fg
            }

            Item { Layout.fillWidth: true }

            IconSwitch {
                Layout.alignment: Qt.AlignVCenter
                icon: Config.gPlane
                checked: root.airplaneMode
                onClicked: root.toggleAirplane()
            }

            IconSwitch {
                Layout.alignment: Qt.AlignVCenter
                icon: Config.gWifi
                checked: Networking.wifiEnabled
                onClicked: root.setWifi(!Networking.wifiEnabled)
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

        // network list ------------------------------------------------------
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: Math.min(Config.maxListHeight, root.rowCount * Config.rowHeight)
            radius: Config.radiusSm
            color: Config.bgElevated
            border.color: Config.border
            border.width: 1
            clip: true

            Behavior on Layout.preferredHeight {
                NumberAnimation { duration: 180; easing.type: Easing.OutCubic }
            }

            ListView {
                id: list
                anchors.fill: parent
                model: (Networking.wifiEnabled && root.wifiDevice) ? root.sorted : []

                ScrollBar.vertical: ScrollBar {
                    visible: list.contentHeight > Config.maxListHeight
                }

                delegate: NetworkRow {
                    required property var modelData
                    network: modelData
                    width: list.width
                    onClicked: root.pick(modelData)
                }
            }

            Text {
                anchors.centerIn: parent
                visible: root.rowCount === 0
                text: !root.wifiDevice
                      ? "No Wi-Fi adapter found"
                      : (!Networking.wifiEnabled
                         ? "Wi-Fi is off"
                         : "No networks found")
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

        // footer: refresh ---------------------------------------------------
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 38
            radius: Config.radiusSm
            color: refreshHover.hovered ? Config.bgHover : Config.bgElevated
            border.color: Config.border

            Row {
                anchors.centerIn: parent
                spacing: 9

                Text {
                    anchors.verticalCenter: parent.verticalCenter
                    text: Config.gRefresh
                    font.family: Config.glyphFont
                    font.pixelSize: 12
                    color: Config.accent

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
                    text: "Refresh networks"
                    font.family: Config.fontName
                    font.pixelSize: Config.textSize
                    color: Config.fgSub
                }
            }

            MouseArea {
                id: refreshHover
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: Qt.PointingHandCursor
                enabled: !!root.wifiDevice && Networking.wifiEnabled
                onClicked: root.rescan()
            }
        }
    }

    // password dialog ------------------------------------------------------
    Item {
        anchors.fill: parent
        visible: root.view === "password"
        z: 10

        Rectangle {
            id: pwdCard
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
                        text: "Password"
                        font.family: Config.fontName
                        font.pixelSize: Config.titleSize
                        font.bold: true
                        color: Config.fg
                    }

                    Text {
                        text: Config.gTimes
                        font.family: Config.glyphFont
                        font.pixelSize: 11
                        color: Config.fgDim

                        MouseArea {
                            anchors.fill: parent
                            anchors.margins: -6
                            cursorShape: Qt.PointingHandCursor
                            onClicked: root.backToList()
                        }
                    }
                }

                Text {
                    Layout.fillWidth: true
                    text: root.pendingNet ? root.pendingNet.name : ""
                    font.family: Config.fontName
                    font.pixelSize: 10
                    color: Config.fgSub
                    elide: Text.ElideRight
                }

                TextField {
                    id: pwdField
                    Layout.fillWidth: true
                    Layout.preferredHeight: 38
                    echoMode: TextInput.Password
                    placeholderText: "Enter password"
                    placeholderTextColor: Config.fgDim
                    color: Config.fg
                    selectionColor: Qt.alpha(Config.accentActive, 0.4)
                    font.family: Config.fontName
                    font.pixelSize: Config.textSize
                    background: Rectangle {
                        radius: Config.radiusSm
                        color: Config.bgElevated
                        border.color: Config.border
                        border.width: 1
                    }
                    onAccepted: root.submitPsk()
                }

                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 38
                    radius: Config.radiusSm
                    color: connectHover.hovered ? Config.accentActive : Config.accent

                    Text {
                        anchors.centerIn: parent
                        text: "Connect"
                        font.family: Config.fontName
                        font.pixelSize: Config.textSize
                        font.bold: true
                        color: "#ffffff"
                    }

                    MouseArea {
                        id: connectHover
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: root.submitPsk()
                    }
                }
            }
        }
    }

    // active connection dialog ---------------------------------------------
    Item {
        anchors.fill: parent
        visible: root.view === "active"
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
                        text: root.pendingNet ? root.pendingNet.name : ""
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
                    text: "Connected  \u00b7  " + (root.pendingNet ? Math.round(root.pendingNet.signalStrength * 100) : 0) + "%"
                    font.family: Config.fontName
                    font.pixelSize: 10
                    color: Config.fgSub
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
                            var net = root.pendingNet
                            root.backToList()
                            if (net) net.disconnect()
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