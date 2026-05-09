import { getNovel } from "$lib/server/db";
import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
	const idStr = params.novelId?.trim();

	if (!idStr) {
		throw redirect(307, "/");
	}
	const id = parseInt(idStr, 10);
	if (Number.isNaN(id) || id < 1) {
		throw redirect(307, "/");
	}

	const result = getNovel(id);

	if (result == null) {
		throw redirect(307, "/");
	}

	return result;
};
