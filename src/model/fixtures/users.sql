-- Add test users
INSERT INTO users (name, username, email, password, role)
VALUES
  ('John Doe', 'johndoe', 'johndoe@localhost', 'password123', 'member'),
  ('Jane Doe', 'janedoe', 'janedoe@localhost', 'password456', 'issuer'),
  ('Mark Smith', 'marks', 'marks@localhost', 'password789', 'admin');
