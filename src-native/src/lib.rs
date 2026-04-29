#![deny(clippy::all)]

use napi_derive::napi;
use rusqlite::Connection;
use std::path::Path;
use tantivy::{Index, TantivyDocument, collector::TopDocs, directory::MmapDirectory, query::QueryParser, schema::{Value, INDEXED, IndexRecordOption, STORED, STRING, Schema, TextFieldIndexing, TextOptions}, tokenizer::{LowerCaser, RemoveLongFilter, Stemmer, TextAnalyzer}};

#[napi(object)]
pub struct ContentSearchResult {
  pub author: String,
  pub novel_id: u32,
  pub novel_title: String,
  pub chapter_id: String,
  pub chapter_title: String,
  pub chapter_index: u32,
}

#[napi]
pub fn search_content(keyword: String, page: u32) -> Vec<ContentSearchResult> {
  let mut schema_builder = Schema::builder();
  // let id = schema_builder.add_text_field("id", STRING | STORED);
  let author = schema_builder.add_text_field(
    "author",
    TextOptions::default()
      .set_indexing_options(
        TextFieldIndexing::default()
          .set_tokenizer("jieba")
          .set_index_option(IndexRecordOption::WithFreqsAndPositions),
      )
      .set_stored(),
  );
  let novel_id = schema_builder.add_u64_field("novel_id", INDEXED | STORED);
  let novel_title = schema_builder.add_text_field(
    "novel_title",
    TextOptions::default()
      .set_indexing_options(
        TextFieldIndexing::default()
          .set_tokenizer("jieba")
          .set_index_option(IndexRecordOption::WithFreqsAndPositions),
      )
      .set_stored(),
  );
  let chapter_id = schema_builder.add_text_field("chapter_id", STRING | STORED);
  let chapter_title = schema_builder.add_text_field(
    "chapter_title",
    TextOptions::default()
      .set_indexing_options(
        TextFieldIndexing::default()
          .set_tokenizer("jieba")
          .set_index_option(IndexRecordOption::WithFreqsAndPositions),
      )
      .set_stored(),
  );
  let chapter_index = schema_builder.add_u64_field("chapter_index", INDEXED | STORED);
  let content = schema_builder.add_text_field(
    "content",
    TextOptions::default()
      .set_indexing_options(
        TextFieldIndexing::default()
          .set_tokenizer("jieba")
          .set_index_option(IndexRecordOption::WithFreqsAndPositions),
      )
      .set_stored(),
  );
  let schema = schema_builder.build();

  let tokenizer = tantivy_jieba::JiebaTokenizer::new();
  let index_path = Path::new("./index");
  let dir = MmapDirectory::open(index_path).unwrap();
  let index = Index::open_or_create(dir, schema).unwrap();
  let analyzer = TextAnalyzer::builder(tokenizer)
    .filter(RemoveLongFilter::limit(40))
    .filter(LowerCaser)
    .filter(Stemmer::default())
    .build();
  index.tokenizers().register("jieba", analyzer);

  let reader = index.reader().unwrap();
  let searcher = reader.searcher();
  let query_parser = QueryParser::for_index(&index, vec![content]);
  let query = query_parser.parse_query(&keyword).unwrap();

  let top_docs = searcher
    .search(&query, &TopDocs::with_limit(20).and_offset((page * 20).try_into().unwrap()).order_by_score())
    .unwrap();
  let mut results: Vec<ContentSearchResult> = Vec::new();
  for (_, doc_address) in top_docs {
    let retrieved_doc: TantivyDocument = searcher.doc(doc_address).unwrap();
    results.push(ContentSearchResult {
        author: retrieved_doc.get_first(author).unwrap().as_str().unwrap_or_default().to_string(),
        novel_id: retrieved_doc.get_first(novel_id).unwrap().as_u64().unwrap_or_default() as u32,
        novel_title: retrieved_doc.get_first(novel_title).unwrap().as_str().unwrap_or_default().to_string(),
        chapter_id: retrieved_doc.get_first(chapter_id).unwrap().as_str().unwrap_or_default().to_string(),
        chapter_title: retrieved_doc.get_first(chapter_title).unwrap().as_str().unwrap_or_default().to_string(),
        chapter_index: retrieved_doc.get_first(chapter_index).unwrap().as_u64().unwrap_or_default() as u32,
    });
  }
  results
}

#[napi(object)]
pub struct AuthorSearchResult {
  pub author: String,
  pub novel_id: u32,
  pub novel_title: String,
  pub chapter_count: u32,
  pub word_count: u32,
}

