use crate::sites::SiteId;

pub trait PageRepository {
    fn create(page: Page, site_id: SiteId) -> PageId;
    fn delete(page_id: PageId) -> bool;
}


pub struct Page {
    url: String,
    name: String,
}

pub struct PageId {
    value: u8,
}

pub struct PageSite {
    page: PageId,
    site: SiteId,
}