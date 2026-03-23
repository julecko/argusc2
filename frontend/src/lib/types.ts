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

// ── Programs ──────────────────────────────────────────────────────────────────

// ── Program types (from /api/types/program-types) ─────────────────────────────
export interface ProgramType {
	id:    number;
	name:  string;
	color: string; // hex e.g. "#d62728"
}

// ── ProgramCapabilities ────────────────────────────────────────────────────────────────

export interface Capability {
    id: number;
    name: string;
    description: string;
}

// ── Program OS ────────────────────────────────────────────────────────────────
export type ProgramOS = 'windows' | 'linux' | 'android' | 'mac';

// ── Program (flat row from /api/programs list) ────────────────────────────────
export interface Program {
	id:                number;
	type_id:           number;
	type_name:         string | null;
	type_color:        string | null;
	uploaded_by:       number | null;
	uploader_name:     string | null;
	name:              string;
	original_name:     string;
	version:           string;
	os:                ProgramOS;
	storage_path:      string;
	filesize:          number;
	file_hash:         string;
	ws_key:            string;
	description:       string | null;
	downloads:         number;
	allowed_downloads: number; // -1 = unlimited, 0 = forbidden, >0 = fixed limit
	created_at:        string;
	updated_at:        string;
}

export interface ProgramDetail extends Program {
	capabilities: Array<{ id: number; name: string }>;
}
