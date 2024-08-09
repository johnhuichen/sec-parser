use colored::Colorize;
use csv::ReaderBuilder;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use serde::Deserialize;

use crate::deserializer::bool_from_int;
use crate::traits::FileReader;

use super::data_source::FsDataSource;

#[derive(Debug, Deserialize)]
pub struct FsTag {
    pub tag: String,
    pub version: String,
    #[serde(deserialize_with = "bool_from_int")]
    pub custom: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub r#abstract: bool,
    pub datatype: Option<String>,
    pub iord: Option<char>,
    pub crdr: Option<char>,
    pub tlabel: Option<String>,
    pub doc: Option<String>,
}

pub struct FsTagRecords {
    // pub count: usize,
    //
    // lines: FileLines,
}

impl FileReader for FsTagRecords {}

impl FsTagRecords {
    pub fn new(datasource: FsDataSource) -> Result<()> {
        for file in datasource.zip_files {
            let file = File::open(file)?;
            let mut archive = zip::ZipArchive::new(file)?;

            let tag_file = archive.by_name("tag.tsv")?;
            let reader = BufReader::new(tag_file);
            let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(reader);
            for record in reader.deserialize::<FsTag>() {
                match record {
                    Ok(r) => println!("{}", format!("{:?}", r).bright_green()),
                    Err(e) => println!("{}", format!("{:?}", e).bright_red()),
                }
                break;
            }
        }

        Ok(())
        // Ok(FsTagRecords {})
    }
}

impl Iterator for FsTagRecords {
    type Item = FsTag;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
