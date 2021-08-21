-   Structs for the cli input and the data.
-   reqwest for the http calls with json

Notification schema:

repo_user/repo #issue_number
title issue - author - time

(?) maybe image of the users

1. Use Box<> structure to create a allocation on the heap to save the variable size of errors

```rust
async fn make_request() -> Result<(), Box<dyn std::error::Error>> {}
```
