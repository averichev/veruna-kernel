use url::Url;
use crate::pages::page::PageId;

pub struct Site {
    url: Url,
    name: String,
}

pub struct SiteId {
    value: u8,
}

pub struct SitePages{
    pages: Vec<PageId>
}