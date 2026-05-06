#![deny(clippy::all)]

use napi_derive::napi;
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
  let index_path = Path::new("./content_index");
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
pub struct TitleSearchResult {
  pub author: String,
  pub novel_id: u32,
  pub novel_title: String,
}

#[napi]
pub fn search_title(keyword: String, page: u32) -> Vec<TitleSearchResult> {
  let mut schema_builder = Schema::builder();
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
  let schema = schema_builder.build();

  let tokenizer = tantivy_jieba::JiebaTokenizer::new();
  let index_path = Path::new("./title_index");
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
  let query_parser = QueryParser::for_index(&index, vec![novel_title]);
  let query = query_parser.parse_query(&keyword).unwrap();

  let top_docs = searcher
    .search(&query, &TopDocs::with_limit(20).and_offset((page * 20).try_into().unwrap()).order_by_score())
    .unwrap();
  let mut results: Vec<TitleSearchResult> = Vec::new();
  for (_, doc_address) in top_docs {
    let retrieved_doc: TantivyDocument = searcher.doc(doc_address).unwrap();
    results.push(TitleSearchResult {
        author: retrieved_doc.get_first(author).unwrap().as_str().unwrap_or_default().to_string(),
        novel_id: retrieved_doc.get_first(novel_id).unwrap().as_u64().unwrap_or_default() as u32,
        novel_title: retrieved_doc.get_first(novel_title).unwrap().as_str().unwrap_or_default().to_string(),
    });
  }
  results
}
