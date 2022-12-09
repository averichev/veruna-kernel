use async_trait::async_trait;
use url::Url;
use crate::sites::{Reader, Site, SiteBuilder, SiteBuilderImpl, SiteId, SiteIdBuilder, SiteIdBuilderImpl, SiteReader, SiteReadOption, SiteRepository};

#[async_trait(?Send)]
pub trait SiteKit {
    async fn create(&mut self, site: Box<dyn Site>) -> Box<dyn SiteId>;
    fn get_site(&self, url: Url) -> (&Box<dyn Site>, Box<dyn SiteId>);
    fn reader(&self) -> Box<dyn Reader + '_>;
    fn site_id_builder(&self) -> Box<dyn SiteIdBuilder>;
    fn site_builder(&self) -> Box<dyn SiteBuilder>;
}

struct SiteKitImpl {
    site_repository: Box<dyn SiteRepository>,
}

#[async_trait(?Send)]
impl SiteKit for SiteKitImpl {
    async fn create(&mut self, site: Box<dyn Site>) -> Box<dyn SiteId> {
        let result = self.site_repository.create(site).await;
        result
    }

    fn get_site(&self, url: Url) -> (&Box<dyn Site>, Box<dyn SiteId>) {
        let domain = url.domain().unwrap().to_string();
        let created_site = self.site_repository.read(SiteReadOption::Domain(domain));
        created_site
    }

    fn reader(&self) -> Box<dyn Reader + '_> {
        let repo = &self.site_repository;
        SiteReader::new(repo)
    }

    fn site_id_builder(&self) -> Box<dyn SiteIdBuilder> {
        let result = SiteIdBuilderImpl::new();
        result
    }

    fn site_builder(&self) -> Box<dyn SiteBuilder> {
        let result = SiteBuilderImpl::new();
        result
    }
}

pub struct SiteKitFactory;

impl SiteKitFactory {
    pub fn build(repo: Box<dyn SiteRepository>) -> Box<dyn SiteKit> {
        let result: Box<dyn SiteKit> = Box::new(SiteKitImpl { site_repository: repo });
        result
    }
}