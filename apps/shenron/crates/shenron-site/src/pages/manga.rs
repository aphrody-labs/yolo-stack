use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::{
    content::MANGA_VOLUMES,
    i18n::{t, tf, Lang},
    pages::with_lang,
    reader::ReaderIsland,
    seo::SeoHead,
};

fn chapters_in(volume: u32) -> Option<u32> {
    MANGA_VOLUMES
        .iter()
        .find(|(v, _)| *v == volume)
        .map(|(_, n)| *n)
}

#[component]
pub fn MangaPage() -> impl IntoView {
    let params = use_params_map();
    with_lang(move |lang: Lang| {
        let (volume, chapter): (u32, u32) = {
            let p = params.read();
            (
                p.get("volume").and_then(|v| v.parse().ok()).unwrap_or(1),
                p.get("chapter").and_then(|c| c.parse().ok()).unwrap_or(1),
            )
        };
        let code = lang.code();
        let (v, c) = (volume.to_string(), chapter.to_string());
        let args = [("volume", v.as_str()), ("chapter", c.as_str())];
        let prev = (chapter > 1).then(|| format!("/{code}/manga/{volume}/{}", chapter - 1));
        let next = chapters_in(volume)
            .filter(|n| chapter < *n)
            .map(|_| format!("/{code}/manga/{volume}/{}", chapter + 1));
        view! {
            <SeoHead
                lang=lang
                path=format!("/manga/{volume}/{chapter}")
                title=tf(lang, "manga.title", &args)
                description=tf(lang, "manga.intro", &args)
                og_type="book"
            />
            <h1>{tf(lang, "manga.heading", &args)}</h1>
            <p>{tf(lang, "manga.intro", &args)}</p>
            <ReaderIsland lang=lang volume=volume chapter=chapter />
            <nav class="reader-nav">
                {prev.map(|href| view! { <a href=href rel="prev">{t(lang, "manga.prev")}</a> })}
                {next.map(|href| view! { <a href=href rel="next">{t(lang, "manga.next")}</a> })}
            </nav>
        }
    })
}
