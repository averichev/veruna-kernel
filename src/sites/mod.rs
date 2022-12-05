mod site_kit;

use url::Url;
use crate::pages::PageId;

pub trait SiteRepository {
    fn create(&self, site: Site) -> SiteId;
    fn read(&self, read_by: SiteReadOption) -> (Site, SiteId);
    fn delete(&self, site_id: SiteId) -> bool;
}



pub trait SiteKit {
    fn get_site(&self, url: Url) -> (Site, SiteId);
}

pub trait SiteIdBuilder {
    fn build(id: u8) -> SiteId {
        SiteId {
            value: id,
        }
    }
}

pub enum SiteReadOption {
    SiteId(SiteId),
    Domain(String),
}

pub struct Site {
    domain: String,
    name: String,
}

pub struct SiteId {
    value: u8,
}

pub struct SitePages {
    pages: Vec<PageId>,
}