#!/usr/bin/env python3
import glob
import json
import os
import subprocess
import sys

def get_battery_info():
    data = {
        "present": False,
        "name": "BAT0",
        "percentage": 0,
        "status": "Unknown",
        "power_w": 0.0,
        "power_signed_w": 0.0,
        "voltage_v": 0.0,
        "voltage_min_design_v": 0.0,
        "energy_now_wh": 0.0,
        "energy_full_wh": 0.0,
        "energy_full_design_wh": 0.0,
        "health_pct": 100.0,
        "cycles": 0,
        "model": "Unknown",
        "vendor": "Generic",
        "technology": "Li-ion",
        "serial": "",
        "ac_online": False,
        "profile": "balanced",
        "time_str": "",
        "peripherals": []
    }

    # AC Line Power detection
    for ac in glob.glob("/sys/class/power_supply/A*") + glob.glob("/sys/class/power_supply/AC*"):
        try:
            with open(os.path.join(ac, "online"), "r") as f:
                if f.read().strip() == "1":
                    data["ac_online"] = True
                    break
        except Exception:
            pass

    # Battery detection
    batteries = sorted(glob.glob("/sys/class/power_supply/BAT*"))
    curr_a = 0.0
    charge_now_ah = 0.0
    charge_full_ah = 0.0
    charge_full_design_ah = 0.0

    if batteries:
        bat = batteries[0]
        data["present"] = True
        data["name"] = os.path.basename(bat)
        uevent_path = os.path.join(bat, "uevent")
        if os.path.exists(uevent_path):
            try:
                with open(uevent_path, "r") as f:
                    for line in f:
                        if "=" not in line:
                            continue
                        k, v = line.strip().split("=", 1)
                        if k == "POWER_SUPPLY_STATUS":
                            data["status"] = v
                        elif k == "POWER_SUPPLY_CAPACITY":
                            try:
                                data["percentage"] = int(v)
                            except ValueError:
                                pass
                        elif k == "POWER_SUPPLY_POWER_NOW":
                            try:
                                data["power_w"] = round(abs(int(v)) / 1e6, 2)
                            except ValueError:
                                pass
                        elif k == "POWER_SUPPLY_CURRENT_NOW":
                            try:
                                curr_a = abs(int(v)) / 1e6
                            except ValueError:
                                pass
                        elif k == "POWER_SUPPLY_VOLTAGE_NOW":
                            try:
                                data["voltage_v"] = round(int(v) / 1e6, 2)
                            except ValueError:
                                pass
                        elif k == "POWER_SUPPLY_VOLTAGE_MIN_DESIGN":
                            try:
                                data["voltage_min_design_v"] = round(int(v) / 1e6, 2)
                            except ValueError:
                                pass
                        elif k == "POWER_SUPPLY_ENERGY_NOW":
                            try:
                                data["energy_now_wh"] = round(int(v) / 1e6, 2)
                            except ValueError:
                                pass
                        elif k == "POWER_SUPPLY_CHARGE_NOW":
                            try:
                                charge_now_ah = int(v) / 1e6
                            except ValueError:
                                pass
                        elif k == "POWER_SUPPLY_ENERGY_FULL":
                            try:
                                data["energy_full_wh"] = round(int(v) / 1e6, 2)
                            except ValueError:
                                pass
                        elif k == "POWER_SUPPLY_CHARGE_FULL":
                            try:
                                charge_full_ah = int(v) / 1e6
                            except ValueError:
                                pass
                        elif k == "POWER_SUPPLY_ENERGY_FULL_DESIGN":
                            try:
                                data["energy_full_design_wh"] = round(int(v) / 1e6, 2)
                            except ValueError:
                                pass
                        elif k == "POWER_SUPPLY_CHARGE_FULL_DESIGN":
                            try:
                                charge_full_design_ah = int(v) / 1e6
                            except ValueError:
                                pass
                        elif k == "POWER_SUPPLY_CYCLE_COUNT":
                            try:
                                data["cycles"] = int(v)
                            except ValueError:
                                pass
                        elif k == "POWER_SUPPLY_MODEL_NAME":
                            data["model"] = v
                        elif k == "POWER_SUPPLY_MANUFACTURER":
                            data["vendor"] = v
                        elif k == "POWER_SUPPLY_TECHNOLOGY":
                            data["technology"] = v
                        elif k == "POWER_SUPPLY_SERIAL_NUMBER":
                            data["serial"] = v.strip()
            except Exception:
                pass

        # Derivations if energy/power are represented via charge/current
        if data["power_w"] == 0.0 and curr_a > 0 and data["voltage_v"] > 0:
            data["power_w"] = round(curr_a * data["voltage_v"], 2)
        if data["energy_now_wh"] == 0.0 and charge_now_ah > 0 and data["voltage_v"] > 0:
            data["energy_now_wh"] = round(charge_now_ah * data["voltage_v"], 2)
        if data["energy_full_wh"] == 0.0 and charge_full_ah > 0 and data["voltage_v"] > 0:
            data["energy_full_wh"] = round(charge_full_ah * data["voltage_v"], 2)
        if data["energy_full_design_wh"] == 0.0 and charge_full_design_ah > 0 and data["voltage_v"] > 0:
            data["energy_full_design_wh"] = round(charge_full_design_ah * data["voltage_v"], 2)

        # Health percentage calculation
        if data["energy_full_design_wh"] > 0 and data["energy_full_wh"] > 0:
            data["health_pct"] = round((data["energy_full_wh"] / data["energy_full_design_wh"]) * 100, 1)

        # Signed power (positive = charging, negative = discharging)
        is_charging = data["status"].lower() == "charging"
        data["power_signed_w"] = data["power_w"] if is_charging else -data["power_w"]

        # Time calculation
        if is_charging and data["power_w"] > 0.3:
            rem_wh = max(0.0, data["energy_full_wh"] - data["energy_now_wh"])
            hrs = rem_wh / data["power_w"]
            h = int(hrs)
            m = int((hrs - h) * 60)
            data["time_str"] = f"{h}h {m}m until full" if h > 0 else f"{m}m until full"
        elif data["status"].lower() == "discharging" and data["power_w"] > 0.3:
            hrs = data["energy_now_wh"] / data["power_w"]
            h = int(hrs)
            m = int((hrs - h) * 60)
            data["time_str"] = f"{h}h {m}m remaining" if h > 0 else f"{m}m remaining"
        elif data["status"].lower() in ("full", "fully-charged") or data["percentage"] >= 100:
            data["time_str"] = "Fully charged"
        elif data["ac_online"]:
            data["time_str"] = "Plugged in, idle"
        else:
            data["time_str"] = "Calculating..."

    # Power Profile check
    try:
        res = subprocess.run(
            ["powerprofilesctl", "get"],
            capture_output=True,
            text=True,
            timeout=1
        )
        if res.returncode == 0 and res.stdout.strip():
            data["profile"] = res.stdout.strip()
    except Exception:
        pass

    # Optional: check DBus for peripherals with batteries (mice, keyboards, headsets)
    try:
        import dbus
        bus = dbus.SystemBus()
        upower = bus.get_object("org.freedesktop.UPower", "/org/freedesktop/UPower")
        dev_paths = upower.EnumerateDevices(dbus_interface="org.freedesktop.UPower")
        for p in dev_paths:
            dev = bus.get_object("org.freedesktop.UPower", p)
            props = dbus.Interface(dev, "org.freedesktop.DBus.Properties").GetAll("org.freedesktop.UPower.Device")
            dtype = int(props.get("Type", 0))
            # 1: line power, 2: battery (laptop). Ignore these for peripherals.
            if dtype not in (1, 2) and bool(props.get("IsPresent", True)):
                pct = float(props.get("Percentage", 0))
                model = str(props.get("Model", "Peripheral"))
                type_name = "device"
                if dtype == 5:
                    type_name = "mouse"
                elif dtype == 6:
                    type_name = "keyboard"
                elif dtype in (18, 19, 20):
                    type_name = "audio"
                elif dtype == 13:
                    type_name = "gamepad"
                data["peripherals"].append({
                    "model": model,
                    "percentage": round(pct),
                    "type": type_name
                })
    except Exception:
        pass

    return data

if __name__ == "__main__":
    info = get_battery_info()
    # Output single-line JSON for easy SplitParser consumption
    print(json.dumps(info))
