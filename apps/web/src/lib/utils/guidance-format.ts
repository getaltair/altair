import type { InitiativeStatus } from '$lib/types/core.js';
import type {
	QuestStatus,
	Priority,
	RoutineStatus,
	RoutineFrequency,
	EpicStatus
} from '$lib/types/guidance.js';

/**
 * Map any entity status to Tailwind badge classes (bg + text).
 * Superset covering Initiative, Quest, Epic, and Routine statuses.
 */
export function statusColor(
	status: InitiativeStatus | QuestStatus | EpicStatus | RoutineStatus
): string {
	switch (status) {
		case 'active':
		case 'in_progress':
			return 'bg-[#a8c5a0]/30 text-[#5a8a52]';
		case 'completed':
			return 'bg-[#c7e7fa]/50 text-[#446273]';
		case 'paused':
		case 'pending':
			return 'bg-[#f0e6c8]/50 text-[#8a6a2a]';
		case 'archived':
		case 'cancelled':
			return 'bg-[#e8eef0] text-[#566162]';
		default:
			return 'bg-surface-low text-on-surface-muted';
	}
}

export function priorityColor(p: Priority | null): string {
	switch (p) {
		case 'critical':
		case 'high':
			return 'text-warm-terracotta';
		case 'medium':
			return 'text-morning-gold';
		case 'low':
			return 'text-sage-whisper';
		default:
			return 'text-on-surface-muted';
	}
}

export function priorityDotColor(p: Priority): string {
	switch (p) {
		case 'critical':
		case 'high':
			return 'bg-warm-terracotta';
		case 'medium':
			return 'bg-morning-gold';
		case 'low':
			return 'bg-sage-whisper';
		default:
			return 'bg-outline-variant';
	}
}

export function priorityLabel(p: Priority | null): string {
	if (!p) return '';
	return p.charAt(0).toUpperCase() + p.slice(1);
}

export function frequencyLabel(freq: RoutineFrequency): string {
	switch (freq) {
		case 'biweekly':
			return 'Biweekly';
		default:
			return freq.charAt(0).toUpperCase() + freq.slice(1);
	}
}

export function formatDate(dateStr: string): string {
	const d = new Date(dateStr);
	return d.toLocaleDateString('en-US', {
		month: 'short',
		day: 'numeric',
		year: 'numeric'
	});
}

export function formatDuration(minutes: number | null): string {
	if (!minutes) return '--';
	if (minutes < 60) return `${minutes}m`;
	const h = Math.floor(minutes / 60);
	const m = minutes % 60;
	return m > 0 ? `${h}h ${m}m` : `${h}h`;
}
