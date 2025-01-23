-- Table for storing user information
CREATE TABLE Users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    phone TEXT,
    photo TEXT,
    address TEXT,
    role TEXT CHECK(role IN ('member', 'issuer', 'admin')) DEFAULT 'member',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Table for storing book information
CREATE TABLE Books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    isbn TEXT UNIQUE NOT NULL,
    category TEXT,
    year INTEGER,
    added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Table for storing individual book copies with composite key
CREATE TABLE BookCopies (
    id INTEGER NOT NULL DEFAULT 0,
    book_id INTEGER NOT NULL,
    status TEXT CHECK(status IN ('available', 'borrowed', 'reserved')) DEFAULT 'available',
    location TEXT,
    added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (book_id, id),
    FOREIGN KEY (book_id) REFERENCES Books(id) ON DELETE CASCADE
);

-- Table for storing borrowing transactions
CREATE TABLE Borrowing (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    copy_id INTEGER NOT NULL,
    book_id INTEGER NOT NULL,
    borrow_date DATE NOT NULL DEFAULT CURRENT_DATE,
    return_date DATE,
    status TEXT CHECK(status IN ('borrowed', 'returned', 'late')) DEFAULT 'borrowed',
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE,
    FOREIGN KEY (book_id, copy_id) REFERENCES BookCopies(book_id, id) ON DELETE CASCADE
);

-- Table for storing fine details
CREATE TABLE Fines (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_id INTEGER NOT NULL,
    fine_amount REAL NOT NULL,
    paid BOOLEAN DEFAULT FALSE,
    paid_date DATE,
    FOREIGN KEY (transaction_id) REFERENCES Borrowing(id) ON DELETE CASCADE
);

-- Table for storing categories
CREATE TABLE Categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL
);

-- Associative table for book-category relationships (many-to-many)
CREATE TABLE BookCategories (
    book_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    PRIMARY KEY (book_id, category_id),
    FOREIGN KEY (book_id) REFERENCES Books(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES Categories(id) ON DELETE CASCADE
);

-- Table for tracking book reviews
CREATE TABLE Reviews (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    book_id INTEGER NOT NULL,
    rating INTEGER CHECK(rating BETWEEN 1 AND 5),
    review_text TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE,
    FOREIGN KEY (book_id) REFERENCES Books(id) ON DELETE CASCADE
);

-- Table for book reservations
CREATE TABLE Reservations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    copy_id INTEGER NOT NULL,
    book_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    reservation_date DATE NOT NULL DEFAULT CURRENT_DATE,
    status TEXT CHECK(status IN ('active', 'cancelled', 'fulfilled')) DEFAULT 'active',
    FOREIGN KEY (book_id, copy_id) REFERENCES BookCopies(book_id, id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE
);
