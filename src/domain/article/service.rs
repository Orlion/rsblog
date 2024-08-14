use super::model::Article;

pub trait ArticleService {
    fn query_active_article(&self, article_id: u64) -> Result<Article, String>;
    fn page_active_article_list(&self, page: u64, page_size: u64) -> Result<Vec<Article>, String>;
}
