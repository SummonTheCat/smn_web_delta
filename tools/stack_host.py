#!/usr/bin/env python3
import json
import subprocess
import os
from pathlib import Path
import platform

def resolve_exec_path(raw_path: Path) -> Path:
    system = platform.system().lower()

    if system != "windows" and raw_path.suffix == ".exe":
        return raw_path.with_suffix("")

    if system == "windows" and raw_path.suffix != ".exe":
        return raw_path.with_suffix(".exe")

    return raw_path


def main():
    script_dir = Path(__file__).resolve().parent
    config_file = script_dir / "host_stack.json"

    if not config_file.exists():
        raise FileNotFoundError(f"Missing host_stack.json at {config_file}")

    with open(config_file, "r") as f:
        config = json.load(f)

    pid_file = script_dir / ".pids"
    if pid_file.exists():
        pid_file.unlink()

    system = platform.system().lower()
    processes = []

    for svc in config["services"]:
        name = svc["name"]

        # service path relative to current running directory
        raw_path = Path(svc["path"]).resolve()
        exec_path = resolve_exec_path(raw_path)

        print(f"Starting {name} -> {exec_path}")

        if not exec_path.exists():
            print(f"[WARN] Executable missing: {exec_path}")
            continue

        if system == "windows":
            proc = subprocess.Popen(
                [str(exec_path)],
                cwd=exec_path.parent,
                creationflags=subprocess.CREATE_NEW_PROCESS_GROUP,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL
            )
            processes.append(proc)

        else:
            proc = subprocess.Popen(
                [str(exec_path)],
                cwd=exec_path.parent,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL
            )

        processes.append(proc)

    # Write correct PIDs
    with open(pid_file, "w") as f:
        for p in processes:
            f.write(str(p.pid) + "\n")

    print("All services started.")


if __name__ == "__main__":
    main()
