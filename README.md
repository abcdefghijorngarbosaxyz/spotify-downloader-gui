<h1 align="center">
  <br />
  <img width="256px" height="256px" src="assets/icons/128x128@2x.png" />
  <br />
  spotDL GUI (Early Development)
  <br />
  <br />
  
[![wakatime](https://wakatime.com/badge/user/41c7b3a0-cdec-4bfa-878d-91749cfc2273/project/cc0f66d6-62aa-40c4-9827-90326d664d19.svg)](https://wakatime.com/badge/user/41c7b3a0-cdec-4bfa-878d-91749cfc2273/project/cc0f66d6-62aa-40c4-9827-90326d664d19)

</h1>

<p align="center">
Download your favorite Spotify songs and playlists, along with album art and metadata (from YouTube if a match is found), in a GUI!
</p>

<br />
<br />

## Features & TODOs

<details>
  <summary>App</summary>

* [ ] Use Spotify API
  * [ ] Search
    * [ ] By title
    * [ ] By artist
    * [ ] By playlist
    * [ ] By album
  * [ ] Download
    * [ ] By song
    * [ ] By artist
    * [ ] By playlist
    * [ ] By album
  * [ ] Recommendations (User API key)
* [ ] spotDL integration
  * [ ] Start app with `spotdl app`
  * [x] spotDL binaries sidecar
    * [x] All platforms
      * [x] Windows
      * [x] Darwin
      * [x] Linux
  * [ ] spotDL auth options (optional)
    * [ ] `--user-auth`
    * [ ] `--client-id`
    * [ ] `--client-secret`
    * [ ] `--auth-token`
  * [ ] spotDL download options
    * [ ] `--audio-providers`
    * [ ] `--lyrics-providers`
    * [ ] `--output`
    * [ ] `--output-dir`
    * [ ] `--bitrate`
    * [ ] `--threads`
    * [ ] `--format`
    * [ ] `--save-file`
    * [ ] `--preload`
    * [ ] `--m3u`
    * [ ] `--overwrite`
    * [ ] `--ytm-data`
    * [ ] `--generate-lrc`
    * [ ] `--force-update-metadata`
* [ ] Standalone (package spotDL binaries, and ffmpeg)
* [ ] Dependant (use installed spotDL CLI, and ffmpeg)
  * [ ] Check if spotDL installed, download if not
  * [ ] Check if ffmpeg installed, download if not
* [ ] Views
  * [x] Custom titlebar in Windows/Linux
  * [ ] Accelerators (macro shortcuts)
  * [ ] Window menu
    * [x] File
      * [x] Open download folder
      * [x] Select download folder
      * [x] Exit
    * [x] View
      * [x] Toggle developer tools
    * [x] Window
      * [x] Minimize
      * [x] Always on top
    * [ ] Help
      * [x] Documentations
      * [ ] Release notes
      * [x] Report issue
      * [x] Join us on Discord
      * [ ] Check for updates
      * [x] About
  * [x] Custom "about" window
  * [ ] Sidenavbar
  * [ ] Search page
  * [ ] Download queue page
  * [ ] Context menu

</details>

<details>
  <summary>Documentation</summary>

* [ ] Installation
* [ ] Usage
* [ ] FAQ
* [ ] Open-source notices
* [ ] Code documentation (JSDoc, Rust)
* [ ] Contributing guidelines
* [ ] Github wiki
</details>

<details>
  <summary>Workflows</summary>

* [ ] Release
* [ ] Format code on dev push
</details>

<br />
Details are subject to change over time.

## Notice

To avoid legal issues with Spotify's Digital Rights Management techonology, spotDL downloads songs from YouTube, YouTube Music, and other supported music providers.

## Supported Platforms

| Platform     | Versions    |
| :----------- | :---------- |
| Windows      | 7 and above |
| macOS (todo) |             |
| Linux (todo) |             |

## The Logo

An arrow down (representing a download symbol), on fire, in YouTube red colors, because spotDL mainly downloads from YouTube/YTMusic, with the Spotify symbols inside.

<b>It's not a fish!</b>

## Semver

spotDL GUI is following Cargo's [Semantic Versioning](https://doc.rust-lang.org/cargo/reference/semver.html#semver-compatibility)

## Contributing

See [Contributing](Contributing.md)

## License

[MIT](LICENSE)
