### Run

1. Run the server:

   ```bash
   cargo run
   ```

2. In your browser or via `curl`, you can now request a password:

    - **Default 32-character password:**

      ```bash
      curl http://127.0.0.1:8080/password
      ```

    - **Custom length (e.g., 16 characters):**

      ```bash
      curl "http://127.0.0.1:8080/password?length=16"
      ```
      
### Tests

Run all tests:

   ```bash
   cargo test
   ```
