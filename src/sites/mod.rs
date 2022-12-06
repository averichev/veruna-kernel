mod site_kit;

use url::Url;
use crate::pages::PageId;
use shaku::{Provider};

pub trait SiteRepository {
    fn create(&self, site: Box<dyn Site>) -> Box<dyn SiteId>;
    fn read(&self, read_by: SiteReadOption) -> (Box<dyn Site>, Box<dyn SiteId>);
    fn delete(&self, site_id: Box<dyn Site>) -> bool;
}

pub trait SiteKit {
    fn get_site(&self, url: Url) -> (Box<dyn Site>, Box<dyn SiteId>);
}

pub trait CreatedSite {
    fn site(&self) -> dyn Site;
    fn site_id(&self) -> Box<dyn SiteId>;
}

pub trait SiteIdBuilder {
    fn build(id: u8) -> dyn SiteId;
}

pub trait SiteBuilder {
    fn build(&self) -> Box<dyn Site>;
}

#[derive(Provider)]
#[shaku(interface = SiteBuilder)]
pub struct SiteBuilderImpl;

impl SiteBuilder for SiteBuilderImpl {
    fn build(&self) -> Box<dyn Site> {
        let result = SiteImpl { domain: "".to_string(), name: "".to_string() };
        Box::new(result)
    }
}

pub enum SiteReadOption {
    SiteId(Box<dyn Site>),
    Domain(String),
}

pub trait Site {}

struct SiteImpl {
    domain: String,
    name: String,
}

impl Site for SiteImpl {}

pub trait SiteId {}

pub struct SitePages {
    pages: Vec<PageId>,
}