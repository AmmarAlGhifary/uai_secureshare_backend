### Installation

1. Clone the repository:

   ```
   git clone https://github.com/AmmarAlGhifary/uai_secureshare_backend
   cd uai-secureshare-backend
   ```
2. Create a .env file in the root of the project with the following variables:

    ```
    # -----------------------------------------------------------------------------
    # Database (PostgreSQL)
    # -----------------------------------------------------------------------------
    DATABASE_URL=postgresql://username:password@localhost:5432/file_share

    # -----------------------------------------------------------------------------
    # JSON Web Token Credentials
    # -----------------------------------------------------------------------------
    JWT_SECRET_KEY=my_ultra_secure_jwt_secret_key
    JWT_MAXAGE=60
    ```

3. Install the necessary dependencies:

    ```
    cargo build
    ```

4. Run database migrations:

    ```
    sqlx migrate run
    ```

5. Start the server

    ```
    cargo run
    ```

## API Endpoints

- **POST /api/auth/register**: Register a new user.
- **POST /api/auth/login**: Login a user and return a JWT token.
- **GET /api/users/me**: Retrieve the authenticated user's information.
- **PUT /api/users/name**: Update the authenticated user's name.
- **PUT /api/users/password**: Change the authenticated user's password.
- **GET /api/users/search-emails**: Search for users by their email addresses.
- **POST /api/file/upload**: Upload a file (requires authentication).
- **GET /api/file/retrieve**: Retrieve an uploaded file by ID (requires authentication).
- **POST /api/list/send**: Send a list of files to another user.
- **GET /api/list/receive**: Retrieve the list of files received from another user.


```
uai_secureshare_backend
├─ .gitignore
├─ Cargo.lock
├─ Cargo.toml
├─ migrations
│  └─ 20241216104626_tables.sql
├─ README.md
└─ src
   ├─ config.rs          # Configuration settings for the app
   ├─ db.rs              # Database connection and setup
   ├─ dtos.rs            # Data Transfer Objects with validation
   ├─ error.rs           # Error handling logic
   ├─ handler
   │  ├─ auth.rs         # Authentication handlers
   │  ├─ file.rs         # File management handlers
   │  ├─ file_query.rs   # Handlers for querying files
   │  ├─ mod.rs          # Module for handler subdirectory
   │  └─ users.rs        # User management handlers
   ├─ main.rs            # Main entry point for the application
   ├─ middleware.rs      # Middleware logic for Axum
   ├─ models.rs          # Database models and schema
   ├─ router.rs          # Router definitions for Axum
   └─ utils
      ├─ decrypt.rs      # Decryption utilities
      ├─ encrypt.rs      # Encryption utilities
      ├─ keys.rs         # Key generation and management
      ├─ mod.rs          # Module for utils subdirectory
      ├─ password.rs     # Password hashing and verification
      └─ token.rs        # Token management utilities
```