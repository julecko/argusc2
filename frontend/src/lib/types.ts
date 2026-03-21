// ── Notifications ─────────────────────────────────────────────────────────────

export type NotifType = 'device' | 'upload' | 'system';

export interface Notification {
    id: number;
    type: NotifType;
    title: string;
    text: string;
    time: string;
    unread: boolean;
}

// ── DataTable ────────────────────────────────────────────────────────────────

export type BadgeVariant = 'green' | 'amber' | 'gray' | 'red' | 'blue';

export interface Column {
    key: string;
    label: string;
    width?: string;
}

export type Row = Record<string, unknown>;

export interface DataTableProps {
    columns: Column[];
    rows: Row[];
    statusKey?: string;
    statusColors?: Record<string, BadgeVariant>;
}