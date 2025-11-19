#!/usr/bin/env python3
import subprocess
import sys
from pathlib import Path


TOOLS = Path(__file__).resolve().parent


def run_script(path: Path):
    print(f"\n========================================")
    print(f" Running: {path.name}")
    print(f"========================================\n")

    try:
        subprocess.check_call([sys.executable, str(path)])
    except subprocess.CalledProcessError as e:
        print(f"[ERROR] Script failed: {path.name}")
        sys.exit(e.returncode)


def main():
    kill_script = TOOLS / "stack_kill.py"
    update_script = TOOLS / "update.py"
    build_script = TOOLS / "build_all.py"
    start_script = TOOLS / "stack_host.py"
    nginx_script = TOOLS / "nginx_host.py"

    # 1. Kill any running services
    run_script(kill_script)

    # 2. Update repo to latest + clean
    run_script(update_script)

    # 3. Rebuild everything
    run_script(build_script)

    # 4. Start services permanently
    run_script(start_script)

    # 5. Nginx config + SSL + reload nginx
    run_script(nginx_script)

    print("\n========================================")
    print(" System is now deployed and running.")
    print("========================================\n")

    input("Press ENTER to run basic external test...")

    # 6. A simple external connectivity test (curl)
    # The domain is read from nginx_config.json
    import json
    cfg = json.loads((TOOLS / "nginx_config.json").read_text())
    domain = cfg["baseAddress"]

    print(f"[TEST] Running curl https://{domain}")
    try:
        subprocess.call(["curl", f"https://{domain}"])
    except Exception as e:
        print(f"[WARN] Curl test failed: {e}")

    print("\nFinished.")


if __name__ == "__main__":
    main()
