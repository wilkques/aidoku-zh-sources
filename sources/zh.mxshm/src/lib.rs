#![no_std]
extern crate alloc;

mod fetch;
mod html;

use aidoku::{
    AidokuError, Chapter, DeepLinkHandler, DeepLinkResult, FilterValue, Home, HomeLayout, Listing,
    ListingProvider, Manga, MangaPageResult, Page, Result, Source,
    alloc::{String, Vec, string::ToString as _},
    prelude::*,
};

use crate::fetch::Fetch;
use crate::html::GenManga;

struct Mxshm;

impl Source for Mxshm {
    fn new() -> Self {
        Self
    }

    fn get_search_manga_list(
        &self,
        query: Option<String>,
        page: i32,
        _filters: Vec<FilterValue>,
    ) -> Result<MangaPageResult> {
        let response = Fetch::search_or_filter(query.as_deref(), page, &_filters)?
            .request()?
            .html()?;

        GenManga::manga_page_result(&response)
    }

    fn get_manga_update(
        &self,
        _manga: Manga,
        _needs_details: bool,
        _needs_chapters: bool,
    ) -> Result<Manga> {
        Err(AidokuError::Unimplemented)
    }

    fn get_page_list(&self, _manga: Manga, _chapter: Chapter) -> Result<Vec<Page>> {
        Err(AidokuError::Unimplemented)
    }
}

impl ListingProvider for Mxshm {
    fn get_manga_list(&self, _listing: Listing, _page: i32) -> Result<MangaPageResult> {
        Err(AidokuError::Unimplemented)
    }
}

impl Home for Mxshm {
    fn get_home(&self) -> Result<HomeLayout> {
        Err(AidokuError::Unimplemented)
    }
}

impl DeepLinkHandler for Mxshm {
    fn handle_deep_link(&self, _url: String) -> Result<Option<DeepLinkResult>> {
        Err(AidokuError::Unimplemented)
    }
}

register_source!(Mxshm, ListingProvider, Home, DeepLinkHandler);

// you can also implement tests via our custom test runner!
#[cfg(test)]
mod test {
    use super::*;
    use aidoku_test::aidoku_test;

    // all tests need to be annotated with the #[aidoku_test] attribute instead of #[test]
    #[aidoku_test]
    fn test_get_search_manga_list() {
        let source = Mxshm::new();

		let result = source
			.get_search_manga_list(Some("幼惑".to_string()), 1, Vec::new())
			.unwrap();

		println!("完整結果: {:#?}", result);
    }
}
