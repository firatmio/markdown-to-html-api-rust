# Markdown → HTML API (Rust)

![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)  ![License](https://img.shields.io/badge/License-MIT-green.svg)

---

## Overview

A minimal **Markdown to HTML conversion service** built with Rust and Actix-web.
Users can send Markdown text via API and receive rendered HTML instantly. Includes a simple frontend for live preview.

---

## Features

* Convert Markdown to HTML via POST `/render`
* Health check endpoint GET `/health`
* Live preview frontend served from the server
* Optional syntax highlighting for code blocks (can be added)
* Minimal dependencies, lightweight and fast

---

## Technologies Used

* **Backend:** Rust, Actix-web
* **Markdown Parsing:** Comrak
* **Static Files / Frontend:** Vanilla HTML + JS served via Actix-files
* **Logging:** env\_logger

---

## Getting Started

### Prerequisites

* Rust (latest stable recommended)
* Cargo (comes with Rust)

---

### Installation & Running

1. Clone the repository:

```bash
git clone <repo-url>
cd markdown-to-html-api
```

2. Run the server in development mode:

```bash
cargo run
```

3. Access frontend:

```
http://127.0.0.1:8080
```

4. Access API endpoint:

```bash
POST http://127.0.0.1:8080/render
Content-Type: application/json
Body: { "markdown": "# Hello\nSome **Markdown**" }
```

---

## Example Usage

Open the frontend in a browser, write Markdown in the left textarea, and see live HTML preview on the right. No button click required; it updates automatically.

API usage example via cURL:

```bash
curl -X POST http://127.0.0.1:8080/render \
  -H "Content-Type: application/json" \
  -d '{"markdown": "# Hello\n\nThis is **Markdown**!"}'
```

---

## Project Structure

```
markdown-to-html-api/
├── src/
│   └── main.rs          # Main Rust backend file
├── frontend/            # HTML + JS live preview
│   └── index.html
├── Cargo.toml            # Rust dependencies
├── Dockerfile            # Optional containerization
└── README.md             # Project documentation
```

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
