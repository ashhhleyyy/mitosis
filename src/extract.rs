use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use crate::util::ApiResult;

const TRANSLATION_PATH_REGEX: &str = "^data/([a-z0-9_.-]*)/lang/([a-z]{2}_[a-z]{2}).json$";

#[derive(Clone, Debug)]
pub enum TranslationSource {
    /// The content of a lang file in JSON format
    LangJson {
        locale: String,
        content: String,
    },
    /// A single mod JAR
    ZipPack(PathBuf),
    /// A single datapack directory
    DirectoryPack(PathBuf),
    /// A directory of mod JARs
    ModsDirectory(PathBuf),
    /// A directory of datapacks
    DatapacksDirectory(PathBuf),
}

// TODO: Should mods be validated here (check fabric.mod.json exists maybe?)
impl TranslationSource {
    pub async fn find_children(&self) -> ApiResult<Vec<TranslationSource>> {
        match self {
            TranslationSource::ZipPack(path) => {
                // TODO: Handle JIJ mods here?
                let path = path.clone();
                tokio::task::spawn_blocking(move || {
                    extract_lang_files_from_zip(&path)
                }).await.expect("failed to execute blocking task")
            },
            TranslationSource::DirectoryPack(path) => {
                extract_lang_files_from_dir(path).await
            },
            TranslationSource::ModsDirectory(directory) => {
                let mut entries = tokio::fs::read_dir(directory).await?;
                let mut mods = Vec::new();
                while let Some(entry) = entries.next_entry().await? {
                    if let Some(ext) = entry.path().extension() {
                        if ext == "jar" {
                            mods.push(TranslationSource::ZipPack(entry.path()));
                        }
                    }
                }
                Ok(mods)
            },
            TranslationSource::DatapacksDirectory(path) => {
                let mut entries = tokio::fs::read_dir(path).await?;
                let mut datapacks = Vec::new();
                while let Some(entry) = entries.next_entry().await? {
                    let is_dir = entry.file_type().await?.is_dir();
                    if is_dir {
                        let mut mcmeta = entry.path();
                        mcmeta.push("pack.mcmeta");
                        if mcmeta.exists() {
                            datapacks.push(TranslationSource::DirectoryPack(entry.path()));
                        }
                    } else if let Some(ext) = entry.path().extension() {
                        if ext == "zip" {
                            datapacks.push(TranslationSource::ZipPack(entry.path()));
                        }
                    }
                }
                Ok(datapacks)
            }
            _ => Ok(vec![]),
        }
    }

    pub fn parse_lang(self) -> ApiResult<(String, HashMap<String, String>)> {
        match self {
            TranslationSource::LangJson {
                content,
                locale
            } => {
                Ok((locale, serde_json::from_str(&content)?))
            }
            _ => panic!("tried to parse_lang on a non LangJson TranslationSource")
        }
    }
}

pub fn extract_lang_files_from_zip(path: &Path) -> ApiResult<Vec<TranslationSource>> {
    let file = fs::File::open(&path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let regex = regex::Regex::new(TRANSLATION_PATH_REGEX).unwrap();

    let mut translations = Vec::new();

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let filename = entry.name().to_string();
        trace!("In {:?}: {}", path, filename);
        if let Some(captures) = regex.captures(&filename) {
            trace!("Match in {:?}: {}", path, filename);
            let mut content = String::new();
            entry.read_to_string(&mut content)?;
            translations.push(TranslationSource::LangJson {
                content,
                locale: captures.get(2).unwrap().as_str().to_string(),
            });
        }
    }

    debug!("Loaded sources from ZIP {:?}: {:?}", path, translations);

    Ok(translations)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub async fn extract_lang_files_from_dir(path: &Path) -> ApiResult<Vec<TranslationSource>> {
    let regex = regex::Regex::new(TRANSLATION_PATH_REGEX).unwrap();
    let mut translations = Vec::new();

    for entry in WalkDir::new(path).into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
    {
        let filename = entry.path().strip_prefix(path).unwrap_or_else(|_| entry.path())
            .display().to_string();
        if let Some(captures) = regex.captures(&filename) {
            let content = tokio::fs::read_to_string(entry.path()).await?;
            translations.push(TranslationSource::LangJson {
                content,
                locale: captures.get(2).unwrap().as_str().to_string(),
            });
        }
    }

    Ok(translations)
}
