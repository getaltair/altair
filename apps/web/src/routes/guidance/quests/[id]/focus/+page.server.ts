import type { ServerLoadEvent } from '@sveltejs/kit';

export const load = async ({ params }: ServerLoadEvent) => {
  return { id: params.id as string };
};