#[napi]
pub fn search_author(keyword: String, page: u32) -> Vec<AuthorSearchResult> {
    let conn = Connection::open("./alicesw.db").unwrap();
    let mut stmt = conn.prepare(
        "SELECT 
            n.author,
            n.id AS novel_id, 
            n.title AS novel_title, 
            (MAX(c.chapter_index) + 1) AS next_chapter_index,
            SUM(LENGTH(c.content)) AS total_content_length
        FROM novels n
        LEFT JOIN chapters c ON n.id = c.novel_id
        WHERE n.author LIKE ?1
        GROUP BY n.id, n.title
        ORDER BY n.id ASC
        LIMIT ?2 OFFSET ?3"
    ).unwrap();

    let mut results: Vec<AuthorSearchResult> = Vec::new();

    let result_iter = stmt.query_map([format!("%{}%", keyword), (page*20).to_string(), ((page+1)*20).to_string()], |row| {
        Ok(AuthorSearchResult {
            author: row.get(0)?,
            novel_id: row.get(1)?,
            novel_title: row.get(2)?,
            chapter_count: row.get(3)?,
            word_count: row.get(4)?,
        })
    }).unwrap();

    for item in result_iter {
        results.push(item.unwrap());
    }

    results
}

#[napi(object)]
pub struct NovelSearchResult {
  pub author: String,
  pub novel_id: u32,
  pub novel_title: String,
  pub chapter_count: u32,
  pub word_count: u32,
}

#[napi]
pub fn search_novel(keyword: String, page: u32) -> Vec<NovelSearchResult> {
    let conn = Connection::open("./alicesw.db").unwrap();
    let mut stmt = conn.prepare(
        "SELECT 
            n.author,
            n.id AS novel_id, 
            n.title AS novel_title, 
            (MAX(c.chapter_index) + 1) AS next_chapter_index,
            SUM(LENGTH(c.content)) AS total_content_length
        FROM novels n
        LEFT JOIN chapters c ON n.id = c.novel_id
        WHERE n.title LIKE ?1
        GROUP BY n.id, n.title
        ORDER BY n.id ASC
        LIMIT ?2 OFFSET ?3"
    ).unwrap();

    let mut results: Vec<NovelSearchResult> = Vec::new();

    let result_iter = stmt.query_map([format!("%{}%", keyword), (page*20).to_string(), ((page+1)*20).to_string()], |row| {
        Ok(NovelSearchResult {
            author: row.get(0)?,
            novel_id: row.get(1)?,
            novel_title: row.get(2)?,
            chapter_count: row.get(3)?,
            word_count: row.get(4)?,
        })
    }).unwrap();

    for item in result_iter {
        results.push(item.unwrap());
    }

    results
}

#[napi(object)] 
pub struct ChapterInfo {
  pub id: String,
  pub chapter_index: u32,
  pub title: String,
  pub word_count: u32,
}

#[napi(object)]
pub struct NovelResult {
  pub author: String,
  pub novel_id: u32,
  pub novel_title: String,
  pub chapter_count: u32,
  pub word_count: u32,
  pub chapter_info: Vec<ChapterInfo>,
}

#[napi]
pub fn get_novel(id: u32) -> Option<NovelResult> {
    let conn = Connection::open("./alicesw.db").unwrap();
    let mut stmt = conn.prepare(
        "SELECT 
              n.author, 
              n.id, 
              n.title,
              COUNT(c.id) as chapter_count,
              COALESCE(SUM(LENGTH(c.content)), 0) as total_words
          FROM novels n
          LEFT JOIN chapters c ON n.id = c.novel_id
          WHERE n.id = ?1
          GROUP BY n.id"
    ).unwrap();

    let novel_base = stmt.query_row([id], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, u32>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, u32>(3)?,
            row.get::<_, u32>(4)?,
        ))
    }).unwrap();

    let mut chapter_stmt = conn.prepare(
        "SELECT id, chapter_index, title, LENGTH(content) 
         FROM chapters 
         WHERE novel_id = ?1 
         ORDER BY chapter_index ASC"
    ).unwrap();

    let chapter_iter = chapter_stmt.query_map([id], |row| {
        Ok(ChapterInfo {
            id: row.get(0)?,
            chapter_index: row.get(1)?,
            title: row.get(2)?,
            word_count: row.get(3)?,
        })
    }).unwrap();

    let mut chapters = Vec::new();
    for chapter in chapter_iter {
        if let Ok(c) = chapter {
            chapters.push(c);
        }
    }

    Some(NovelResult {
        author: novel_base.0,
        novel_id: novel_base.1,
        novel_title: novel_base.2,
        chapter_count: novel_base.3,
        word_count: novel_base.4,
        chapter_info: chapters,
    })
}

#[napi(object)] 
pub struct ChapterResult {
  pub title: String,
  pub content: String,
}

#[napi]
pub fn get_chapter(id: String, novel_id: u32) -> Option<ChapterResult> {
    let conn = Connection::open("./alicesw.db").unwrap();
    let mut stmt = conn.prepare(
        "SELECT  
            title, 
            content,
          FROM chapters
          WHERE id = ?1
            AND novel_id = ?2"
    ).unwrap();

    let chapter = stmt.query_row([id, novel_id.to_string()], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
        ))
    }).unwrap();

    Some(ChapterResult {
        title: chapter.0,
        content: chapter.1,
    })
}