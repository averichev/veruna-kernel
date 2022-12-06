use std::sync::Arc;
use url::Url;
use crate::sites::{Site, SiteBuilder, SiteId, SiteIdBuilder, SiteModule, SiteReadOption, SiteRepository};
use shaku::HasProvider;

pub struct SiteKit {
    site_repository: Arc<dyn SiteRepository>,
}

impl SiteKit {
    pub fn get_site(&self, url: Url) -> (&Box<dyn Site>, Box<dyn SiteId>) {
        let domain = url.domain().unwrap().to_string();
        let created_site = self.site_repository.read(SiteReadOption::Domain(domain));
        created_site
    }
    pub fn site_id_builder() -> Box<dyn SiteIdBuilder> {
        let module = SiteModule::builder().build();
        let service: Box<dyn SiteIdBuilder> = module.provide().unwrap();
        service
    }

    pub fn site_builder() -> Box<dyn SiteBuilder> {
        let module = SiteModule::builder().build();
        let service: Box<dyn SiteBuilder> = module.provide().unwrap();
        service
    }
}