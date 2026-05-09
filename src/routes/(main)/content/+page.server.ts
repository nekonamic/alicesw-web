import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { searchContent } from "fts-native";
import { PAGE_SIZE } from "$env/static/private";

export const load: PageServerLoad = async ({ url }) => {
	const kw = url.searchParams.get("kw")?.trim();
	const pageStr = url.searchParams.get("page")?.trim();

	if (!kw || !pageStr) {
		throw redirect(307, "/");
	}

	const page = parseInt(pageStr, 10);
	if (Number.isNaN(page) || page < 1) {
		throw redirect(307, "/");
	}

	const result = await searchContent(kw, page);

	if (page * Number(PAGE_SIZE) - result.total > Number(PAGE_SIZE)) {
		throw redirect(307, "/");
	}

	return { result };
};
