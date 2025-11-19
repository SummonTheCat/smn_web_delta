#!/usr/bin/env python3
import subprocess
import sys

def run(cmd):
    result = subprocess.run(cmd, shell=True)
    if result.returncode != 0:
        print(f"Command failed: {cmd}")
        sys.exit(result.returncode)

def main():
    print("This will reset the repository to match origin completely.")
    print("Local changes and untracked files will be LOST.")
    confirm = input("Continue? (yes/no): ").strip().lower()

    if confirm not in ("yes", "y"):
        print("Aborted.")
        return

    # Fetch latest changes
    print("\nFetching latest from origin...")
    run("git fetch --all --prune")

    # Determine current branch
    print("Detecting current branch...")
    branch = subprocess.check_output(
        "git rev-parse --abbrev-ref HEAD", shell=True
    ).decode().strip()

    print(f"Resetting to origin/{branch} ...")
    run(f"git reset --hard origin/{branch}")

    print("Cleaning untracked files...")
    run("git clean -fdx")

    print("\nRepository successfully updated and cleaned.")

if __name__ == "__main__":
    main()
