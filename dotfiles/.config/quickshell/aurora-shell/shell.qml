import Quickshell
import Quickshell.Io
import "bluetooth_manager" as BluetoothManager
import "networks_manager" as NetworkManager
import "volume_adjuster" as VolumeManager
import "control_center" as ControlCenter

ShellRoot {
    id: aurora

    BluetoothManager.Bluetooth { id: bluetooth }
    NetworkManager.Networks   { id: networks }
    VolumeManager.Volume      { id: volume }
    ControlCenter.ControlCenter {
        id: controlCenter
        bluetoothManager: bluetooth
        networksManager: networks
        volumeManager: volume
    }

    function manager(name: string): var {
        if (name === "bluetooth" || name === "bluetooth_manager" || name === "bt") return bluetooth
        if (name === "network" || name === "networks" || name === "networks_manager" || name === "wifi") return networks
        if (name === "volume" || name === "volume_adjuster" || name === "vol") return volume
        if (name === "control" || name === "control_center" || name === "cc" || name === "dashboard" || name === "quick") return controlCenter
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
