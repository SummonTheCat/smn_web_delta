#!/usr/bin/env python3
import json
import subprocess
from pathlib import Path


HTTP_TEMPLATE = """# ===============================
# {baseAddress} & subdomains
# ===============================

# --- Redirect all HTTP to HTTPS ---
server {{
    listen 80;
    listen [::]:80;
    server_name {all_domains};
    return 301 https://$host$request_uri;
}}
"""

HTTPS_TEMPLATE = """# --- HTTPS Server Block ---
server {{
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name {all_domains};

    # --- SSL Configuration ---
    ssl_certificate {ssl_certificate};
    ssl_certificate_key {ssl_certificate_key};
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

    location / {{
{rules}

        # --- Default fallback (optional) ---
        return 404;
    }}

    # --- Proxy Headers ---
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
}}
"""


def run(cmd: str):
    print(f"[RUN] {cmd}")
    subprocess.check_call(cmd, shell=True)


def build_rule(subdomain: str, port: int, base: str) -> str:
    if subdomain == "":
        fqdn = base
        www_fqdn = f"www.{base}"

        return f"""        # --- {fqdn} traffic to port {port} ---
        if ($host = "{fqdn}") {{
            proxy_pass http://127.0.0.1:{port};
            break;
        }}

        if ($host = "{www_fqdn}") {{
            proxy_pass http://127.0.0.1:{port};
            break;
        }}"""

    else:
        fqdn = f"{subdomain}.{base}"
        return f"""        # --- {fqdn} traffic to port {port} ---
        if ($host = "{fqdn}") {{
            proxy_pass http://127.0.0.1:{port};
            break;
        }}"""


def acquire_certificate(domains):
    print("\n========================================")
    print(" Certbot SSL Certificate Acquisition")
    print("========================================\n")

    print("Certbot will now request SSL certificates for:")
    for d in domains:
        print(f"  - {d}")

    print("""
NOTE: You will be prompted by certbot for:
  • An email address
  • Agreement to Let's Encrypt Terms of Service
  • Optional marketing emails
  • HTTP validation confirmation

This is normal and required for certificate issuance.
""")

    domain_args = " ".join([f"-d {d}" for d in domains])

    cmd = f"certbot certonly --nginx {domain_args}"
    print("[INFO] Running Certbot…")
    run(cmd)

    print("[INFO] Certificate acquisition complete.\n")


def main():
    script_dir = Path(__file__).resolve().parent
    config_path = script_dir / "nginx_config.json"

    if not config_path.exists():
        raise FileNotFoundError(f"Missing nginx_config.json at {config_path}")

    with open(config_path, "r") as f:
        cfg = json.load(f)

    base = cfg["baseAddress"]
    cert_path = cfg["sslCertificatePath"]
    key_path = cfg["sslCertificateKeyPath"]
    out_path = Path(cfg["nginxConfigPath"])

    # Build hostname list
    domains = []
    for s in cfg["servers"]:
        sub = s["subDomain"]
        if sub == "":
            domains.append(base)
            domains.append("www." + base)
        else:
            domains.append(f"{sub}.{base}")

    # STEP 1 — Acquire certificate before generating config
    acquire_certificate(domains)

    # STEP 2 — Build nginx config
    all_domains = " ".join(domains)

    rule_blocks = []
    for s in cfg["servers"]:
        rule_blocks.append(build_rule(s["subDomain"], s["port"], base))

    rules_str = "\n\n".join(rule_blocks)

    http_block = HTTP_TEMPLATE.format(
        baseAddress=base,
        all_domains=all_domains
    )

    https_block = HTTPS_TEMPLATE.format(
        all_domains=all_domains,
        ssl_certificate=cert_path,
        ssl_certificate_key=key_path,
        rules=rules_str
    )

    final_config = http_block + "\n" + https_block

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(final_config, encoding="utf-8")

    print(f"Nginx configuration written to: {out_path}")
    print("Reloading nginx…")
    run("systemctl reload nginx")

    print("\nAll done.")


if __name__ == "__main__":
    main()
