use mysql::{params, prelude::Queryable};
use crate::model::article::Article;

pub struct ArticleDao {
    pool: mysql::Pool,
}

impl ArticleDao {
    pub fn new(pool: mysql::Pool) -> ArticleDao {
        ArticleDao { pool }
    }

    pub fn get_articles(&self, offset: u32, limit: u32) -> Result<Vec<Article>, mysql::Error> {
        let mut conn = self.pool.get_conn()?;
        let stmt = conn.prep("SELECT article_id, title, content FROM article ORDER BY article_id DESC LIMIT :offset,:limit")?;
        conn.exec_map(&stmt, params! { "offset" => offset, "limit" => limit }, |(article_id, title, content)| {
            Article {
                article_id,
                title,
                content,
            }
        })
    }
}
