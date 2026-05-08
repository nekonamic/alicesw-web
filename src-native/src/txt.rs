use napi::bindgen_prelude::*;
use napi_derive::napi;
use sevenz_rust2::*;
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
pub fn generate_svevnz(download_result: DownloadResult) -> Buffer {
  let mut archive_buffer = Cursor::new(Vec::new());

  let mut writer = ArchiveWriter::new(&mut archive_buffer).unwrap();

  let folder_name = format!("{}-{}", download_result.title, download_result.author);

  for (index, chapter) in download_result.chapters.iter().enumerate() {
    let file_path = format!("{}/{}-{}.txt", folder_name, index, chapter.title);

    let file_data = chapter.data.as_bytes();
    let file_size = file_data.len() as u64;

    let mut entry = ArchiveEntry::new_file(file_path.as_str());
    entry.size = file_size;
    entry.has_stream = true;
    entry.has_crc = true;

    let data_cursor = Cursor::new(file_data);
    writer.push_archive_entry(entry, Some(data_cursor)).unwrap();
  }

  writer.finish().unwrap();

  let result_bytes = archive_buffer.into_inner();
  Buffer::from(result_bytes)
}
