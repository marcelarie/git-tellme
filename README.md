## git-tellme: github notifications for the CLI

I want my own git notifications using the Github API without the browser.

For the moment I will return the data to the terminal. Maybe later I can put a
frontend [dunst](https://github.com/dunst-project/dunst) style. The project is
still a **work in progress**. The main idea its to have a small program to
subscribe to github user repositories, user profiles and manage all your
notifications from the comfort of your terminal. Subscribe to users/projects to
get notified when a new repository is created will be possible too.

For the terminal fonts with ligatures like
[FiraCode](https://github.com/tonsky/FiraCode) are recommended.

#### Main commands

To get your notifications:

```bash
$ git-tellme # or git-tellme -n
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ Issue                                                                  ┃
┃ Protocol error - Target closed: puppeteer stopped working.             ┃
┃ puppeteer/puppeteer https://github.com/puppeteer/puppeteer/issues/7455 ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
# click on the link to open the issue
```

To get your repositories:

```bash
$ git-tellme -r
┏━━━━━━━━━━━━┓
┃ 398634567  ┃
┃ git-tellme ┃
┗━━━━━━━━━━━━┛
```

To get someone else repositories:

```bash
$ git-tellme -ru rylev
┏━━━━━━━━━━━━━┓
┃ 336591832   ┃
┃ const-utf16 ┃
┗━━━━━━━━━━━━━┛
┏━━━━━━━━━━━┓
┃ 27019437  ┃
┃ coreutils ┃
┗━━━━━━━━━━━┛
```

```bash
$ git-tellme -h
git-tellme 0.1.0

USAGE:
    git-tellme [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                 Prints help information
    -n, --get-notifications    Get Github user notifications
    -r, --repos                Get Github user repositories
    -s, --subscribe            (WIP)
    -V, --version              Prints version information

OPTIONS:
    -u, --user <user>          (WIP) Select specific user profile
```

If you want to get your notifications when you open your terminal you can add
`git-tellme` to you're `~/.bashrc`, `~/.zshrc` or `~/.config/fish/config.fish`

```bash
echo 'git-tellme' >> ~/.bashrc
```

The performance will get better, I promise.

## Before you start

##### 1. First of all you will need a [personal access token from github](https://github.com/settings/tokens).

-   How to do get one
    [here](https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token),
    you only can see the token one time after its creation.

`git-tellme` uses the GitHub API so that token has to be accesible for the whole
system. The best way to do that is with `pass` and `gpg`.
[Here](https://www.thepolyglotdeveloper.com/2018/12/manage-passwords-gpg-command-line-pass/)
you have a better explanation but basically you will need to run this commands:

```bash
gpg --gen-key
# Identify and create your database
# ...
gpg --list-keys
# The command will return something like this
# /home/user/.gnupg/pubring.gpg
# ------------------------------
# pub   4096R/68214821 2015-06-24
# uid                  First Middle Last Suffix <first.last@host.tld>
# sub   4096R/36A6F06D 2015-06-24
pass init 68214821 # <-- use the pub key to initialize pass
# mkdir: created directory ‘/home/user/.password-store/’
# Password store initialized for 68214821
```

##### Extra: I really recommend to create a `~/.gnupg/gpg-agent.conf` file with this content inside:

So the gpg-agent stops asking every second for the master password.

```bash
default-cache-ttl 34560000
max-cache-ttl 34560000
```

##### 2. Then to save your GitHub token:

```bash
pass insert -m git-tellme/token # <-- IMPORTANT: the name needs to be git-tellme/token
Enter contents of g and press Ctrl+D when finished:
# PASTE_YOUR_TOKEN_HERE
```

To learn more about `pass`: https://wiki.archlinux.org/title/Pass  
To learn more about `gpg`: https://www.gnupg.org/gph/en/manual/c14.html

I know, too much ? This will be easier later, give me time...

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
PATH=$PATH:`{pwd}`/target/release/git-tellme
```

## Commands

## Main Todos

-   [ ] Ask for username and authentication token or password and write it on
        the .env
-   [x] Get notifications from the user
-   [x] Draw dynamic boxes for the CLI
-   [x] Open issue on the browser when clicking on the issue id ( on CLI)
-   [ ] Get user profile on the browser when clicking on the user.
-   [ ] Same with repos.
-   [ ] Create cronos to check for new repos from other users.
-   [ ] Frontend with Gtk (?)
-   [ ] Notification Bell 🔔
-   [ ] Work with FZF, FZY

### Using

-   [GitHub API](https://docs.github.com/en/rest)
-   [reqwest](https://crates.io/crates/reqwest)
-   [tokio](https://crates.io/crates/tokio)
-   [pass](https://www.passwordstore.org/)
-   [gnuPG](https://gnupg.org/)
