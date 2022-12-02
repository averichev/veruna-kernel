use url::Url;
use crate::pages::PageId;

pub trait SiteKit {
    fn get_site(url: Url) -> (Site, SiteId);
}

pub trait SiteIdBuilder {
    fn build(id: u8) -> SiteId {
        SiteId {
            value: id,
        }
    }
}

pub trait SiteRepository {
    fn create(site: Site) -> SiteId;
    fn read(site_id: SiteId) -> Site;
    fn delete(site_id: SiteId) -> bool;
}

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