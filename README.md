# Rust Mini Blog

A fullstack mini-blog application built with **Rust**, **Axum**, **SeaORM**, and **MySQL**. This project features a robust backend with JWT authentication and server-side rendering using **Tera** templates.

## 🚀 Features

- **Full CRUD Operations:** Create, Read, Update, and Delete blog posts.
- **JWT Authentication:** Secure user registration and login system using JSON Web Tokens stored in `HttpOnly` cookies.
- **Password Hashing:** Secure password storage using `bcrypt`.
- **Database Migrations:** Automatic database schema management with SeaORM migrations.
- **Modern UI:** Clean and responsive design built with Vanilla CSS.
- **Asynchronous Core:** Powered by `Tokio` for high-performance async execution.

## 🛠️ Tech Stack

- **Backend:** [Axum](https://github.com/tokio-rs/axum)
- **Database ORM:** [SeaORM](https://www.sea-ql.org/SeaORM/)
- **Database:** MySQL
- **Templating:** [Tera](https://tera.netlify.app/)
- **Authentication:** `jsonwebtoken` & `bcrypt`

## 📋 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [MySQL](https://www.mysql.com/) server running
- [GitHub CLI](https://cli.github.com/) (optional, for repository management)

## ⚙️ Setup & Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/snipkode/rust-blog.git
   cd rust-blog
   ```

2. **Configure Environment Variables:**
   Create a `.env` file in the root directory (or edit the existing one):
   ```env
   DATABASE_URL=mysql://username:password@localhost:3306/rust_blog
   HOST=127.0.0.1
   PORT=3000
   JWT_SECRET=your-super-secret-jwt-key
   COOKIE_SECRET=your-at-least-64-character-long-cookie-secret
   ```

3. **Database Setup:**
   Ensure your MySQL server is running and create the database:
   ```sql
   CREATE DATABASE rust_blog;
   ```

4. **Run the Application:**
   ```bash
   cargo run
   ```
   The application will automatically run migrations and start the server at `http://127.0.0.1:3000`.

## 📂 Project Structure

- `src/`: Main application logic.
    - `handlers/`: Route handlers for auth and blog posts.
    - `entities/`: SeaORM database models.
    - `app_state.rs`: Shared application state.
- `migration/`: Database migration scripts.
- `templates/`: HTML templates (Tera).
- `static/`: Static assets (CSS).

## 🛡️ Authentication Flow

1. **Register:** Create a new account at `/register`.
2. **Login:** Authenticate at `/login`. A JWT is generated and stored in a secure cookie.
3. **Authorized Access:** Routes like `/post/new`, `/post/:id/edit`, and `/post/:id/delete` are protected and require a valid JWT.
4. **Logout:** Clear the session at `/logout`.

## 📜 License

Distributed under the MIT License. See `LICENSE` for more information.

---
Built with ❤️ using Rust.
