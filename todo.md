- [x] steamgriddb integration <-- The eponymous crate looks promising

- [x] better steamgriddb search <-- use steam grid, choose the game and not its
      name from dropdown
- [x] asynchronous downloads
- [x] asynchronous games stdio <-- tokio-stream is hell, use self-made
      "nonblocking" api via threads
- [ ] support for animated webp grids <-- See iced_gif
- [ ] cwd override
- [ ] game sorting
- [ ] tighter integration with existing runners
- [ ] store integration (itch, gog, steam, egs...)
- [ ] wine : per game prefix
- [ ] custom split dropdown widget
- [ ] runners as plugins
- [x] better theme-ing
- [ ] custom file dialog
- [ ] in-ui controller support
- [ ] \__GL_SHADER_DISK_CACHE
- [x] figure out the blur <-- was fixed by applying nearest instead of bilinear
      filtering on the images, thanks to PR 1894 on iced
- [ ] better file dialog (restart at last position) <-- further resarch suggest
      that we would have to write our own
- [x] remove unix features from w64 builds
- [x] replace subprocess with tokio processes (should fix process death issues)
- [ ] replace nosleep-nix dbus dependency with zbus

Ongoing tasks:

- More runners
  - [ ] Xemu
  - [ ] Cemu
  - [x] pcsx2
  - [ ] xenia
  - [ ] dolphin
  - [ ] libretro

Requires to wait third party:

- Rewrite UI code with whatever the Cosmic shell ends up with
- Better logs (require TextArea widget) also, perhaps using subscriptions
