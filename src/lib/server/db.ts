import Database from "better-sqlite3";
import { PAGE_SIZE } from "$env/static/private";
const pageSize = Number(PAGE_SIZE);

const db = new Database("./data.db", {
  readonly: true,
  fileMustExist: true,
});
db.pragma("journal_mode = OFF");
db.pragma("query_only = true");
db.pragma("cache_size = -1024000");
db.pragma("temp_store = MEMORY");

export interface SearchResult {
  author: string;
  novel_id: number;
  novel_title: string;
  chapter_count: number;
  word_count: number;
}

export interface ChapterInfo {
  id: string;
  chapter_index: number;
  title: string;
  word_count: number;
}

export interface NovelResult extends SearchResult {
  chapter_info: ChapterInfo[];
}

export interface ChapterResult {
  title: string;
  content: string;
  chapter_index: number;
  total_chapters: number;
  prev_id: number;
  next_id: number;
}

export interface RandomResult {
  author: string;
  id: string;
  title: number;
}

const getNovelStmt = db.prepare(`
  SELECT
    n.author,
    n.id AS novel_id,
    n.title AS novel_title,
    COUNT(c.id) AS chapter_count,
    COALESCE(SUM(LENGTH(c.content)), 0) AS word_count
  FROM novels n
  LEFT JOIN chapters c ON n.id = c.novel_id
  WHERE n.id = :id
  GROUP BY n.id
`);

const getChapterListStmt = db.prepare(`
  SELECT id, chapter_index, title, LENGTH(content) AS word_count
  FROM chapters
  WHERE novel_id = :id
  ORDER BY chapter_index ASC
`);

const getChapterStmt = db.prepare(`
  SELECT *
  FROM (
    SELECT
      id,
      title,
      content,
      chapter_index,
      LAG(id) OVER (PARTITION BY novel_id ORDER BY chapter_index) AS prev_id,
      LEAD(id) OVER (PARTITION BY novel_id ORDER BY chapter_index) AS next_id,
      COUNT(*) OVER (PARTITION BY novel_id) AS total_chapters
    FROM chapters
    WHERE novel_id = :novelId
  ) t
  WHERE id = :id;
`);

const getRandomNovelStmt = db.prepare(
  `
  SELECT author, id, title FROM novels ORDER BY RANDOM() LIMIT :pageSize;
  `,
);

export function getNovel(id: number): NovelResult | null {
  const novel = getNovelStmt.get({ id }) as SearchResult | undefined;
  if (!novel) return null;

  const chapters = getChapterListStmt.all({ id }) as ChapterInfo[];

  return { ...novel, chapter_info: chapters };
}

export function getChapter(id: string, novelId: number): ChapterResult | null {
  return (
    (getChapterStmt.get({ id, novelId }) as ChapterResult | undefined) ?? null
  );
}

export function getRandomNovel(): RandomResult[] | null {
  return (
    (getRandomNovelStmt.all({ pageSize }) as RandomResult[] | undefined) ?? null
  );
}
