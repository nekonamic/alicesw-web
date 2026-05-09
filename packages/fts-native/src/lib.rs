#![deny(clippy::all)]

pub mod epub;
pub mod txt;

use napi::bindgen_prelude::{AsyncTask, Result};
use napi::{Env, Task};
use napi_derive::napi;
use once_cell::sync::Lazy;
use std::path::Path;
use tantivy::{
  collector::{Count, TopDocs},
  directory::MmapDirectory,
  query::QueryParser,
  schema::{
    Field, IndexRecordOption, Schema, TextFieldIndexing, TextOptions, Value, INDEXED, STORED,
    STRING,
  },
  tokenizer::{LowerCaser, RemoveLongFilter, Stemmer, TextAnalyzer},
  Index, IndexReader, TantivyDocument,
};

const PAGE_SIZE: usize = 24;

struct ContentFields {
  _id: Field,
  author: Field,
  novel_id: Field,
  novel_title: Field,
  chapter_id: Field,
  chapter_title: Field,
  chapter_index: Field,
  content: Field,
}

struct NovelFields {
  author: Field,
  id: Field,
  title: Field,
}

pub struct SearchContentTask {
  keyword: String,
  page: u32,
}

pub struct SearchNovelTask {
  keyword: String,
  page: u32,
  search_type: NovelSearchType,
}

pub enum NovelSearchType {
  Title,
  Author,
}

fn build_text_options() -> TextOptions {
  TextOptions::default()
    .set_indexing_options(
      TextFieldIndexing::default()
        .set_tokenizer("jieba")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions),
    )
    .set_stored()
}

fn build_content_schema() -> (Schema, ContentFields) {
  let mut builder = Schema::builder();

  let fields = ContentFields {
    _id: builder.add_text_field("id", STRING | STORED),
    author: builder.add_text_field("author", build_text_options()),
    novel_id: builder.add_u64_field("novel_id", INDEXED | STORED),
    novel_title: builder.add_text_field("novel_title", build_text_options()),
    chapter_id: builder.add_text_field("chapter_id", STRING | STORED),
    chapter_title: builder.add_text_field("chapter_title", build_text_options()),
    chapter_index: builder.add_u64_field("chapter_index", INDEXED | STORED),
    content: builder.add_text_field("content", build_text_options()),
  };

  (builder.build(), fields)
}

fn build_novel_schema() -> (Schema, NovelFields) {
  let mut builder = Schema::builder();

  let fields = NovelFields {
    id: builder.add_u64_field("id", INDEXED | STORED),
    author: builder.add_text_field("author", build_text_options()),
    title: builder.add_text_field("title", build_text_options()),
  };

  (builder.build(), fields)
}

static CONTENT: Lazy<(Index, ContentFields)> = Lazy::new(|| {
  let (schema, fields) = build_content_schema();
  let dir = MmapDirectory::open(Path::new("./content_index")).unwrap();
  let index = Index::open_or_create(dir, schema).unwrap();

  let tokenizer = tantivy_jieba::JiebaTokenizer::new();
  let analyzer = TextAnalyzer::builder(tokenizer)
    .filter(RemoveLongFilter::limit(40))
    .filter(LowerCaser)
    .filter(Stemmer::default())
    .build();

  index.tokenizers().register("jieba", analyzer);

  (index, fields)
});

static CONTENT_READER: Lazy<IndexReader> = Lazy::new(|| CONTENT.0.reader().unwrap());

static CONTENT_QUERY: Lazy<QueryParser> =
  Lazy::new(|| QueryParser::for_index(&CONTENT.0, vec![CONTENT.1.content]));

static NOVEL: Lazy<(Index, NovelFields)> = Lazy::new(|| {
  let (schema, fields) = build_novel_schema();
  let dir = MmapDirectory::open(Path::new("./novel_index")).unwrap();
  let index = Index::open_or_create(dir, schema).unwrap();

  let tokenizer = tantivy_jieba::JiebaTokenizer::new();
  let analyzer = TextAnalyzer::builder(tokenizer)
    .filter(RemoveLongFilter::limit(40))
    .filter(LowerCaser)
    .filter(Stemmer::default())
    .build();

  index.tokenizers().register("jieba", analyzer);

  (index, fields)
});

static NOVEL_READER: Lazy<IndexReader> = Lazy::new(|| NOVEL.0.reader().unwrap());

static NOVEL_TITLE_QUERY: Lazy<QueryParser> =
  Lazy::new(|| QueryParser::for_index(&NOVEL.0, vec![NOVEL.1.title]));

static NOVEL_AUTHOR_QUERY: Lazy<QueryParser> =
  Lazy::new(|| QueryParser::for_index(&NOVEL.0, vec![NOVEL.1.author]));

#[napi(object)]
pub struct ContentSearchResult {
  pub author: String,
  pub novel_id: u32,
  pub novel_title: String,
  pub chapter_id: String,
  pub chapter_title: String,
  pub chapter_index: u32,
}

