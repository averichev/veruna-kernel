use crate::sites::site::SiteId;

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