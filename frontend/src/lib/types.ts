export type BadgeVariant = 'green' | 'amber' | 'gray';

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
