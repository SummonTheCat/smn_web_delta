#!/usr/bin/env python3
import json
import subprocess
import shutil
import os
from pathlib import Path


def run(cmd, cwd):
    print(f"[RUN] {cmd} (cwd={cwd})")
    subprocess.check_call(cmd, cwd=str(cwd), shell=True)


def copy_item(src: Path, dst: Path):
    if not src.exists():
        print(f"  [WARN] Missing: {src}")
        return

    if src.is_dir():
        shutil.copytree(src, dst / src.name, dirs_exist_ok=True)
    else:
        dst.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dst / src.name)


def build_project(proj_path: Path, copy_list, output_root: Path):
    proj_name = proj_path.name
    print(f"\n=== Building {proj_name} ===")

    run("cargo build --release", cwd=proj_path)

    target_release = proj_path / "target" / "release"
    out_dir = output_root / proj_name
    out_dir.mkdir(parents=True, exist_ok=True)

    exe_name = proj_name + ".exe"
    exe_path = target_release / exe_name

    if not exe_path.exists():
        candidates = list(target_release.glob("*.exe"))
        if not candidates:
            raise FileNotFoundError(f"No .exe produced for project: {proj_name}")
        exe_path = max(candidates, key=lambda p: p.stat().st_mtime)

    print(f"  Copying binary: {exe_path} -> {out_dir}")
    shutil.copy2(exe_path, out_dir / exe_path.name)

    for item in copy_list:
        src = proj_path / item
        print(f"  Copying extra: {src}")
        copy_item(src, out_dir)

    print(f"=== Done {proj_name} ===\n")


def main():
    script_dir = Path(__file__).resolve().parent
    config_file = script_dir / "build_config.json"
    if not config_file.exists():
        raise FileNotFoundError(f"Config file not found: {config_file}")

    with open(config_file, "r") as f:
        config = json.load(f)

    # IMPORTANT: output bin is relative to current working directory
    output_root = Path(config.get("output_bin", "bin")).resolve()
    output_root.mkdir(parents=True, exist_ok=True)

    for proj in config["projects"]:
        proj_path = Path(proj["path"]).resolve()
        copy_list = proj.get("copy", [])
        build_project(proj_path, copy_list, output_root)


if __name__ == "__main__":
    main()
