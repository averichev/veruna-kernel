use crate::pages::page::{Page, PageId};

pub mod page;

pub trait PageRepository {
    fn add(page: Page) -> PageId;
    fn delete(page_id: PageId) -> bool;
}