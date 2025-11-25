# WWM Overlay - MIDI Music Player (21 Keys)
A beautiful music player for Where Winds Meet that plays your MIDI files by automatically pressing the right keyboard keys in-game.

<img width="1125" height="620" alt="Screenshot 2025-11-25 105732" src="https://github.com/user-attachments/assets/3884a12e-3950-410a-b48d-44756581b159" />

https://github.com/user-attachments/assets/af8d0f3d-3985-4db3-b6cf-bea16f4cb2b2


## What is this?

This app lets you play music in Where Winds Meet's music minigame! Just add your MIDI files, click play, and the app will automatically press the keyboard keys for you. It's like having an auto-play feature for the in-game instrument.

## Features

- **Beautiful Spotify-style interface** - Dark theme with smooth animations
- **Easy to use** - Just drag your MIDI files and click play
- **Smart music selection** - Automatically adjusts notes to fit the game's instrument
- **Queue system** - Build your playlist and play songs in order
- **Favorites** - Mark your favorite songs for quick access
- **Multiple playlists** - Create, rename, and manage custom playlists
- **Drag & drop reordering** - Reorder songs in queue and playlists
- **Real-time progress** - See exactly where you are in the song
- **Global hotkeys** - Control playback from anywhere, even when the game is focused
- **Song duration display** - See how long each MIDI file is

## How to Use

### First Time Setup

1. **Download the app** - Get the latest release from the releases page
2. **Extract the files** - Unzip to any folder you like
3. **Add your MIDI files** - Place your `.mid` files in the `album` folder
4. **Run the app** - Double-click `wwm-overlay.exe`

### Playing Music

1. **Open the game** - Launch Where Winds Meet and open the music minigame
2. **Select a song** - In the app, click on any song in your library
3. **Add to queue** - Click the playlist icon to add songs to your queue or playlists
4. **Play** - Click the play button (or press F9)
5. **Focus the game** - The app will automatically switch focus to the game
6. **Enjoy!** - The music will play automatically

### Keyboard Shortcuts (Global Hotkeys)

These shortcuts work even when the game is focused:

| Key | Action |
|-----|--------|
| **F9** | Play / Pause |
| **F10** | Previous track |
| **F11** | Next track |
| **F12** | Stop |
| **End** | Stop (alternative) |
| **Ctrl+L** | Toggle loop mode |
| **Ctrl+P** | Previous track (alternative) |
| **Ctrl+N** | Next track (alternative) |

### In-App Controls

- **Click any song** to start playing
- **Heart icon** - Add/remove from favorites
- **Playlist icon** - Add to queue or saved playlists
- **Drag handle** (top of sidebar) - Move the window around
- **Play/Pause button** - Control playback at the bottom
- **Loop button** - Toggle repeat mode

### Managing Playlists

1. Go to the **Playlists** tab in the sidebar
2. Click **New** to create a playlist
3. Name your playlist and click Create
4. Add songs from the library using the playlist icon
5. Click on a playlist to view and manage its songs
6. **Drag songs** to reorder them
7. **Click X** on a song to remove it from the playlist
8. **Click Play** to load the playlist to queue and start playing

### Tips

- **Finding MIDI files**: Search online for "song name midi" or "song name .mid"
- **Song not playing right?**: The app automatically adjusts the notes, but some complex songs might not work well with the game's limited keyboard layout
- **Multiple songs**: Add multiple songs to your queue for a continuous playlist
- **Searching**: Use the search box to quickly find songs in your library
- **Favorites**: Click the heart icon to quickly access your favorite songs later

## Troubleshooting

**The app won't focus the game**
- Make sure Where Winds Meet is running
- The game window must be visible (not minimized)
- Try clicking on the game first, then press play

**Hotkeys not working**
- Some hotkeys may conflict with other applications
- F12 is commonly used by browsers (dev tools) - try using End instead for stop
- Make sure the app is running (check system tray)

**Music sounds wrong**
- The game only has 21 keys (3 octaves), so some complex songs won't sound perfect
- The app tries its best to fit the notes into the available range
- Try different MIDI files to see what works best

**Songs not showing up**
- Make sure your MIDI files are in the `album` folder
- Files must have the `.mid` extension
- Click the refresh button in the sidebar to reload the list

**Progress bar jumps around**
- This can happen if multiple playback sources conflict
- Try stopping and restarting the song

## Where to Put MIDI Files

```
wwm-overlay/
├── wwm-overlay.exe
├── album/              <- Put your .mid files here!
│   ├── song1.mid
│   ├── song2.mid
│   └── song3.mid
└── ...
```

## Support

Having issues? Here are some things to try:

1. Restart the app
2. Make sure the game is running
3. Check that your MIDI files are valid
4. Try a different MIDI file to see if the issue is file-specific

## Building from Source

If you want to build the app yourself:

1. **Install dependencies**:
   - [Rust](https://www.rust-lang.org/tools/install)
   - [Node.js](https://nodejs.org/) or [Bun](https://bun.sh/)

2. **Clone the repository** and navigate to the folder

3. **Install packages**:
   ```bash
   npm install
   # or
   bun install
   ```

4. **Run in development mode**:
   ```bash
   npm run tauri-dev
   # or
   bun run tauri-dev
   ```

5. **Build for release**:
   ```bash
   npm run tauri-build
   # or
   bun run tauri-build
   ```

6. **Find the executable**: After building, the app will be in `src-tauri/target/release/`

7. **Create album folder**: Make sure to create an `album` folder next to the `.exe` file and add your MIDI files there

## Credits

Built with:
- Tauri (desktop app framework)
- Svelte (user interface)
- Rust (backend and MIDI processing)

Music icon: Material Design Icons

Created by YueLyn

---

Enjoy making music in Where Winds Meet!
