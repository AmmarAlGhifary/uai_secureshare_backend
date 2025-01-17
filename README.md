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