import { getChapter } from "$lib/server/db";
import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
	const novelIdStr = params.novelId?.trim();
	const chapterId = params.chapterId?.trim();

	if (!chapterId || !novelIdStr) {
		throw redirect(307, "/");
	}
	const novelId = parseInt(novelIdStr, 10);
	if (Number.isNaN(novelId) || novelId < 1) {
		throw redirect(307, "/");
	}

	const result = getChapter(chapterId, novelId);

	if (result == null) {
		throw redirect(307, "/");
	}

	return result;
};
