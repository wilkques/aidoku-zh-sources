#![no_std]
use aidoku::{
    FilterValue, Result,
    alloc::{String, string::ToString as _},
    helpers::uri::encode_uri,
    imports::{defaults::defaults_get, net::Request},
    prelude::*,
};

const FILTER_TAG: [&str; 23] = [
    "全部", "青春", "性感", "长腿", "多人", "御姐", "巨乳", "新婚", "媳妇", "暧昧", "清纯", "调教",
    "少妇", "风骚", "同居", "淫乱", "好友", "女神", "诱惑", "偷情", "出轨", "正妹", "家教",
];
const FILTER_AREA: [&str; 4] = ["-1", "1", "2", "3"];
const FILTER_END: [&str; 3] = ["-1", "0", "1"];
const FILTER_LIST_TYPE: [&str; 2] = ["booklist", "update"];

fn base_url() -> String {
    defaults_get::<String>("url").unwrap_or(String::from("https://www.mxs13.cc"))
}

fn user_agent() -> String {
    defaults_get::<String>("User-Agent")
		.unwrap_or(String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36"))
}

#[derive(Clone)]
pub enum Fetch {
    Filter {
        tag: String,
        area: String,
        end: String,
        page: i32,
    },
    Search {
        query: String,
        page: i32,
    },
    MangaChapters {
        id: String,
    },
    ListType {
        list_type: String,
        page: i32,
    },
    Manga {
        id: String,
    },
}

impl Fetch {
    pub fn search_or_filter(
        query: Option<&str>,
        page: i32,
        filters: &[FilterValue],
    ) -> Result<Self> {
        if let Some(q) = query {
            return Ok(Self::Search {
                query: encode_uri(&q),
                page,
            });
        }

        // let mut query = String::new();
        let mut tag = String::new();
        let mut area = String::new();
        let mut end = String::new();
        let mut list_type = String::new();

        for filter in filters {
            match filter {
                FilterValue::Text { value, .. } => {
                    return Ok(Self::Search {
                        query: encode_uri(value.clone()),
                        page,
                    });
                }
                FilterValue::Select { id, value } => match id.as_str() {
                    "题材" => {
                        if FILTER_TAG.contains(&value.as_str()) {
                            let index = FILTER_TAG
                                .iter()
                                .position(|&r| r == value.as_str())
                                .unwrap_or(0);
                            tag = FILTER_TAG[index].to_string();
                        }
                    }
                    "地区" => {
                        if FILTER_AREA.contains(&value.as_str()) {
                            let index = FILTER_AREA
                                .iter()
                                .position(|&r| r == value.as_str())
                                .unwrap_or(0);
                            area = FILTER_AREA[index].to_string();
                        }
                    }
                    "进度" => {
                        if FILTER_END.contains(&value.as_str()) {
                            let index = FILTER_END
                                .iter()
                                .position(|&r| r == value.as_str())
                                .unwrap_or(0);
                            end = FILTER_END[index].to_string();
                        }
                    }
                    "列表" => {
                        if FILTER_LIST_TYPE.contains(&value.as_str()) {
                            let index = FILTER_LIST_TYPE
                                .iter()
                                .position(|&r| r == value.as_str())
                                .unwrap_or(0);

                            list_type = FILTER_LIST_TYPE[index].to_string();

                            if list_type == "update" {
                                return Ok(Self::ListType { list_type, page });
                            }
                        }
                    }
                    _ => continue,
                },
                _ => continue,
            }
        }

        Ok(Self::Filter {
            tag,
            area,
            end,
            page,
        })
    }

    pub fn gen_url(&self) -> String {
        match self {
            Fetch::Manga { id } => {
                format!("{}/chapter/{}", base_url(), id)
            }
            Fetch::MangaChapters { id } => {
                format!("{}/book/{}", base_url(), id)
            }
            Fetch::Search { query, page } => {
                format!("{}/search?keyword={}&page={}", base_url(), query, page)
            }
            Fetch::ListType { list_type, page } => {
                format!("{}/{}?page={}", base_url(), list_type, page)
            }
            Fetch::Filter {
                tag,
                area,
                end,
                page,
            } => {
                format!(
                    "{}/booklist?tag={}&area={}&end={}&page={}",
                    base_url(),
                    tag,
                    area,
                    end,
                    page
                )
            }
        }
    }

    pub fn request(&self) -> Result<Request> {
        let url = self.gen_url();

        Ok(Request::get(&url)?.header("User-Agent", &user_agent()))
    }
}
