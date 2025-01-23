-- Drop tables in reverse order to handle foreign key dependencies
DROP TABLE IF EXISTS Reservations;
DROP TABLE IF EXISTS Reviews;
DROP TABLE IF EXISTS BookCategories;
DROP TABLE IF EXISTS Categories;
DROP TABLE IF EXISTS Fines;
DROP TABLE IF EXISTS Borrowing;
DROP TABLE IF EXISTS BookCopies;
DROP TABLE IF EXISTS Books;
DROP TABLE IF EXISTS Users;
