//! Application shell, layout and static route tree.

use leptos::prelude::*;
use leptos_meta::{Html, MetaTags, Stylesheet, provide_meta_context};
use leptos_router::{
    SsrMode,
    components::{Route, Router, Routes},
    hooks::use_location,
    path,
    static_routes::{StaticParamsMap, StaticRoute},
};

use crate::{
    content::{databook_slugs, langs, manga_chapters, wiki_slugs},
    i18n::{Lang, t},
    pages::{CharactersPage, DatabookPage, HomePage, MangaPage, NotFound, WikiPage},
};

/// HTML document shell rendered by the server for every route.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

/// Static params for the locale-only routes (`/{lang}`, `/{lang}/characters`).
fn lang_params() -> StaticRoute {
    StaticRoute::new().prerender_params(|| async move {
        let mut map = StaticParamsMap::new();
        map.insert("lang", langs());
        map
    })
}

/// `/{lang}/wiki/{slug}`
fn wiki_params() -> StaticRoute {
    StaticRoute::new().prerender_params(|| async move {
        let mut map = StaticParamsMap::new();
        map.insert("lang", langs());
        map.insert("slug", wiki_slugs());
        map
    })
}

/// `/{lang}/databooks/{slug}`
fn databook_params() -> StaticRoute {
    StaticRoute::new().prerender_params(|| async move {
        let mut map = StaticParamsMap::new();
        map.insert("lang", langs());
        map.insert("slug", databook_slugs());
        map
    })
}

/// `/{lang}/manga/{volume}/{chapter}`
fn manga_params() -> StaticRoute {
    StaticRoute::new().prerender_params(|| async move {
        let chapters = manga_chapters();
        let mut map = StaticParamsMap::new();
        map.insert("lang", langs());
        map.insert(
            "volume",
            chapters.iter().map(|(v, _)| v.to_string()).collect(),
        );
        map.insert(
            "chapter",
            chapters.iter().map(|(_, c)| c.to_string()).collect(),
        );
        map
    })
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/shenron-site.css" />
        <Router>
            <Layout>
                <Routes fallback=|| view! { <NotFound /> }>
                    <Route
                        path=path!("/:lang")
                        view=HomePage
                        ssr=SsrMode::Static(lang_params())
                    />
                    <Route
                        path=path!("/:lang/characters")
                        view=CharactersPage
                        ssr=SsrMode::Static(lang_params())
                    />
                    <Route
                        path=path!("/:lang/wiki/:slug")
                        view=WikiPage
                        ssr=SsrMode::Static(wiki_params())
                    />
                    <Route
                        path=path!("/:lang/manga/:volume/:chapter")
                        view=MangaPage
                        ssr=SsrMode::Static(manga_params())
                    />
                    <Route
                        path=path!("/:lang/databooks/:slug")
                        view=DatabookPage
                        ssr=SsrMode::Static(databook_params())
                    />
                </Routes>
            </Layout>
        </Router>
    }
}

/// Header (nav + locale switch), main and footer. The locale comes from the
/// URL prefix so the layout can live outside the `Routes` tree.
#[component]
fn Layout(children: Children) -> impl IntoView {
    let location = use_location();
    let pathname = location.pathname;
    let lang = Memo::new(move |_| Lang::from_path(&pathname.get()).unwrap_or(Lang::DEFAULT));
    // Same page in another locale: swap the first segment.
    let switch_href = move |target: Lang| {
        let path = pathname.get();
        let rest = match Lang::from_path(&path) {
            Some(l) => path
                .trim_start_matches('/')
                .strip_prefix(l.code())
                .unwrap_or(""),
            None => "",
        };
        format!("/{}{rest}", target.code())
    };

    view! {
        <Html attr:lang=move || lang.get().code() />
        <header class="site-header">
            <nav aria-label=move || t(lang.get(), "nav.home")>
                <a class="brand" href=move || format!("/{}", lang.get().code())>
                    {move || t(lang.get(), "site.name")}
                </a>
                <a href=move || format!("/{}/characters", lang.get().code())>
                    {move || t(lang.get(), "nav.characters")}
                </a>
                <a href=move || format!("/{}/manga/1/1", lang.get().code())>
                    {move || t(lang.get(), "nav.manga")}
                </a>
                <a href=move || format!("/{}/databooks/daizenshuu-7", lang.get().code())>
                    {move || t(lang.get(), "nav.databooks")}
                </a>
                <div
                    class="lang-switch"
                    role="group"
                    aria-label=move || t(lang.get(), "lang.switch_label")
                >
                    {Lang::ALL
                        .iter()
                        .map(|target| {
                            let target = *target;
                            view! {
                                <a
                                    href=move || switch_href(target)
                                    hreflang=target.code()
                                    lang=target.code()
                                    aria-current=move || (lang.get() == target).then_some("true")
                                >
                                    {move || t(lang.get(), &target.name_key())}
                                </a>
                            }
                        })
                        .collect_view()}
                </div>
            </nav>
        </header>
        <main>{children()}</main>
        <footer class="site-footer">
            <p class="muted">{move || t(lang.get(), "footer.tagline")}</p>
        </footer>
    }
}
