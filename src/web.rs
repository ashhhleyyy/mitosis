use std::collections::HashMap;
use warp::Filter;
use crate::config::Config;
use crate::translations::TranslationsHolder;
use crate::util::TranslationsMap;

type WebApiResult = Result<Box<dyn warp::Reply>, warp::Rejection>;

async fn translate(default_locale: String, translations: TranslationsHolder, locale: String, keys: Vec<String>) -> WebApiResult {
    let mut translated = HashMap::new();
    let translations = translations.0.translations.read().unwrap();

    match translations.get(&locale) {
        Some(locale_translations) => {
            for key in keys {
                if let Some(translation) = locale_translations.get(&key) {
                    translated.insert(key.clone(), translation.clone());
                } else {
                    translation_fallback(&translations, &mut translated, &key, &default_locale);
                }
            }
        },
        None => {
            for key in keys {
                translation_fallback(&translations, &mut translated, &key, &default_locale);
            }
        },
    }

    Ok(Box::new(warp::reply::json(&translated)))
}

fn translation_fallback(translations: &TranslationsMap, translated: &mut HashMap<String, String>, key: &str, default_locale: &str) {
    if let Some(default_translations) = translations.get(default_locale) {
        translated.insert(key.to_string(),default_translations.get(key)
            .cloned().unwrap_or_else(|| key.to_string()));
    } else {
        translated.insert(key.to_string(), key.to_string());
    }
}

async fn reload_translations(translations: TranslationsHolder) -> WebApiResult {
    match translations.reload().await {
        Ok(_) => {
            Ok(Box::new(warp::reply::json(&"ok")))
        },
        Err(e) => {
            error!("Failed to reload translations: {}", e);
            Ok(Box::new(warp::reply::json(&"failed")))
        },
    }
}

pub async fn run(config: Config, translations: TranslationsHolder) {
    let cors = warp::cors();

    let translate_bulk = warp::path("translate")
        .and(warp::filters::method::get())
        .and(warp::filters::path::param())
        .and(warp::filters::body::json())
        .and_then({
            let translations = translations.clone();
            let config = config.clone();
            move |locale: String, keys: Vec<String>|
                translate(config.fallback_locale.clone(), translations.clone(), locale, keys)
        });

    let reload_translations = warp::path("reload")
        .and(warp::filters::method::post())
        .and_then({
            let translations = translations.clone();
            move || reload_translations(translations.clone())
        });

    let trans = warp::path("trans")
        .map(|| "🏳️‍⚧️");

    let combined = translate_bulk
        .or(reload_translations)
        .or(trans);

    warp::serve(combined.with(cors))
        .run(([127, 0, 0, 1], config.listen_port))
        .await;
}
