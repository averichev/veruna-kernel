pub mod site_kit;

use crate::pages::PageId;
use shaku::{module, Interface, Provider};

pub trait SiteRepository: Interface {
    fn create(&mut self, site: Box<dyn Site>) -> Box<dyn SiteId>;
    fn read(&self, read_by: SiteReadOption) -> (Box<dyn Site>, Box<dyn SiteId>);
    fn delete(&self, site_id: Box<dyn Site>) -> bool;
}

pub trait CreatedSite {
    fn site(&self) -> dyn Site;
    fn site_id(&self) -> Box<dyn SiteId>;
}

pub trait SiteIdBuilder {
    fn build(&self, id: u8) -> Box<dyn SiteId>;
}

#[derive(Provider)]
#[shaku(interface = SiteIdBuilder)]
pub struct SiteIdBuilderImpl;

impl SiteIdBuilder for SiteIdBuilderImpl {
    fn build(&self, id: u8) -> Box<dyn SiteId> {
        let result = SiteIdImpl { value: id };
        Box::new(result)
    }
}

pub trait SiteBuilder: Interface {
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
    SiteId(Box<dyn SiteId>),
    Domain(String),
}

pub trait Site : Interface {}

struct SiteImpl {
    domain: String,
    name: String,
}

impl Site for SiteImpl {}

pub trait SiteId {
    fn value(&self) -> u8;
}

struct SiteIdImpl {
    value: u8,
}

impl SiteId for SiteIdImpl {
    fn value(&self) -> u8 {
        self.value
    }
}

pub struct SitePages {
    pages: Vec<PageId>,
}

module! {
    SiteModule {
        components = [],
        providers = [SiteIdBuilderImpl]
    }
}