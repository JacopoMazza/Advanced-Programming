use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Book {
    pub name: String,
    pub code: String,
    pub pub_year: u16,
    pub author: String,
    pub pub_company: String,
}

#[derive(Debug)]
pub struct Article {
    pub name: String,
    pub code: String,
    pub pub_year: u16,
    pub orchid: String,
}
#[derive(Debug)]
pub struct Magazine {
    pub name: String,
    pub code: String,
    pub pub_year: u16,
    pub number: u16,
    pub month:u8
}

pub struct Library{
    pub book_list: Vec<Book>,
    pub article_list: Vec<Article>,
    pub magazine_list: Vec<Magazine>,
}

impl Library{
    pub fn new() -> Library{
        Library {
            book_list: Vec::new(),
            article_list: Vec::new(),
            magazine_list: Vec::new()
        }


    }

    pub fn add_book(&mut self,book:Book){
        self.book_list.push(book);
    }

    pub fn add_article(&mut self,article:Article){
        self.article_list.push(article);
    }
    pub fn add_magazine(&mut self,mag:Magazine){
        self.magazine_list.push(mag);
    }
}

impl Display for Library {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result{
        write!(f, "Books: {:?}\nArticles{:?}\nMagzines{:?}",self.book_list, self.article_list, self.magazine_list)
    }
}


