# CM0304 Save Editor

[Português (Brasil)](README.pt-BR.md)

Browse and edit **Championship Manager 03/04 4.1.5** saves on **Windows x64**. The editor lets you explore players, staff and clubs, prepare changes and write a validated copy of your career. It uses Rust for save handling and Tauri with TypeScript for the desktop interface.

## Download

Download the Windows x64 ZIP, extract it and run the installer. The editor uses Microsoft Edge WebView2. Windows SmartScreen may show a warning because the installer has no Authenticode signature. Use the published SHA-256 checksum to verify the ZIP. [Latest release and downloads](https://github.com/leandrolsouza/cm0304-save-editor/releases/latest)

The editor checks for signed updates at startup. If a career is open or an operation is under way when the download finishes, the update waits until you next open the editor.

## Using the editor

1. Select **Open save** and choose an uncompressed `.cm4` save from CM0304 4.1.5.
2. Explore **Players**, **Staff** or **Clubs**. **My club** opens automatically when the editor recognises a club managed by a human manager. Select **Add to review** to stage edits.
3. Open **Changes** to review original and proposed values and any derived effects. You can search the changes, return to the affected field, or undo and redo edits.
4. Select **Save validated copy** and choose a new filename. Use up to 26 characters before `.cm4`, the limit observed in the game. Load the copy in CM0304 to continue the career. To recover your original career, load the source save in the game.

The **Language** control switches between English and Brazilian Portuguese. Starting with v1.1.1, the editor defaults to English if you have yet to select a language. An explicit choice carries over when you update.

### Features

| Area | What you can do |
| --- | --- |
| Scouting | Filter players, save searches, keep watchlists, compare profiles and export CSV or JSON. Enter `uid:` followed by a number for an exact match. |
| Players | Edit attributes, ability and potential, positions, fitness, morale and supported injury and suspension records. Review derived effects when changing current ability or goalkeeper classification. |
| Contracts | Edit wages, expiry, squad status and shirt numbers on supported contracts. Owner and loan contracts can be reviewed separately. Some contracts support bonuses, release clauses and transfer status. |
| Staff | Browse club and national-team roles and edit supported professional attributes, ability, reputation and personality. |
| Clubs | Review squads and wages, inspect transfer instalments and sell-on clauses, and edit supported finances, facilities and stadiums. Compare squads across saves or simulate a change in the starting lineup. |
| Bulk edits | Preview changes, exclude individual players, reuse presets and review the resulting operation together. |
| Career copies | Save alternatives, reopen previously written copies and export a session plan. Import a plan of up to 16 MB with the matching source save. |

### Compatibility

The editor requires the uncompressed container and internal `game_db.dat` version 147 found in CM0304 4.1.5. Available fields depend on each save's record layout. For example, basic finance records can be read, while edits require a supported full finance record. Shared stadium capacities are available for inspection. The game may recalculate values after a career is loaded or advanced.

Edits are applied to a new file. The editor refuses to overwrite an existing destination or write against a source that changed since it was opened. It rejects attribute targets that have no exact representation, and changes to current ability require an explicit policy. [Save format research (Portuguese)](research/FORMAT.md) · [In-game validation (Portuguese)](reports/v0.5-matrix.md)

## Release notes

**v1.1.1** defaults to English, preserves explicit language preferences and ships with an English installation guide. The public documentation and bug report form are now in English, with a Brazilian Portuguese user guide still available.

**v1.1.0** keeps the active tab and scroll position when you add an edit to the review, shows draft and staged values on the profile, and lets you return directly to the relevant field from the review. You can remove individual players from a bulk preview and open the destination folder or history after saving a copy. This is the first public release with the signed updater. [GitHub Releases](https://github.com/leandrolsouza/cm0304-save-editor/releases)

## Development and support

Build and test from Windows with Node.js 22+, Rust and the Windows C++ build tools. [Development guide](docs/DEVELOPMENT.md) · [Roadmap](ROADMAP.md)

To report a problem, include the editor and game versions, the steps to reproduce it and what you expected. Please leave out personal information and game files. [GitHub Issues](https://github.com/leandrolsouza/cm0304-save-editor/issues)

An independent project distributed under the [MIT License](LICENSE).
