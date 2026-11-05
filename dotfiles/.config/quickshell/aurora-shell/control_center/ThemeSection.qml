import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Qt.labs.folderlistmodel
import Quickshell
import "."

// An expandable theme switcher: a full-width header tile that reveals a
// live-searchable two-column grid of every theme in ~/.config/themes.
// The width is strictly constrained to the control center body and dynamically
// divides across exactly 2 equal columns without wrapping or layout jumping.
Item {
    id: root

    property string homeDir: Quickshell.env("HOME") || ""
    property string activeThemeDir: ""
    property bool expanded: false
    property string searchQuery: ""

    signal applyTheme(string dir)

    Layout.fillWidth: true
    implicitHeight: body.implicitHeight

    readonly property string themesPath: root.homeDir !== "" ? root.homeDir + "/.config/themes" : ""
    readonly property string themesUrl: root.themesPath !== "" ? "file://" + root.themesPath + "/" : ""

    FolderListModel {
        id: themeModel
        folder: root.themesUrl
        showDirs: true
        showFiles: false
        showDotAndDotDot: false
        sortField: FolderListModel.Name
        onStatusChanged: if (status === FolderListModel.Ready) root.updateFiltered()
    }

    property var filteredThemes: []

    function updateFiltered() {
        var res = []
        var q = root.searchQuery.toLowerCase().trim()
        for (var i = 0; i < themeModel.count; i++) {
            var fn = themeModel.get(i, "fileName")
            var fp = themeModel.get(i, "filePath")
            if (q === "" || fn.toLowerCase().indexOf(q) !== -1 || fn.replace(/-/g, " ").toLowerCase().indexOf(q) !== -1) {
                res.push({ fileName: fn, filePath: fp })
            }
        }
        root.filteredThemes = res
    }

    onSearchQueryChanged: updateFiltered()

    function prettify(name) {
        return name.length ? name.replace(/-/g, " ") : ""
    }

    ColumnLayout {
        id: body
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        spacing: 8

        Rectangle {
            id: header
            Layout.fillWidth: true
            Layout.preferredHeight: Config.themeHeaderHeight
            radius: Config.tileRadius
            color: headerMouse.hovered ? Config.bgHover : Config.bgElevated
            border.width: 1
            border.color: root.expanded
                ? Qt.alpha(Config.accent, 0.7)
                : (headerMouse.hovered ? Qt.alpha(Config.accent, 0.7) : Config.border)

            Behavior on color { ColorAnimation { duration: 130 } }
            Behavior on border.color { ColorAnimation { duration: 130 } }

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 10
                anchors.rightMargin: 10
                spacing: 10

                Rectangle {
                    Layout.preferredWidth: 32
                    Layout.preferredHeight: 32
                    Layout.alignment: Qt.AlignVCenter
                    radius: 9
                    color: root.expanded ? Qt.alpha(Config.accent, 0.28) : Config.accentSoft

                    Text {
                        anchors.centerIn: parent
                        text: Config.gPalette
                        font.family: Config.glyphFont
                        font.pixelSize: 15
                        color: Config.accentActive
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignVCenter
                    spacing: 1

                    Text {
                        Layout.fillWidth: true
                        text: "Themes"
                        font.family: Config.fontName
                        font.pixelSize: Config.textSize
                        font.bold: true
                        color: Config.fg
                        elide: Text.ElideRight
                    }

                    Text {
                        Layout.fillWidth: true
                        text: root.activeThemeDir !== ""
                            ? root.prettify(root.activeThemeDir)
                            : themeModel.count + " themes"
                        font.family: Config.fontName
                        font.pixelSize: 9
                        color: Config.fgDim
                        elide: Text.ElideRight
                    }
                }

                Rectangle {
                    Layout.preferredWidth: 22
                    Layout.preferredHeight: 22
                    Layout.alignment: Qt.AlignVCenter
                    radius: 7
                    color: Qt.alpha(Config.fg, 0.06)

                    Text {
                        anchors.centerIn: parent
                        text: Config.gChevronDown
                        font.family: Config.glyphFont
                        font.pixelSize: 9
                        color: Config.fgDim
                        rotation: root.expanded ? 180 : 0

                        Behavior on rotation {
                            NumberAnimation { duration: 150; easing.type: Easing.OutCubic }
                        }
                    }
                }
            }

            MouseArea {
                id: headerMouse
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: Qt.PointingHandCursor
                onClicked: root.expanded = !root.expanded
            }
        }

        // Search Bar when expanded
        Rectangle {
            id: searchBar
            visible: root.expanded
            Layout.fillWidth: true
            Layout.preferredHeight: 32
            radius: 8
            color: Config.bgElevated
            border.width: 1
            border.color: searchInput.activeFocus ? Qt.alpha(Config.accentActive, 0.7) : Config.border

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 8
                anchors.rightMargin: 8
                spacing: 6

                Text {
                    text: Config.gSearch
                    font.family: Config.glyphFont
                    font.pixelSize: 11
                    color: Config.fgDim
                }

                TextInput {
                    id: searchInput
                    Layout.fillWidth: true
                    font.family: Config.fontName
                    font.pixelSize: 10
                    color: Config.fg
                    clip: true
                    text: root.searchQuery
                    onTextChanged: root.searchQuery = text

                    Text {
                        anchors.fill: parent
                        text: "Search themes..."
                        font: searchInput.font
                        color: Config.fgDim
                        visible: !searchInput.text && !searchInput.activeFocus
                    }
                }

                Text {
                    visible: root.searchQuery.length > 0
                    text: Config.gTimes
                    font.family: Config.glyphFont
                    font.pixelSize: 10
                    color: Config.fgDim

                    MouseArea {
                        anchors.fill: parent
                        cursorShape: Qt.PointingHandCursor
                        onClicked: {
                            searchInput.text = ""
                            root.searchQuery = ""
                        }
                    }
                }
            }
        }

        GridView {
            id: grid
            visible: root.expanded && root.filteredThemes.length > 0
            Layout.fillWidth: true
            Layout.preferredHeight: Math.min(
                Math.ceil(root.filteredThemes.length / 2) * Config.themeCellHeight,
                Config.maxThemeGridHeight)
            model: root.filteredThemes
            cellWidth: Math.floor(grid.width / 2)
            cellHeight: Config.themeCellHeight
            clip: true
            interactive: true

            ScrollBar.vertical: ScrollBar {
                id: vbar
                policy: grid.contentHeight > grid.height ? ScrollBar.AlwaysOn : ScrollBar.AsNeeded
                active: grid.moving || vbar.hovered
                width: 4
                contentItem: Rectangle {
                    implicitWidth: 4
                    radius: 2
                    color: vbar.hovered ? Config.accentActive : Qt.alpha(Config.fg, 0.25)
                }
            }

            delegate: Item {
                width: grid.cellWidth
                height: Config.themeCellHeight

                Rectangle {
                    id: card
                    anchors.fill: parent
                    anchors.margins: 4
                    radius: 9
                    color: themeMouse.hovered ? Config.bgHover : Config.bgElevated
                    border.width: modelData.fileName === root.activeThemeDir ? 2 : 1
                    border.color: modelData.fileName === root.activeThemeDir
                        ? Config.accentActive
                        : (themeMouse.hovered ? Qt.alpha(Config.accent, 0.7) : Config.border)

                    Behavior on color { ColorAnimation { duration: 120 } }

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 6
                        spacing: 4

                        Rectangle {
                            Layout.fillWidth: true
                            Layout.preferredHeight: Config.themeImgHeight
                            radius: 6
                            color: Qt.alpha(Config.fg, 0.06)
                            clip: true

                            Image {
                                anchors.fill: parent
                                source: modelData.filePath + "/preview.png"
                                fillMode: Image.PreserveAspectCrop
                                clip: true
                                sourceSize: Qt.size(160, 56)
                            }

                            // Check badge for active theme
                            Rectangle {
                                anchors.top: parent.top
                                anchors.right: parent.right
                                anchors.margins: 3
                                width: 16
                                height: 16
                                radius: 8
                                color: Config.accentActive
                                visible: modelData.fileName === root.activeThemeDir

                                Text {
                                    anchors.centerIn: parent
                                    text: Config.gCheck
                                    font.family: Config.glyphFont
                                    font.pixelSize: 8
                                    color: "#ffffff"
                                }
                            }
                        }

                        Text {
                            Layout.fillWidth: true
                            Layout.alignment: Qt.AlignLeft
                            text: root.prettify(modelData.fileName)
                            font.family: Config.fontName
                            font.pixelSize: Config.themeLabelSize
                            font.bold: modelData.fileName === root.activeThemeDir
                            color: modelData.fileName === root.activeThemeDir ? Config.accentActive : Config.fg
                            elide: Text.ElideRight
                        }
                    }

                    MouseArea {
                        id: themeMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: root.applyTheme(modelData.fileName)
                    }
                }
            }
        }

        // Empty state when search has 0 results
        Rectangle {
            visible: root.expanded && root.filteredThemes.length === 0
            Layout.fillWidth: true
            Layout.preferredHeight: 50
            radius: 8
            color: Config.bgElevated
            border.width: 1
            border.color: Config.border

            Text {
                anchors.centerIn: parent
                text: "No themes matching \"" + root.searchQuery + "\""
                font.family: Config.fontName
                font.pixelSize: 10
                color: Config.fgDim
            }
        }
    }
}
