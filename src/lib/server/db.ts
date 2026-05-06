import Database from 'better-sqlite3';

const db = new Database('./alicesw.db');

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
}

const searchAuthorStmt = db.prepare<[string, number, number]>(`
  SELECT 
    n.author,
    n.id AS novel_id, 
    n.title AS novel_title, 
    (MAX(c.chapter_index) + 1) AS chapter_count,
    SUM(LENGTH(c.content)) AS word_count
  FROM novels n
  LEFT JOIN chapters c ON n.id = c.novel_id
  WHERE n.author LIKE ?
  GROUP BY n.id, n.title
  ORDER BY n.id ASC
  LIMIT ? OFFSET ?
`);

const searchTitleStmt = db.prepare<[string, number, number]>(`
  SELECT 
    n.author,
    n.id AS novel_id, 
    n.title AS novel_title, 
    (MAX(c.chapter_index) + 1) AS chapter_count,
    SUM(LENGTH(c.content)) AS word_count
  FROM novels n
  LEFT JOIN chapters c ON n.id = c.novel_id
  WHERE n.title LIKE ?
  GROUP BY n.id, n.title
  ORDER BY n.id ASC
  LIMIT ? OFFSET ?
`);

const getNovelStmt = db.prepare<[number]>(`
  SELECT 
    n.author, 
    n.id AS novel_id, 
    n.title AS novel_title,
    COUNT(c.id) AS chapter_count,
    COALESCE(SUM(LENGTH(c.content)), 0) AS word_count
  FROM novels n
  LEFT JOIN chapters c ON n.id = c.novel_id
  WHERE n.id = ?
  GROUP BY n.id
`);

const getChapterListStmt = db.prepare<[number]>(`
  SELECT id, chapter_index, title, LENGTH(content) AS word_count
  FROM chapters 
  WHERE novel_id = ?
  ORDER BY chapter_index ASC
`);

const getChapterStmt = db.prepare<[string, number]>(`
  SELECT title, content
  FROM chapters
  WHERE id = ? AND novel_id = ?
`);

export function searchAuthor(keyword: string, page: number): SearchResult[] {
  return searchAuthorStmt.all(`%${keyword}%`, 20, page * 20) as SearchResult[];
}

export function searchTitle(keyword: string, page: number): SearchResult[] {
  return searchTitleStmt.all(`%${keyword}%`, 20, page * 20) as SearchResult[];
}

export function getNovel(id: number): NovelResult | null {
  const novel = getNovelStmt.get(id) as SearchResult | undefined;
  if (!novel) return null;

  const chapters = getChapterListStmt.all(id) as ChapterInfo[];

  return { ...novel, chapter_info: chapters };
}

export function getChapter(id: string, novelId: number): ChapterResult | null {
  return (getChapterStmt.get(id, novelId) as ChapterResult | undefined) ?? null;
}