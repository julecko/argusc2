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

// ── ProgramCapabilities ────────────────────────────────────────────────────────────────

export interface Capability {
    id: string;
    label: string;
    desc: string;
}

// ── Programs ──────────────────────────────────────────────────────────────────

export type ProgramOS = 'windows' | 'linux' | 'android' | 'mac';

export interface ProgramType {
    id: number;
    name: string;
    color: string; // hex color
}

export interface Program {
    id: number;
    type_id: number;
    type_name: string | null;   // joined from program_types
    type_color: string | null;   // joined from program_types
    uploaded_by: number | null;
    uploader_name: string | null;   // joined from admins
    name: string;
    original_name: string;
    version: string;
    os: ProgramOS;
    storage_path: string;
    filesize: number;
    file_hash: string;
    ws_key: string;
    description: string | null;
    downloads: number;
    allowed_downloads: number;
    created_at: string;
    updated_at: string;
}
