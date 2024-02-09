# shy🍂

A command line remote controller for MusicBee

<img src="./assets/tape.gif" width="85%"></img>

> [!IMPORTANT]
> shy requires the MusicBee web server plugin [mb_WWWserver](https://github.com/fawni/mb_WWWserver) to be able to run.

## Installation

```
cargo install --git https://github.com/fawni/shy
```

## Usage

> ###### In the following examples, | represents or

```sh
shy np # print now playing
shy add track.mp3 track.flac path/to/album # adds track.mp3 track.flac and valid audio files in album directory to queue
shy v +10 | 80 | -40 # increase volume by 10 points | set volume to 80 | decrease volume by 40
shy seek 5 | 70% | -20 # seek 5 seconds | set position to 70% | go back 20 seconds
```

`shy -h` for more information

## License

[OSL-3.0](LICENSE)
