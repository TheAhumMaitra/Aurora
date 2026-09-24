import QtQuick
import QtQuick.Layouts
import Qt.labs.folderlistmodel

// An expandable theme switcher: a full-width header tile that reveals a
// scrollable two-column grid of every theme in ~/.config/themes. Clicking a
// card emits `applyTheme(dir)`; the active theme is highlighted with an accent
// border.
Item {
    id: root

    property string homeDir: ""
    property string activeThemeDir: ""
    property bool expanded: false

    signal applyTheme(string dir)

    implicitWidth: Config.popupWidth
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
    }

    function prettify(name) {
        return name.length ? name.replace(/-/g, " ") : ""
    }

    ColumnLayout {
        id: body
        anchors.fill: parent
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

        GridView {
            id: grid
            visible: root.expanded
            Layout.fillWidth: true
            Layout.preferredHeight: Math.min(
                Math.ceil(themeModel.count / 2) * Config.themeCellHeight,
                Config.maxThemeGridHeight)
            model: themeModel
            cellWidth: Config.themeCellWidth
            cellHeight: Config.themeCellHeight
            clip: true
            interactive: true

            delegate: Item {
                width: Config.themeCellWidth
                height: Config.themeCellHeight

                Rectangle {
                    id: card
                    anchors.fill: parent
                    anchors.margins: 4
                    radius: 9
                    color: themeMouse.hovered ? Config.bgHover : Config.bgElevated
                    border.width: 1
                    border.color: model.fileName === root.activeThemeDir
                        ? Qt.alpha(Config.accent, 0.9)
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

                            Image {
                                anchors.fill: parent
                                source: model.filePath + "/preview.png"
                                fillMode: Image.PreserveAspectCrop
                                clip: true
                                sourceSize: Qt.size(160, 56)
                            }
                        }

                        Text {
                            Layout.fillWidth: true
                            Layout.alignment: Qt.AlignLeft
                            text: root.prettify(model.fileName)
                            font.family: Config.fontName
                            font.pixelSize: Config.themeLabelSize
                            color: model.fileName === root.activeThemeDir ? Config.accentActive : Config.fg
                            elide: Text.ElideRight
                        }
                    }

                    MouseArea {
                        id: themeMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: root.applyTheme(model.fileName)
                    }
                }
            }
        }
    }
}