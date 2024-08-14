use super::model::Article;

pub trait ArticleRepo {
    fn query_article(&self, req: QueryArticleReq) -> Result<i64, String>;
    fn query_article_list(&self, req: QueryArticleListReq) -> Result<Vec<Article>, String>;
}

pub struct QueryArticleReq {
    pub article_id: u64,
    pub status: super::model::ArticleStatus,
}

pub struct QueryArticleListReq {
    pub page: u64,
    pub page_size: u64,
    pub status: super::model::ArticleStatus,
}
