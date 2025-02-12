-- Trigger to generate copy_id based on book_id
CREATE TRIGGER generate_copy_id
AFTER INSERT ON BookCopies
FOR EACH ROW
BEGIN
    UPDATE BookCopies
    SET id = (SELECT COALESCE(MAX(id), 0) + 1 FROM BookCopies WHERE book_id = NEW.book_id)
    WHERE id = 0 AND book_id = NEW.book_id;
END;

-- Triggers for 'updated_at'
CREATE TRIGGER update_users_timestamp
AFTER UPDATE ON Users
FOR EACH ROW
BEGIN
    UPDATE Users
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;
CREATE TRIGGER update_books_timestamp
AFTER UPDATE ON Books
FOR EACH ROW
BEGIN
    UPDATE Books
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;
CREATE TRIGGER update_book_copies_timestamp
AFTER UPDATE ON BookCopies
FOR EACH ROW
BEGIN
    UPDATE BookCopies
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;
CREATE TRIGGER update_borrowing_timestamp
AFTER UPDATE ON Borrowing
FOR EACH ROW
BEGIN
    UPDATE Borrowing
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;
CREATE TRIGGER update_fines_timestamp
AFTER UPDATE ON Fines
FOR EACH ROW
BEGIN
    UPDATE Fines
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;
CREATE TRIGGER update_categories_timestamp
AFTER UPDATE ON Categories
FOR EACH ROW
BEGIN
    UPDATE Categories
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;
CREATE TRIGGER update_reviews_timestamp
AFTER UPDATE ON Reviews
FOR EACH ROW
BEGIN
    UPDATE Reviews
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;
CREATE TRIGGER update_reservations_timestamp
AFTER UPDATE ON Reservations
FOR EACH ROW
BEGIN
    UPDATE Reservations
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;

-- This trigger ensures that whenever a borrowing record is marked as returned,
-- the system automatically assigns the proper return date and adjusts
-- the status if the book was returned late.
CREATE TRIGGER update_borrowing_return_and_status
AFTER UPDATE OF status ON Borrowing
FOR EACH ROW
WHEN NEW.status = 'returned'
BEGIN
    UPDATE Borrowing
    SET return_date = COALESCE(NEW.return_date, CURRENT_DATE),
        status = CASE
                    WHEN COALESCE(NEW.return_date, CURRENT_DATE) > NEW.due_date THEN 'late'
                    ELSE 'returned'
                 END
    WHERE id = NEW.id;
END;
