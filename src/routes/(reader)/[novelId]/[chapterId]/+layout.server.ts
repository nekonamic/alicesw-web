import type { LayoutServerLoad } from "./$types";
import { getChapterInfoList } from "$lib/server/db";

export const load: LayoutServerLoad = async ({ params }) => {
	const idStr = params.novelId;
	const id = parseInt(idStr, 10);
	if (Number.isNaN(id) || id < 1) {
		return null;
	}
	const result = getChapterInfoList(Number(id));
	return { result };
};
