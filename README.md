# termban

A terminal Kanban board for tracking multiple projects, built with
[`ratatui`](https://github.com/ratatui-org/ratatui) and
[`crossterm`](https://github.com/crossterm-rs/crossterm).

Each project has three columns — **Not Done**, **In Progress**, and **Done**
— and your board is saved automatically to disk, so it's there next time you
open it.

## Install / Run

Requires the Rust toolchain ([rustup.rs](https://rustup.rs)).

```bash
git clone https://github.com/rafidahmed816/termban.git
cd termban
cargo run --release
```

## Keybindings

| Key         | Action                                       |
|-------------|-----------------------------------------------|
| `←` / `→`   | Switch project                                |
| `Tab`       | Switch column                                 |
| `↑` / `↓`   | Move the task cursor within the column        |
| `t`         | Add a task to the current column              |
| `p`         | Add a new project                             |
| `d`         | Delete the current project                    |
| `x`         | Delete the highlighted task                   |
| `[` / `]`   | Move the highlighted task to the prev/next column |
| `s`         | Cycle sort order (alphabetical / recently updated / task count) |
| `Enter`     | Confirm text input (when adding a task/project) |
| `Esc`       | Cancel text input                             |
| `q`         | Quit                                          |

## Data storage

Termban stores your projects in `$XDG_DATA_HOME/termban/projects.json` (or `~/.local/share/termban/projects.json` if `XDG_DATA_HOME` is unset). The
file is written after every change, so you shouldn't lose data even if the
app exits unexpectedly.

## License

MIT — see [LICENSE](LICENSE).