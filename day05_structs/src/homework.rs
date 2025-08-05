// 创建一个 `Book` 结构体，包含标题、作者、页数、是否可借阅
#[derive(Debug, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: u32,
    pub is_available: bool,
}

// 为 `Book` 添加借阅、归还、获取信息等方法
impl Book {
    pub fn new(title: String, author: String, pages: u32) -> Self {
        Book {
            title,
            author,
            pages,
            is_available: true,
        }
    }

    pub fn borrow_book(&mut self) -> Result<(), String> {
        if self.is_available {
            self.is_available = false;
            Ok(())
        } else {
            Err("Book is already borrowed".to_string())
        }
    }

    pub fn return_book(&mut self) -> Result<(), String> {
        if !self.is_available {
            self.is_available = true;
            Ok(())
        } else {
            Err("Book is already available".to_string())
        }
    }

    pub fn get_info(&self) -> String {
        format!(
            "Title: {}, Author: {}, Pages: {}, Available: {}",
            self.title, self.author, self.pages, self.is_available
        )
    }
}

// 实现一个 `Library` 结构体，管理多本书籍
#[derive(Debug)]
pub struct Library {
    books: Vec<Book>,
}

impl Library {
    pub fn new() -> Self {
        Library { books: Vec::new() }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn find_book_by_title(&mut self, title: &str) -> Option<&mut Book> {
        self.books.iter_mut().find(|book| book.title == title)
    }

    pub fn borrow_book(&mut self, title: &str) -> Result<(), String> {
        match self.find_book_by_title(title) {
            Some(book) => book.borrow_book(),
            None => Err("Book not found".to_string()),
        }
    }

    pub fn return_book(&mut self, title: &str) -> Result<(), String> {
        match self.find_book_by_title(title) {
            Some(book) => book.return_book(),
            None => Err("Book not found".to_string()),
        }
    }

    pub fn list_available_books(&self) -> Vec<&Book> {
        self.books.iter().filter(|book| book.is_available).collect()
    }

    pub fn list_all_books(&self) -> &Vec<Book> {
        &self.books
    }
}

// 创建一个 `BankAccount` 结构体，实现存款、取款功能
#[derive(Debug)]
pub struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

impl BankAccount {
    pub fn new(account_number: String, holder_name: String, initial_balance: f64) -> Self {
        BankAccount {
            account_number,
            holder_name,
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be positive".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be positive".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        Ok(())
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn get_account_info(&self) -> String {
        format!(
            "Account: {}, Holder: {}, Balance: ${:.2}",
            self.account_number, self.holder_name, self.balance
        )
    }
}

pub fn homework() {
    println!("=== Book and Library Demo ===");
    
    let mut library = Library::new();
    
    library.add_book(Book::new(
        "The Rust Programming Language".to_string(),
        "Steve Klabnik and Carol Nichols".to_string(),
        552,
    ));
    
    library.add_book(Book::new(
        "Programming Rust".to_string(),
        "Jim Blandy and Jason Orendorff".to_string(),
        624,
    ));

    println!("Available books:");
    for book in library.list_available_books() {
        println!("  {}", book.get_info());
    }

    println!("\nBorrowing 'The Rust Programming Language'...");
    match library.borrow_book("The Rust Programming Language") {
        Ok(()) => println!("Book borrowed successfully!"),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nAvailable books after borrowing:");
    for book in library.list_available_books() {
        println!("  {}", book.get_info());
    }

    println!("\nReturning 'The Rust Programming Language'...");
    match library.return_book("The Rust Programming Language") {
        Ok(()) => println!("Book returned successfully!"),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Bank Account Demo ===");
    
    let mut account = BankAccount::new(
        "12345".to_string(),
        "John Doe".to_string(),
        1000.0,
    );

    println!("{}", account.get_account_info());

    println!("\nDepositing $500...");
    match account.deposit(500.0) {
        Ok(()) => println!("Deposit successful! New balance: ${:.2}", account.get_balance()),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nWithdrawing $200...");
    match account.withdraw(200.0) {
        Ok(()) => println!("Withdrawal successful! New balance: ${:.2}", account.get_balance()),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nTrying to withdraw $2000 (should fail)...");
    match account.withdraw(2000.0) {
        Ok(()) => println!("Withdrawal successful! New balance: ${:.2}", account.get_balance()),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nFinal account info: {}", account.get_account_info());
}


