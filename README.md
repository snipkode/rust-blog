# Blog Mini Rust

Aplikasi blog mini fullstack yang dibuat menggunakan **Rust**, **Axum**, **SeaORM**, dan **MySQL**. Proyek ini memiliki backend yang tangguh dengan autentikasi JWT dan rendering sisi server menggunakan template **Tera**.

## Fitur

- **Operasi CRUD Lengkap:** Membuat, Membaca, Memperbarui, dan Menghapus postingan blog.
- **Autentikasi JWT:** Sistem registrasi dan login pengguna yang aman menggunakan JSON Web Tokens yang disimpan di dalam cookie `HttpOnly`.
- **Hashing Password:** Penyimpanan password yang aman menggunakan `bcrypt`.
- **Migrasi Database:** Manajemen skema database otomatis dengan migrasi SeaORM.
- **Antarmuka Modern:** Desain bersih dan responsif yang dibuat dengan Vanilla CSS.
- **Inti Asinkron:** Menggunakan `Tokio` untuk eksekusi asinkron berperforma tinggi.

## Teknologi Utama

- **Backend:** [Axum](https://github.com/tokio-rs/axum)
- **ORM Database:** [SeaORM](https://www.sea-ql.org/SeaORM/)
- **Database:** MySQL
- **Templating:** [Tera](https://tera.netlify.app/)
- **Autentikasi:** `jsonwebtoken` & `bcrypt`

## Prasyarat

- [Rust](https://www.rust-lang.org/tools/install) (versi stabil terbaru)
- Server [MySQL](https://www.mysql.com/) yang sedang berjalan
- [GitHub CLI](https://cli.github.com/) (opsional, untuk manajemen repositori)

## Persiapan & Instalasi

1. **Clone repositori:**
   ```bash
   git clone https://github.com/snipkode/rust-blog.git
   cd rust-blog
   ```

2. **Konfigurasi Variabel Lingkungan:**
   Buat file `.env` di direktori root (atau edit yang sudah ada):
   ```env
   DATABASE_URL=mysql://username:password@localhost:3306/rust_blog
   HOST=127.0.0.1
   PORT=3000
   JWT_SECRET=kunci-jwt-rahasia-anda
   COOKIE_SECRET=rahasia-cookie-minimal-64-karakter
   ```

3. **Pengaturan Database:**
   Pastikan server MySQL Anda berjalan dan buat database baru:
   ```sql
   CREATE DATABASE rust_blog;
   ```

4. **Jalankan Aplikasi:**
   ```bash
   cargo run
   ```
   Aplikasi akan menjalankan migrasi secara otomatis dan memulai server di `http://127.0.0.1:3000`.

## Struktur Proyek

- `src/`: Logika utama aplikasi.
    - `handlers/`: Handler rute untuk autentikasi dan postingan blog.
    - `entities/`: Model database SeaORM.
    - `app_state.rs`: State aplikasi yang digunakan bersama.
- `migration/`: Skrip migrasi database.
- `templates/`: Template HTML (Tera).
- `static/`: Aset statis (CSS).

## Alur Autentikasi

1. **Daftar:** Buat akun baru di `/register`.
2. **Login:** Masuk di `/login`. JWT akan dibuat dan disimpan dalam cookie yang aman.
3. **Akses Terproteksi:** Rute seperti `/post/new`, `/post/:id/edit`, dan `/post/:id/delete` diproteksi dan memerlukan JWT yang valid.
4. **Logout:** Hapus sesi melalui `/logout`.

## Lisensi

Didistribusikan di bawah Lisensi MIT. Lihat `LICENSE` untuk informasi lebih lanjut.

---
Dibuat menggunakan Rust.
