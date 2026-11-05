//! Content catalog used to prerender static routes and to emit the sitemap.
//!
//! In the skeleton the catalog is a set of constants; the real site feeds it
//! from the Shenron API (wiki entities, manga volumes, databooks).

use crate::i18n::Lang;

/// Public origin, used for canonical URLs, `og:url`, hreflang and the sitemap.
pub const SITE_ORIGIN: &str = "https://dragonballfr.com";
/// Default OpenGraph image, served from `public/`.
pub const OG_IMAGE_PATH: &str = "/og/shenron.png";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WikiKind {
    Character,
    Technique,
    Place,
}

#[derive(Clone, Copy, Debug)]
pub struct WikiEntry {
    pub slug: &'static str,
    pub kind: WikiKind,
}

pub const WIKI: &[WikiEntry] = &[
    WikiEntry { slug: "son-goku", kind: WikiKind::Character },
    WikiEntry { slug: "bulma", kind: WikiKind::Character },
    WikiEntry { slug: "piccolo", kind: WikiKind::Character },
    WikiEntry { slug: "vegeta", kind: WikiKind::Character },
    WikiEntry { slug: "freezer", kind: WikiKind::Character },
    WikiEntry { slug: "kamehameha", kind: WikiKind::Technique },
    WikiEntry { slug: "genkidama", kind: WikiKind::Technique },
    WikiEntry { slug: "namek", kind: WikiKind::Place },
];

pub const DATABOOKS: &[&str] = &[
    "daizenshuu-1",
    "daizenshuu-7",
    "chouzenshuu-1",
    "super-exciting-guide",
];

/// `(volume, number of chapters in that volume)`.
pub const MANGA_VOLUMES: &[(u32, u32)] = &[(1, 11), (2, 12), (3, 12)];

pub fn wiki_slugs() -> Vec<String> {
    WIKI.iter().map(|e| e.slug.to_owned()).collect()
}

pub fn character_slugs() -> impl Iterator<Item = &'static str> {
    WIKI.iter()
        .filter(|e| e.kind == WikiKind::Character)
        .map(|e| e.slug)
}

pub fn databook_slugs() -> Vec<String> {
    DATABOOKS.iter().map(|s| (*s).to_owned()).collect()
}

/// Every `(volume, chapter)` pair.
pub fn manga_chapters() -> Vec<(u32, u32)> {
    MANGA_VOLUMES
        .iter()
        .flat_map(|(v, n)| (1..=*n).map(move |c| (*v, c)))
        .collect()
}

pub fn langs() -> Vec<String> {
    Lang::ALL.iter().map(|l| l.code().to_owned()).collect()
}

/// Locale-independent paths (without the `/{lang}` prefix) of every page.
pub fn all_paths() -> Vec<String> {
    let mut paths = vec![String::new(), "/characters".to_owned(), "/theme".to_owned()];
    paths.extend(WIKI.iter().map(|e| format!("/wiki/{}", e.slug)));
    paths.extend(
        manga_chapters()
            .into_iter()
            .map(|(v, c)| format!("/manga/{v}/{c}")),
    );
    paths.extend(DATABOOKS.iter().map(|s| format!("/databooks/{s}")));
    paths
}

/// Absolute URL of a page for a locale.
pub fn url(lang: Lang, path: &str) -> String {
    format!("{SITE_ORIGIN}/{}{path}", lang.code())
}

/// Human-readable label from a slug (`son-goku` -> `Son Goku`).
pub fn humanize(slug: &str) -> String {
    slug.split('-')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
