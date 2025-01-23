-- Trigger to generate copy_id based on book_id
CREATE TRIGGER generate_copy_id
AFTER INSERT ON BookCopies
FOR EACH ROW
BEGIN
    UPDATE BookCopies
    SET id = (SELECT COALESCE(MAX(id), 0) + 1 FROM BookCopies WHERE book_id = NEW.book_id)
    WHERE id = 0 AND book_id = NEW.book_id;
END;
