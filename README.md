# rsnic
**rsnic** (pronounced "arsenic") is the **R**u**S**t **N**eat **I**nvidious **C**lient.

It is a CLI-frontend for [Invidious instances](https://invidio.us/) released under the [ISC License](https://www.isc.org/licenses/) (MIT/2-clause BSD equivalent).

![Screenshot](/docs/img/screenshot.png)

## Features
- [x] Search and play videos with a media player of your choice
- [ ] Search by uploader/channels
- [ ] Download videos/audio using a downloader of your choice
- [ ] Manage subscriptions from the CLI

## Installation
Cargo installation coming soon.

## Usage
Run `rsnic search [query]` to search, `rsnic -h` for help.

## Configuration
rsnic creates a configuration file on first run, on Linux this is `~/.config/rsnic/rsnic.toml`.
It looks like this by default:

```Toml
instance = 'https://invidious.snopyta.org'
player = 'mpv'
player_args = []
results = 20
```

### Settings
* `instance` is the Invidious instance to use (if you have connection issues try switching this!)
* `player` is the media player to use, the default of [mpv](mpv.io) is highly recommended.
* `player_args` is for media player arguments, for example `player_args = ['--no-audio']`.
* `results` is the number of results to display after a search, the maximum is 20.

## Alternatives
* [mps-youtube](https://github.com/mps-youtube/mps-youtube) (requires getting a YouTube API key?)
* [straw-viewer](https://github.com/trizen/straw-viewer) (has a GTK+ and CLI version)
* [Hosted Invidious instances](https://invidio.us/)
* [Minitube](https://flavio.tordini.org/minitube)
* [SMTube](https://sourceforge.net/projects/smtube/)