#[napi(object)]
pub struct ContentSearchResponse {
  pub results: Vec<ContentSearchResult>,
  pub total: u32,
}

#[napi(object)]
pub struct NovelSearchResult {
  pub author: String,
  pub novel_id: u32,
  pub novel_title: String,
}

#[napi(object)]
pub struct NovelSearchResponse {
  pub results: Vec<NovelSearchResult>,
  pub total: u32,
}

fn get_str(doc: &TantivyDocument, field: Field) -> String {
  doc
    .get_first(field)
    .and_then(|v| v.as_str())
    .unwrap_or("")
    .to_string()
}

fn get_u32(doc: &TantivyDocument, field: Field) -> u32 {
  doc.get_first(field).and_then(|v| v.as_u64()).unwrap_or(0) as u32
}

fn calc_offset(page: u32) -> usize {
  (page.saturating_sub(1) as usize).saturating_mul(PAGE_SIZE)
}

#[napi]
impl Task for SearchContentTask {
  type Output = ContentSearchResponse;
  type JsValue = ContentSearchResponse;

  fn compute(&mut self) -> Result<Self::Output> {
    let searcher = CONTENT_READER.searcher();

    let query = match CONTENT_QUERY.parse_query(&self.keyword) {
      Ok(q) => q,
      Err(_) => {
        return Ok(ContentSearchResponse {
          results: vec![],
          total: 0,
        });
      }
    };

    let search_result = searcher.search(
      &query,
      &(
        TopDocs::with_limit(PAGE_SIZE)
          .and_offset(calc_offset(self.page))
          .order_by_score(),
        Count,
      ),
    );

    let (top_docs, total) = match search_result {
      Ok(res) => res,
      Err(_) => {
        return Ok(ContentSearchResponse {
          results: vec![],
          total: 0,
        })
      }
    };

    let mut results = Vec::with_capacity(top_docs.len());
    for (_, addr) in top_docs {
      if let Ok(doc) = searcher.doc(addr) {
        results.push(ContentSearchResult {
          author: get_str(&doc, CONTENT.1.author),
          novel_id: get_u32(&doc, CONTENT.1.novel_id),
          novel_title: get_str(&doc, CONTENT.1.novel_title),
          chapter_id: get_str(&doc, CONTENT.1.chapter_id),
          chapter_title: get_str(&doc, CONTENT.1.chapter_title),
          chapter_index: get_u32(&doc, CONTENT.1.chapter_index),
        });
      }
    }

    Ok(ContentSearchResponse {
      results,
      total: total as u32,
    })
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn search_content(keyword: String, page: u32) -> AsyncTask<SearchContentTask> {
  AsyncTask::new(SearchContentTask { keyword, page })
}

#[napi]
impl Task for SearchNovelTask {
  type Output = NovelSearchResponse;
  type JsValue = NovelSearchResponse;

  fn compute(&mut self) -> Result<Self::Output> {
    let searcher = NOVEL_READER.searcher();

    // 根据枚举类型，选择对应的全局 QueryParser
    let query_parser = match self.search_type {
      NovelSearchType::Title => &*NOVEL_TITLE_QUERY,
      NovelSearchType::Author => &*NOVEL_AUTHOR_QUERY,
    };

    let query = match query_parser.parse_query(&self.keyword) {
      Ok(q) => q,
      Err(_) => {
        return Ok(NovelSearchResponse {
          results: vec![],
          total: 0,
        });
      }
    };

    // 【优化点】使用 Tuple Collector 合并 TopDocs 和 Count，单次遍历索引
    let search_result = searcher.search(
      &query,
      &(
        TopDocs::with_limit(PAGE_SIZE)
          .and_offset(calc_offset(self.page))
          .order_by_score(),
        Count,
      ),
    );

    let (top_docs, total) = match search_result {
      Ok(res) => res,
      Err(_) => {
        return Ok(NovelSearchResponse {
          results: vec![],
          total: 0,
        })
      }
    };

    let mut results = Vec::with_capacity(top_docs.len());

    for (_, addr) in top_docs {
      if let Ok(doc) = searcher.doc(addr) {
        results.push(NovelSearchResult {
          author: get_str(&doc, NOVEL.1.author),
          novel_id: get_u32(&doc, NOVEL.1.id),
          novel_title: get_str(&doc, NOVEL.1.title),
        });
      }
    }

    Ok(NovelSearchResponse {
      results,
      total: total as u32,
    })
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn search_title(keyword: String, page: u32) -> AsyncTask<SearchNovelTask> {
  AsyncTask::new(SearchNovelTask {
    keyword,
    page,
    search_type: NovelSearchType::Title,
  })
}

#[napi]
pub fn search_author(keyword: String, page: u32) -> AsyncTask<SearchNovelTask> {
  AsyncTask::new(SearchNovelTask {
    keyword,
    page,
    search_type: NovelSearchType::Author,
  })
}
