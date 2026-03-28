CAPS = [
    "shell",
    "cmd",
    "screenshot",
    "keylog_start",
    "keylog_stop",
    "download",
    "upload",
]

def caps_string() -> str:
    return ";".join(CAPS)