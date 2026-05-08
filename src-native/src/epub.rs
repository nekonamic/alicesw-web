use epub_builder::EpubContent;
use epub_builder::ReferenceType;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::io::Cursor;

#[napi(object)]
pub struct DownloadChapter {
  pub title: String,
  pub data: String,
}

#[napi(object)]
pub struct DownloadNovel {
  pub title: String,
  pub author: String,
}

#[napi(object)]
pub struct DownloadResult {
  pub title: String,
  pub author: String,
  pub chapters: Vec<DownloadChapter>,
}

#[napi]
pub fn generate_epub(download_result: DownloadResult) -> Buffer {
  let mut builder =
    epub_builder::EpubBuilder::new(epub_builder::ZipLibrary::new().unwrap()).unwrap();

  builder
    .metadata("title", &download_result.title)
    .unwrap()
    .metadata("author", &download_result.author)
    .unwrap()
    .add_content(
      EpubContent::new(
        "title.xhtml",
        Cursor::new(generate_xhtml(&download_result.title)),
      )
      .title("标题")
      .reftype(ReferenceType::Cover),
    )
    .unwrap();

  builder.inline_toc();

  for (index, chapter) in download_result.chapters.iter().enumerate() {
    builder
      .add_content(
        EpubContent::new(
          format!("chapter_{}.xhtml", index),
          Cursor::new(generate_xhtml(&chapter.data)),
        )
        .title(&chapter.title)
        .reftype(ReferenceType::Text),
      )
      .unwrap();
  }

  let mut output_data: Vec<u8> = Vec::new();
  {
    let mut cursor = Cursor::new(&mut output_data);
    builder.generate(&mut cursor).unwrap();
  }

  Buffer::from(output_data)
}

fn generate_xhtml(input: &str) -> Vec<u8> {
  let body_content = input
    .lines()
    .map(|line| format!("<p>{}</p>", line))
    .collect::<Vec<String>>()
    .join("\n");

  format!(
    r#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml"
      xmlns:epub="http://www.idpf.org/2007/ops">
<head>
  <title></title>
</head>
<body>
{}
</body>
</html>"#,
    body_content
  )
  .into_bytes()
}
