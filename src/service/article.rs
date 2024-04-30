use crate::model::article::Article;

pub struct ArticleService {
    dao: crate::dao::article::ArticleDao,
}

impl ArticleService {
    pub fn new(dao: crate::dao::article::ArticleDao) -> ArticleService {
        ArticleService { dao }
    }

    pub fn get_articles(&self, offset: u32, limit: u32) -> Result<Vec<Article>, mysql::Error> {
        self.dao.get_articles(offset, limit)
    }
}