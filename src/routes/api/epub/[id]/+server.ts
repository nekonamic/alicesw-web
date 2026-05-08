import { type RequestHandler, redirect } from "@sveltejs/kit";
import { getDownloadNovel } from "$lib/server/db";
import { generateEpub } from "fts-native";

export const GET: RequestHandler = async ({ params }) => {
  const idStr = params.id?.trim();

  if (!idStr) {
    throw redirect(307, "/");
  }

  const id = parseInt(idStr, 10);

  if (Number.isNaN(id) || id < 1) {
    throw redirect(307, "/");
  }

  const novel = getDownloadNovel(id)!;

  const buffer = generateEpub(novel);

  return new Response(new Uint8Array(buffer), {
    headers: {
      "Content-Type": "application/epub+zip",
      "Content-Disposition": `attachment; filename*=UTF-8''${encodeURIComponent(
        novel.title,
      )}.epub`,
      "Content-Length": buffer.length.toString(),
    },
  });
};
