export function formatThroughput(value: number): string {
  if (value >= 1_000) {
    return `${(value / 1_000).toFixed(1)} Gbps`;
  }

  return `${value.toFixed(1)} Mbps`;
}

export function formatPercent(value: number): string {
  return `${value.toFixed(1)}%`;
}

export function formatTimestamp(value: string | null): string {
  if (!value) {
    return 'never';
  }

  return new Date(value).toLocaleTimeString('en-US', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  });
}

export function statusColor(status: string): string {
  if (status === 'online' || status === 'info') {
    return 'success';
  }

  if (status === 'degraded' || status === 'warn') {
    return 'warning';
  }

  if (status === 'offline' || status === 'error') {
    return 'error';
  }

  return 'mutedForeground';
}
