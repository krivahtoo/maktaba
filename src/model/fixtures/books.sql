-- Insert sample books
INSERT INTO Books (title, author, isbn, category, year)
VALUES
  ('Book 1', 'Author 1', '1234567890', 'Category 1', 2020),
  ('Book 2', 'Author 2', '9876543210', 'Category 2', 2019),
  ('Book 3', 'Author 3', '4567891230', 'Category 1', 2021);

INSERT INTO BookCopies (book_id, status, location)
VALUES
  (1, 'available', 'Library 1'),
  (1, 'available', 'Library 1'),
  (2, 'available', 'Library 1'),
  (2, 'available', 'Library 1'),
  (1, 'available', 'Library 2'),
  (2, 'available', 'Library 1'),
  (1, 'available', 'Library 1'),
  (2, 'available', 'Library 2'),
  (1, 'available', 'Library 1'),
  (3, 'available', 'Library 1');
