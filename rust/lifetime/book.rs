struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

impl<'a> Book<'a> {
    fn get_title(&self) -> &'a str {
        self.title
    }

    fn get_author(&self) -> &'a str {
        self.author
    }
}

fn main() {
    let book = Book {
        title: "1984",
        author: "George Orwell",
    };

    println!("Title: {}", book.get_title());
    println!("Author: {}", book.get_author);
}
