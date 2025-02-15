// #![allow(dead_code)]

use std::io;

#[derive(Debug)]
#[derive(Clone)]
struct Book {
    id: usize,
    name: String,
    author: String,
    publisher: String,
}

fn main() {
    let mut books = Vec::<Book>::new();

    loop {
        println!("1. Add a book");
        println!("2. Delete a book");
        println!("3. Get all books");
        println!("0. Exit bookstore");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let clean: usize = input.trim().parse().unwrap_or(0);

        match clean {
            x if x == 0 => {
                println!("Have a good day!😎");
                break;
            }
            1 => add_book(&mut books),
            2 => delete_book(&mut books),
            3 => list_books(&mut books),
            _ => {
                println!("Choose a number from the options above!");
                continue;
            }
        }
    }
}

fn add_book(books: &mut Vec<Book>) {
    let mut name = String::new();
    let mut author = String::new();
    let mut publisher = String::new();

    println!("Enter book's name");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line!");

    println!("Enter author's name");
    io::stdin()
        .read_line(&mut author)
        .expect("Failed to read line!");

    println!("Enter publisher's name");
    io::stdin()
        .read_line(&mut publisher)
        .expect("Failed to read line!");

    let new_book: Book = Book {
        id: books.len() + 1,
        name: name.trim().to_lowercase(),
        author: author.trim().to_lowercase(),
        publisher: publisher.trim().to_lowercase(),
    };

    println!("You've added the book: {}", new_book.name);
    books.push(new_book);
}

fn delete_book(books: &mut Vec<Book>) {
    println!("Enter id of book to delete");
    let mut id = String::new();

    io::stdin()
        .read_line(&mut id)
        .expect("Failed to read line!");

    let clean: usize = id.trim().parse().unwrap_or(0);

    let filtered_books: Vec<Book> = books.iter().filter(|b| b.id != clean).cloned().collect();

    *books = filtered_books;

    println!("You've deleted book {}", clean);
}

fn list_books(books: &mut Vec<Book>) {
    println!("Here's the list of all books: {:?}", books);
}
