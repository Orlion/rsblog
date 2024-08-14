pub struct Article {
    pub article_id: u64,
    pub title: String,
    pub content: String,
    pub publish_at: u64,
    pub create_at: u64,
    pub update_at: u64,
    pub category_id: u64,
    pub tag_id_1: u64,
    pub tag_id_2: u64,
    pub tag_id_3: u64,
    pub status: ArticleStatus,
}

pub type ArticleStatus = i8;

const ARTICLE_STATUS_DRAFT: ArticleStatus = 0;
const ARTICLE_STATUS_PUBLISH: ArticleStatus = 1;
const ARTICLE_STATUS_DELETE: ArticleStatus = -1;
