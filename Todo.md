## Dev Todos sáb 21 ago 2021 23:28:31

- [x] Modularize show_notifications_cli
- [x] Create call for n.subject.url with the issue
- [x] Create function to calculate size of box and spaces with multiple data

- [ ] Better colors
- [ ] Add icons on the CLI
- [x] Add CLI commands

- [x] Add --help and --about

## Dev Todos dom 12 sep 2021

- [ ] Run in the brackground when making a subscription
  - Make a first call to that user profile
  - Save the ids of all the repos
  - Create a hashmap of ids as keys
  - Do calls every X time to check for new stuff
  - When a new repo appears notify the user

## Dev Todos

- i'm tired check the code Todos
- [ ] Cache
- [x] binary alias like `gtm` for `git-tellme`
- [x] Redo colors
- [x] Add notifications from Discussions

## Dev Todos dom 17 oct 2021 20:37:03 CEST

- [ ] Check `hyper` HTTP lib and if its better than `reqwest` do a migration
- [x] Ask for the token on start

## Dev todos dom 21 nov 2021 21:39:33 CET

- [ ] Fix error:

```bash
      ❯ cr -- -ru rddddddddddddddddddddd Finished dev [unoptimized + debuginfo] target(s) in 0.04s Running `target/debug/gtm
      -ru rddddddddddddddddddddd` Error: error decoding response body: invalid
      type: map, expected a sequence at line 1 column 0 Caused by: invalid type:
      map, expected a sequence at line 1 column 0
```
