use csv::{self, DeserializeRecordsIter, Reader, ReaderBuilder};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::vec;
use zip::read::ZipFile;
use zip::ZipArchive;

use anyhow::Result;
use serde::Deserialize;

use crate::deserializer::bool_from_int;

use super::data_source::FsDataSource;

#[derive(Debug, Deserialize)]
pub struct FsTag {
    pub tag: String,
    pub version: String,
    #[serde(deserialize_with = "bool_from_int")]
    pub custom: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub r#abstract: bool,
    pub datatype: String,
    pub iord: String,
    pub crdr: String,
    pub tlabel: String,
    pub doc: String,
}

type FsZipArchive = ZipArchive<File>;
type FileIter = vec::IntoIter<FsZipArchive>;
type RecordIter<'a> = csv::DeserializeRecordsIntoIter<BufReader<ZipFile<'a>>, FsTag>;

pub struct FsTagRecords<'a> {
    file_iter: FileIter,
    record_iter: Option<RecordIter<'a>>,
}

impl<'a> FsTagRecords<'a> {
    pub fn new(datasource: FsDataSource) -> Result<Self> {
        let archive_files = datasource
            .zip_files
            .into_iter()
            .map(|path| {
                let file =
                    File::open(&path).unwrap_or_else(|e| panic!("Should open {path:?}: {e}"));
                zip::ZipArchive::new(file)
                    .unwrap_or_else(|e| panic!("Should get zip file {path:?}: {e}"))
            })
            .collect::<Vec<FsZipArchive>>();
        let file_iter = archive_files.into_iter();

        Ok(Self {
            file_iter,
            record_iter: None,
        })
    }

    fn get_next_reader(&mut self) -> Result<Option<RecordIter>> {
        match self.file_iter.next() {
            Some(mut archive) => {
                let tag_file = archive.by_name("tag.tsv")?;

                let reader = BufReader::new(tag_file);
                let reader: Reader<BufReader<ZipFile>> =
                    ReaderBuilder::new().delimiter(b'\t').from_reader(reader);
                let iter = reader.into_deserialize();

                Ok(Some(iter))
            }
            None => Ok(None),
        }
    }
}

impl<'a> Iterator for FsTagRecords<'a> {
    type Item = FsTag;

    //     for record in reader.deserialize::<FsTag>() {
    //         match record {
    //             Ok(r) => println!("{:?}", r),
    //             Err(e) => println!("{:?}", e),
    //         }
    //     }
    fn next(&mut self) -> Option<Self::Item> {
        // loop till get a valid reader or reader is None
        None
    }
}
