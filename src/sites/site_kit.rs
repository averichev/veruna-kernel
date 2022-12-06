use std::sync::Arc;
use url::Url;
use crate::sites::{Site, SiteId, SiteReadOption, SiteRepository};

pub struct SiteKit {
    site_repository: Arc<dyn SiteRepository>,
}

impl crate::sites::SiteKit for SiteKit {
    fn get_site(&self, url: Url) -> (Box<dyn Site>, Box<dyn SiteId>) {
        let domain = url.domain().unwrap().to_string();
        let created_site = self.site_repository.read(SiteReadOption::Domain(domain));
        created_site
    }
}