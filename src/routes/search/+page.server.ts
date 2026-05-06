import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { searchContent } from "../../../src-native";
import { searchTitle, searchAuthor } from "$lib/server/db";

const ALLOWED_TARGETS = ['content', 'title', 'author'] as const;
type SearchTarget = typeof ALLOWED_TARGETS[number];

export const load: PageServerLoad = async ({ url }) => {
    const target = url.searchParams.get('target')?.trim();
    const kw = url.searchParams.get('kw')?.trim();
    const pageStr = url.searchParams.get('page')?.trim();

    if (!kw || !target || !pageStr) {
        throw redirect(307, '/');
    }

    const page = parseInt(pageStr, 10);
    if (Number.isNaN(page) || page < 1) {
        throw redirect(307, '/');
    }

    if (!ALLOWED_TARGETS.includes(target as SearchTarget)) {
        throw redirect(307, '/');
    }
    let result;

    switch (target as SearchTarget) {
        case 'content':
            result = searchContent(kw, page);
            break;
        case 'title':
            result = searchTitle(kw, page);
            break;
        case 'author':
            result = searchAuthor(kw, page);
            break;
    }

    return { result };
};