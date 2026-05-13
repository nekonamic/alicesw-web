import type { LayoutServerLoad } from "./$types";
import { getChapterInfoList } from "$lib/server/db";
import { redirect } from "@sveltejs/kit";

export const load: LayoutServerLoad = async ({ params }) => {
    const id = parseInt(params.novelId, 10);

    if (isNaN(id) || id < 1) {
        throw redirect(307, "/");
    }

    const result = getChapterInfoList(id);
	if (result == null) {
		throw redirect(307, "/");
	}
    return { result };
};