export interface Props {
	title: string | null;
	description?: string;
	cover?: string;
	metas?: Record<string, string>[];
	metaid?: string;
}
