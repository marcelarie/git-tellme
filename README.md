## git-tellme: github notifications for the CLI

I want my own git notifications using the Github API without the browser.

For the moment I will return the data to the terminal. Maybe later I can put a
frontend [dunst](https://github.com/dunst-project/dunst) style. The project is
still a **work in progress**. This is a small program to subscribe to github
user repositories, user profiles and manage all your notifications from the
comfort of your terminal.

For the terminal fonts with ligatures like
[FiraCode](https://github.com/tonsky/FiraCode) are recommended.

## Installation

**First install `rustup`+`cargo`:** https://rustup.rs/

From crates.io:

```bash
cargo install git-tellme
```

From the repository source:

```bash
git clone https://github.com/marcelarie/git-tellme
cd git-tellme
echo AUTH_TOKEN='token YOUR_TOKEN_HERE' > .env
cargo install
cargo build --release
PATH=$PATH:`{pwd}`/target/release/gtm
```

#### Main commands

To get your notifications on the desktop run `gtm -f` on the background with `&`
or create a daemon. For the moment it just listens all the time for
notifications, non stop. And they are persistent. On click the notification will
be opened with `xdg-open` ( your default system browser ):

```bash
$ gtm -f &
```

To get your notifications on the CLI:

```bash
$ gtm # or gtm -n
┌─────────────────────────────────────────────────────────────┐
│ treesitter/highlighter.lua:145: end_col value outside range │
│ https://github.com/neovim/neovim/issues/12861               │
└─────────────────────────────────────────────────────────────┘
# click on the link to open the issue
```

To get your repositories:

```bash
$ gtm -r
┌──────────────────────────────────────────┐
│ git-tellme                               │
│ https://github.com/marcelarie/git-tellme │
└──────────────────────────────────────────┘
```

To get someone else repositories:

```bash
$ gtm -ru rylev
┌──────────────────────────────────────┐
│ const-utf16                          │
│ https://github.com/rylev/const-utf16 │
└──────────────────────────────────────┘
┌────────────────────────────────────┐
│ coreutils                          │
│ https://github.com/rylev/coreutils │
└────────────────────────────────────┘
```

To get help:

```bash
$ gtm -h
git-tellme 0.1.6

USAGE:
    git-tellme [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                 Prints help information
    -n, --get-notifications    Get Github user notifications
    -r, --repos                Get Github user repositories
    -s, --subscribe
    -f, --system               Show notifications on the system
    -V, --version              Prints version information

OPTIONS:
    -t, --token <github-token>    Pass `-t` and you're GitHub token from
                                  https://github.com/settings/tokens
    -u, --user <user>             Select specific user profile
```

If you want to get your notifications when you open your terminal you can add
`git-tellme` to you're `~/.bashrc`, `~/.zshrc` or `~/.config/fish/config.fish`

```bash
echo 'gtm' >> ~/.bashrc
```

## Before you start

##### 1. First of all you will need a [personal access token from github](https://github.com/settings/tokens).

- How to do get one
  [here](https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token),
  you only can see the token one time after its creation.

`git-tellme` uses the GitHub API so that token has to be accessible for the
whole system. At the start `git-tellme` will ask for your token if it does not
find it:

```bash
$ gtm
<WARNING>
    No GitHub token was found.
    To generate one go to: https://github.com/settings/tokens
    To save it use the --token or -t parameter:

gtm --token <YOUR_GITHUB_TOKEN>

For more information try --help
```

You just need to paste the token and done :)

## Main Todos

- [x] Ask for username and authentication token or password and write it on
      redis
- [x] Get notifications from the user
- [x] Draw dynamic boxes for the CLI
- [x] Open issue on the browser when clicking on the issue id ( on CLI)
- [ ] Get user profile on the browser when clicking on the user.
- [ ] Same with repos.
- [ ] Create cronos to check for new repos from other users.
- [ ] Notification Bell 🔔
- [ ] Work with FZF, FZY

### Using

- [GitHub API](https://docs.github.com/en/rest)
- [reqwest](https://crates.io/crates/reqwest)
- [tokio](https://crates.io/crates/tokio)
