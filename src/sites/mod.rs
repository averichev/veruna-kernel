use url::Url;
use crate::pages::PageId;


pub struct Site {
    url: Url,
    name: String,
}

pub struct SiteId {
    value: u8,
}

pub struct SitePages {
    pages: Vec<PageId>,
}

pub trait SiteRepository {
    fn create(site: Site) -> SiteId;
    fn delete(site_id: SiteId) -> bool;
}