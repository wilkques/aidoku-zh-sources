use aidoku::{
    Manga, MangaPageResult, Result,
    alloc::{String, Vec, string::ToString as _},
    imports::html::Document,
    prelude::*,
};

pub trait GenManga {
    fn manga_page_result(&self) -> Result<MangaPageResult>;
    fn update_details(&self, manga: &mut Manga) -> Result<()>;
}

impl GenManga for Document {
    fn manga_page_result(&self) -> Result<MangaPageResult> {
        let mut mangas: Vec<Manga> = Vec::new();

        let items = self
            .select(".mh-item")
            .ok_or_else(|| error!("No manga items found"))?;

        for item in items {
            let id = item
                .select_first("a")
                .ok_or_else(|| error!("No link found"))?
                .attr("href")
                .ok_or_else(|| error!("No link found"))?
                .split("/")
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .pop()
                .unwrap();
            let cover = item
                .select_first("a>p")
                .ok_or_else(|| error!("No cover found"))?
                .attr("style")
                .ok_or_else(|| error!("No style found"))?
                .replace("background-image: url(", "")
                .replace(")", "");
            let title = item
                .select_first(".mh-item-detali>h2>a")
                .ok_or_else(|| error!("No title found"))?
                .text()
                .ok_or_else(|| error!("No title found"))?
                .trim()
                .to_string();
            mangas.push(Manga {
                key: id,
                cover: Some(cover),
                title,
                ..Default::default()
            });
        }

        let has_more = self
			.select("div.page-pagination a:contains(>)")
			.map(|elements| elements.count() > 0)
			.unwrap_or(false);

        Ok(MangaPageResult {
			entries: mangas,
			has_next_page: has_more,
        })
    }

    fn update_details(&self, manga: &mut Manga) -> Result<()> {
        Ok(())
    }
}
