# Mod Menu (native mod for Horizon)

A clean-room Horizon version of what the Java [Mod Menu](https://modrinth.com/mod/modmenu) gives you.
It contains no Mod Menu code.

## What it does

* **A Mods button on the title screen**, in the icon row after Accessibility (three bars, tooltip "Mods").
  It is the only way in: there is no key and no command.
* **A Mods screen laid out like the Java Mod Menu's**:
  * a search box (name, id and description) and a filter button (all, on, off, native, content),
  * "Showing N mods" and the list of every installed mod: icon, name, badges, description,
  * a detail pane for the picked mod: version, type, description and problems, with a **Turn on/off**
    button and a **Settings** button (the mod's own settings page),
  * **Open Mods Folder** and **Done** at the bottom.
* **Mod Menu's own entry** has its own icon (a blue list), is credited "By tariksabanuz" and has no
  Turn off or Settings: it can't be turned off from here and has no settings.

Keys on the screen: type to search, Up/Down/PageUp/PageDown/Home/End pick a mod, the wheel scrolls the list
(or the detail pane when the pointer is over it), Ctrl+V/C/X work in the search box, Escape leaves.

| Java Mod Menu | Here |
| --- | --- |
| Mods button in the main menu | yes, an icon button after Accessibility |
| Mods button in the pause menu | not here |
| Search, filter | yes |
| Mod icons | a coloured block per mod, and Mod Menu's own picture (Horizon mods have no icon files) |
| Badges (Client, Library, ...) | Native, Content, Dev, Off, Stopped, Not built |
| Config button per mod | the Settings button (native mods) |
| Website and Issues buttons, authors, license | not here: Horizon's mod manifests do not carry them |
| Update checker, library mods hidden | not here: the launcher's Mods page does updates |

## How it hooks in

The title screen and the Options screen are Horizon code with no mod hooks, so this mod uses
horizon-mixins HEAD hooks on them (all in `horizon-mixin.toml`):

* `Title::buttons`, `Title::click` and `Title::draw` add, press and draw the button (the icon row moves
  left half a button to stay centred);
* `Game::draw_options`, `options_mouse`, `options_drag`, `options_scroll`, `options_key` and
  `options_shortcut` hand the Options screen's drawing and input to the Mods screen while it is open;
* the named hooks `mods` and `frame` list the mod and clear the open flag when the Options screen is gone.

Because it hooks code that has no hook API, it is pinned to one Horizon revision and needs a rebuild
for another.

## Build and test

Pinned to Horizon `6475f62f3c834b7489e1816bf47f47ea5eb2b879` and horizon-mixins `d9c83d7`
(`compatibility.json`):

```sh
python3 tools/horizon.py stage --horizon ../horizon
python3 tools/horizon.py check --horizon ../horizon
python3 tools/horizon.py build --horizon ../horizon
python3 tools/package.py        # target/mod_menu-0.3.1.hnmod
```

The screen's logic (layout, search, filter, wrapping, keys) and its unit tests are in
`mixins/mod_menu/runtime.rs`; the title screen's shared state and icon are in `title_runtime.rs`; the drawing
and input are in `ui.rs`. License: MIT.
