import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { searchAuthor } from "fts-native";
import { PAGE_SIZE } from "$env/static/private";
const pageSize = Number(PAGE_SIZE);

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

  const result = await searchAuthor(kw, page);

  if (page * pageSize - result.total > pageSize) {
    throw redirect(307, "/");
  }

  return { result };
};
