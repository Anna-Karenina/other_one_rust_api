CREATE TABLE users_roles(
    id SERIAL PRIMARY KEY,
    user_id integer NOT NUll REFERENCES users(id),
    role_id integer NOT NULL REFERENCES roles(id)
)