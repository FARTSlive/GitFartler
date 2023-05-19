use anyhow::{Context, Result};
use serde_jsonlines::JsonLinesReader;

use crate::{
    app::sessions,
    reader::{self, Reader},
};

use super::Bookmark;

pub struct BookmarksReader<'reader> {
    session_reader: &'reader sessions::Reader<'reader>,
}

impl<'reader> BookmarksReader<'reader> {
    pub fn new(session_reader: &'reader sessions::Reader) -> Self {
        Self { session_reader }
    }

    pub fn read(&self) -> Result<Vec<Bookmark>> {
        match self
            .session_reader
            .read_to_string("session/bookmarks.jsonl")
        {
            Ok(content) => {
                let iter = JsonLinesReader::new(content.as_bytes()).read_all();
                let mut bookmarks = vec![];
                for result in iter {
                    let bookmark: Bookmark = result.context("failed to parse bookmark")?;
                    bookmarks.push(bookmark);
                }
                Ok(bookmarks)
            }
            Err(reader::Error::NotFound) => Ok(vec![]),
            Err(err) => Err(err.into()),
        }
    }
}
