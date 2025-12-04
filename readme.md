# Netflix Clone Backend (Rust + Axum)

This is a high-performance REST API built with **Rust** and **Axum 0.8**. It serves as the backend for a Netflix Clone frontend, handling data fetching from TMDB (The Movie Database) and local video streaming.

## 🚀 Features

* **Blazingly Fast:** Built on Rust and Tokio.
* **Modular Architecture:** Clean separation of concerns (Handlers, Models, State).
* **TMDB Integration:** Fetches Trending Movies, Search Results (Movies/TV), and Trailers.
* **Video Streaming:** Supports HTTP Range Requests (Status 206) for smooth video playback.
* **CORS Enabled:** Configured to work with React/Vite frontends.

---

## 🛠️ Prerequisites

Before starting, ensure you have **Rust** installed: https://rust-lang.org/tools/install/

📂 Project Structure

```
netflix_backend/
├── assets/          # Stores local video files (ignored by Git)
├── src/
│   ├── main.rs      # Entry point & Router configuration
│   ├── handlers.rs  # API Route Logic
│   ├── models.rs    # Data Structs (Movie, Video, etc.)
│   └── state.rs     # Shared Application State (API Key, HTTP Client)
├── .env             # Environment variables
├── .gitignore       # Git configuration
└── Cargo.toml       # Rust Dependencies
```
<!--  -->
Environment Variables
Create a .env file in the root directory. This file is mandatory to configure the API keys and server settings.

```env
touch .env
```

Open the .env file and paste the following configuration:
```env
TMDB_API_KEY=your_tmdb_api_key_here
HOST=127.0.0.1
PORT=8080
```

Important Notes:

TMDB_API_KEY: You can get a free key at themoviedb.org.

PORT: We use 8080 to avoid conflicts with the React Frontend (which typically runs on port 3000).

Setup Streaming Assets
```
mkdir assets
```

Download a sample video file (e.g., video.mp4) and place it in the assets/ directory.

Start the server in development mode:

Bash
```
cargo run
```
You should see the following output indicating the server is active:

```
...
Server listening on [http://127.0.0.1:8080](http://127.0.0.1:8080)
```
📡 API Reference
Here are the available endpoints. You can test them using curl or directly in your browser.

1. Trending Movies
Fetches the weekly trending movies and TV shows from TMDB.
- URL: GET /api/trending
- Query Params: ?page=1 (optional)

```
curl http://localhost:8080/api/trending
```
2. Search
Performs a multi-search for Movies and TV Shows.

- URL: GET /api/search
- Query Params: ?query=your_search_term

```
curl "http://localhost:8080/api/search?query=matrix"
```

3. Get Trailers
Fetches YouTube trailer keys for a specific movie ID.
- URL: GET /api/movie/{id}/videos


# Example for "The Matrix" (ID: 603)

```
curl http://localhost:8080/api/movie/603/videos
```
4. Video Streaming
Streams a local video file from the assets folder using HTTP Range Requests (enabling seeking).


URL: GET /stream/{filename}

# Test with curl (Head request to check 200/206 status)
```
curl -I http://localhost:8080/stream/video.mp4
```

# Or open in browser:
http://localhost:8080/stream/video.mp4

