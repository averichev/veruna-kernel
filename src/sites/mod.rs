pub mod site;

use crate::sites::site::{Site, SiteId};

pub trait SiteRepository {
    fn add(site: Site) -> SiteId;
}