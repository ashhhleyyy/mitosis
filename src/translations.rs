use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use crate::extract::TranslationSource;
use crate::util::{ApiResult, TranslationsMap};

const RELOAD_INTERVAL: Duration = Duration::from_secs(60);

#[derive(Clone)]
pub struct TranslationsHolder(pub Arc<TranslationsHolderInner>);

pub struct TranslationsHolderInner {
    pub translations: RwLock<TranslationsMap>,
    last_update_time: Mutex<Instant>,
    sources: Vec<TranslationSource>,
}

impl TranslationsHolder {
    pub fn new(sources: Vec<TranslationSource>, initial: TranslationsMap) -> Self {
        Self(Arc::new(TranslationsHolderInner {
            translations: RwLock::new(initial),
            last_update_time: Mutex::new(Instant::now()),
            sources,
        }))
    }

    pub async fn reload(&self) -> ApiResult<()> {
        // Prevent reloading to frequently
        {
            let mut last_update_time = self.0.last_update_time.lock().unwrap();
            let now = Instant::now();
            if now - *last_update_time < RELOAD_INTERVAL {
                return Ok(());
            }
            *last_update_time = now;
        }

        // load them before getting the write lock, to prevent holding it for too long
        let new_translations = load_translations_from(&self.0.sources).await?;

        let mut translations = self.0.translations.write().unwrap();
        *translations = new_translations;

        Ok(())
    }
}

pub async fn load_translations_from(sources: &[TranslationSource]) -> ApiResult<TranslationsMap> {
    // Recursively follow the translations sources down until the actual JSON files are reached
    let mut sources_queue = sources.to_vec();
    let mut all_sources = Vec::new();
    while let Some(source) = sources_queue.pop() {
        let children = source.find_children().await.expect("failed to get children");
        if children.is_empty() {
            all_sources.push(source.clone());
        }
        for child in children {
            sources_queue.insert(0, child);
        }
    }

    let mut all_translations: TranslationsMap = TranslationsMap::new();
    for translations in all_sources.into_iter().map(|s| s.parse_lang()) {
        let (locale, translations) = translations.expect("failed to parse translations");
        if !all_translations.contains_key(&locale) {
            all_translations.insert(locale.clone(), HashMap::new());
        }
        let locale_translations = all_translations.get_mut(&locale).unwrap();
        for (key, value) in translations {
            if locale_translations.contains_key(&key) {
                warn!("overlapping translations for {} in locale {}", key, locale);
            }
            locale_translations.insert(key.clone(), value.clone());
        }
    }

    info!("Loaded translations for {} locales!", all_translations.len());

    Ok(all_translations)
}
