- [x] steamgriddb integration                        <-- The eponymous crate looks promising
- [x] better steamgriddb search                      <-- use steam grid, choose the game and not its name from dropdown
- [x] asynchronous downloads
- [ ] asynchronous games stdio                       <-- tokio-stream is hell, use self-made "nonblocking" api via threads
- [ ] support for animated webp grids                <-- See iced_gif 
- [ ] cwd override
- [ ] game sorting
- [ ] tighter integration with existing runners
- [ ] store integration (itch, gog, steam, egs...)
- [ ] wine : per game prefix
- [ ] custom split dropdown widget
- [ ] runners as plugins
- [ ] better theme-ing
- [ ] custom file dialog
- [ ] in-ui controller support
- [ ] \__GL_SHADER_DISK_CACHE

Ongoing tasks:
- More runners
  |- [ ] Xemu
  |- [ ] Cemu
  |- [x] pcsx2
  |- [ ] xenia
  |- [ ] dolphin
  |- [ ] libretro

Requires to wait third party:
- Rewrite UI code with whatever the Cosmic shell ends up with
- Better logs (require TextArea widget) also, perhaps using subscriptions
