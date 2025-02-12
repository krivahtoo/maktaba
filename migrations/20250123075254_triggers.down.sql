-- Drop trigger for BookCopies
DROP TRIGGER IF EXISTS generate_copy_id;

-- Drop 'updated_at' triggers
DROP TRIGGER IF EXISTS update_users_timestamp;
DROP TRIGGER IF EXISTS update_books_timestamp;
DROP TRIGGER IF EXISTS update_book_copies_timestamp;
DROP TRIGGER IF EXISTS update_borrowing_timestamp;
DROP TRIGGER IF EXISTS update_fines_timestamp;
DROP TRIGGER IF EXISTS update_categories_timestamp;
DROP TRIGGER IF EXISTS update_reviews_timestamp;
DROP TRIGGER IF EXISTS update_reservations_timestamp;

-- Drop trigger
DROP TRIGGER IF EXISTS update_borrowing_return_and_status;
