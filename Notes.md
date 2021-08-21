# Notes

-   Structs for the cli input and the data.
-   reqwest for the http calls with json

Notification schema:

repo_user/repo #issue_number
title issue - author - time

(?) maybe image of the users

```rust
async fn make_request() -> Result<(), Box<dyn std::error::Error>> {}
```

# Remember sáb 21 ago 2021 21:17:38

    1. Use Box<> structure to create a allocation on the heap to save the variable size of errors
    2. USER_AGENT lets servers identify the application
    3. serde_json::Value->enum to serialize arbitrary json structure
