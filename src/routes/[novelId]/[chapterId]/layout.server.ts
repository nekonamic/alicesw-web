import { getChapterInfoList } from "$lib/server/db";
import { page } from "$app/state";

export function load() {
  const id = page.params.novelId;
  if (!id || !isNaN(Number(id))) {
    return null;
  }
  const result = getChapterInfoList(Number(id));
  return { result };
}
