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
    4. mod name_folder to get a folder/mod.rs and then use name_folder::file::struct;

# Remember dom 22 ago 2021 18:47:48

1. Change Type of returned json

```rust
fn main () {
    let html_url = if n.subject.subject_type == "Issue" {
        make_json_request(&url)
        .await?
        .json::<Issue>()
        .await?
        .html_url
    } else {
        make_json_request(&url)
        .await?
        .json::<PullRequest>()
        .await?
        .html_url
    };
}
```

Nice borders

<!-- local border = { -->
  <!-- { "🭽", "FloatBorder" }, -->

  <!-- { "▔", "FloatBorder" }, -->

  <!-- { "🭾", "FloatBorder" }, -->

  <!-- { "▕", "FloatBorder" }, -->

  <!-- { "🭿", "FloatBorder" }, -->

  <!-- { "▁", "FloatBorder" }, -->

  <!-- { "🭼", "FloatBorder" }, -->

  <!-- { "▏", "FloatBorder" }, -->
