#!/usr/bin/env python3
import os
import signal
from pathlib import Path
import platform
import subprocess
import json


def normalize_executable_name(name: str, system: str):
    """
    On Linux/macOS: remove '.exe' if present.
    On Windows: keep as-is.
    """
    if system != "windows" and name.lower().endswith(".exe"):
        return name[:-4]
    return name


def kill_by_name(proc_name: str, system: str):
    print(f"  Killing by name: {proc_name}")

    try:
        if system == "windows":
            subprocess.call(
                ["taskkill", "/IM", proc_name, "/F"],
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL
            )
        else:
            subprocess.call(
                ["pkill", "-f", proc_name],
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL
            )
    except Exception as e:
        print(f"    Error killing '{proc_name}': {e}")


def load_service_exec_names(script_dir: Path, system: str):
    host_stack = script_dir / "host_stack.json"
    if not host_stack.exists():
        return []

    try:
        with open(host_stack, "r") as f:
            data = json.load(f)
    except Exception:
        return []

    names = []
    for svc in data.get("services", []):
        raw = svc.get("path", "")
        if not raw:
            continue

        exe = Path(raw).name
        exe = normalize_executable_name(exe, system)
        names.append(exe)

    return names


def main():
    script_dir = Path(__file__).resolve().parent
    pid_file = script_dir / ".pids"
    system = platform.system().lower()

    print("Performing process cleanup...")

    # Load executable names adjusted for platform
    exe_names = load_service_exec_names(script_dir, system)

    # Kill by PID
    if pid_file.exists():
        with open(pid_file, "r") as f:
            pids = [line.strip() for line in f.readlines() if line.strip()]

        for pid_str in pids:
            try:
                pid = int(pid_str)
            except ValueError:
                continue

            print(f"Killing PID {pid}")

            try:
                if system == "windows":
                    subprocess.call(
                        ["taskkill", "/PID", str(pid), "/F"],
                        stdout=subprocess.DEVNULL,
                        stderr=subprocess.DEVNULL
                    )
                else:
                    os.kill(pid, signal.SIGKILL)
            except Exception:
                pass

        pid_file.unlink()

    # Fallback name-based cleanup
    print("Running fallback cleanup...")
    for exe in exe_names:
        kill_by_name(exe, system)

    print("Cleanup complete.")


if __name__ == "__main__":
    main()
