pub mod site;

use crate::sites::site::{Site, SiteId};

pub trait SiteRepository {
    fn create(site: Site) -> SiteId;
    fn delete(site_id: SiteId) -> bool;
}