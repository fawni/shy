# shy🍂

> A command line remote controller for MusicBee!

requires the MusicBee web server plugin [mb_WWWserver](https://github.com/fawni/mb_WWWserver) to be installed.

## Installation

```
cargo install --git https://github.com/fawni/shy
```

## Usage

```
A command line remote controller for MusicBee

Usage: shy.exe [COMMAND]

Commands:
  play        Play/Pause the current track [aliases: pause, p]
  stop        Stop playback [aliases: s]
  next        Play the next track in the queue [aliases: n]
  previous    Play the previous track in the queue [aliases: prev, b]
  add         Add track(s) to queue [aliases: a]
  clear       Clear current queue [aliases: c]
  nowplaying  Print information about the current track [aliases: np]
  queue       List queued tracks [aliases: q, list, ls, l]
  volume      Modify player volume [aliases: vol, v]
  seek        Seek playback
  shuffle     Change shuffle status
  repeat      Change repeat status [aliases: loop, r]
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Most subcommands have aliases which are the recommended way of usage.

> ###### In the following examples, | represents or.

```sh
shy np # print now playing
shy add track.mp3 track.flac path/to/album # adds track.mp3 track.flac and valid audio files in album directory to queue
shy v +10 | 80 | -40 # increase volume by 10 points | set volume to 80 | decrease volume by 40
shy seek 5 | 70% | -20% # seek 5 seconds | set position to 70% | go back 20%
```

## Implemented

- [x] Play/Pause
- [x] Stop
- [x] Next
- [x] Previous
- [x] Add to queue
- [x] Clear queue
- [x] Seek
- [x] Volume
- [x] Shuffle
- [x] Repeat
- [ ] Scrobble
- [x] Now playing
- [x] List queue
- [ ] ~~Album art to file~~ _(useless)_

## License

[OSL-3.0](LICENSE)
