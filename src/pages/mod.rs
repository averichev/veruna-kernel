use crate::pages::page::{Page, PageId};
use crate::sites::site::SiteId;

pub mod page;

pub trait PageRepository {
    fn create(page: Page, site_id: SiteId) -> PageId;
    fn delete(page_id: PageId) -> bool;
}