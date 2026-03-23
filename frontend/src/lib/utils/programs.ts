export function downloadsDisplay(downloads: number, allowed: number): string {
    if (allowed === 0 && downloads === 0) return '⛔ forbidden';
    if (allowed === 0 && downloads > 0)   return `${downloads} / ${downloads}`;
    if (allowed === -1)                   return `${downloads} / ∞`;
    const total = downloads + allowed;
    return `${downloads} / ${total}`;
}

export function formatBytes(bytes: number): string {
    if (bytes < 1024)       return `${bytes} B`;
    if (bytes < 1024 ** 2)  return `${(bytes / 1024).toFixed(2)} KB`;
    if (bytes < 1024 ** 3)  return `${(bytes / 1024 ** 2).toFixed(2)} MB`;
    return `${(bytes / 1024 ** 3).toFixed(2)} GB`;
}

export function formatDate(iso: string): string {
    return new Date(iso).toLocaleString('en-US', {
        year: 'numeric', month: 'short', day: 'numeric',
        hour: '2-digit', minute: '2-digit',
    });
}