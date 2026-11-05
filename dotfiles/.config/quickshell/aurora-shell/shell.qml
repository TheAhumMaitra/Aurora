import Quickshell
import Quickshell.Io
import "bluetooth_manager" as BluetoothManager
import "networks_manager" as NetworkManager
import "volume_adjuster" as VolumeManager
import "control_center" as ControlCenter
import "battery_menu" as BatteryManager
import "media_player" as MediaManager
import "display_manager" as DisplayManager

ShellRoot {
    id: aurora

    BluetoothManager.Bluetooth { id: bluetooth }
    NetworkManager.Networks   { id: networks }
    VolumeManager.Volume      { id: volume }
    BatteryManager.Battery    { id: battery }
    MediaManager.Media        { id: media }
    DisplayManager.Display    { id: display }
    ControlCenter.ControlCenter {
        id: controlCenter
        bluetoothManager: bluetooth
        networksManager: networks
        volumeManager: volume
        batteryManager: battery
        mediaManager: media
        displayManager: display
    }

    function manager(name: string): var {
        if (name === "bluetooth" || name === "bluetooth_manager" || name === "bt") return bluetooth
        if (name === "network" || name === "networks" || name === "networks_manager" || name === "wifi") return networks
        if (name === "volume" || name === "volume_adjuster" || name === "vol") return volume
        if (name === "battery" || name === "battery_menu" || name === "battery_manager" || name === "bat" || name === "power") return battery
        if (name === "media" || name === "media_player" || name === "mpris" || name === "music" || name === "player") return media
        if (name === "control" || name === "control_center" || name === "cc" || name === "dashboard" || name === "quick") return controlCenter
        if (name === "display" || name === "display_manager" || name === "disp" || name === "monitors" || name === "monitor" || name === "screen" || name === "brightness") return display
        return null
    }

    IpcHandler {
        target: "aurora"

        function open(component: string): void {
            var m = aurora.manager(component)
            if (m) m.openPopup()
        }

        function close(component: string): void {
            var m = aurora.manager(component)
            if (m) m.closePopup()
        }

        function toggle(component: string): void {
            var m = aurora.manager(component)
            if (m) m.togglePopup()
        }
    }
}
