import type { PageServerLoad } from "./$types";
import { getRandomNovel } from "$lib/server/db";

export const load: PageServerLoad = async () => {
  const result = getRandomNovel();

  return { result };
};
