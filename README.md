## Custom Github notifications cli

I want my own git notifications using the Github API without the browser.
For the moment I will return the data in the CLI. Maybe later I can put a frontend [dunst](https://github.com/dunst-project/dunst) style.

## Run this

First of all you will need a [personal access token from github](https://github.com/settings/tokens),
how to do get one [here](https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token).
Once you have that save it well. You only can see the token one time after its creation.
Save it well!

```bash
git clone https://github.com/marcelarie/git-tellme
cd git-tellme
echo AUTH_TOKEN='token YOUR_TOKEN_HERE' > .env
cargo install
cargo run
```

Or create a binary with `cargo build` and add it to your **$PATH**.

## Main Todos

-   [ ] Ask for username and authentication token or password and write it on the .env
-   [ ] Get notifications from the user
-   [x] Draw dynamic boxes for the CLI
-   [x] Open issue on the browser when clicking on the issue id ( on CLI)
-   [ ] Get user profile on the browser when clicking on the user.
-   [ ] Same with repo.
-   [ ] Create cronos to check for new repos from other users.
-   [ ] Frontend with Gtk (?)
-   [ ] Notification Bell 🔔
-   [ ] Work with FZF, FZY

### Using


[GitHub API](https://docs.github.com/en/rest)
[reqwest](https://crates.io/crates/reqwest)
[tokio](https://crates.io/crates/tokio)
