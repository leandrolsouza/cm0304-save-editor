# Development handover

## Current release

The first public release is **v1.1.0** for Windows x64. Its GitHub Actions workflow builds the NSIS package, ZIP checksum, updater signature and manifest. The native update test used a separate 1.1.1 test build and confirmed that an open career defers the update and that the restart preserves the save and theme. [Release](https://github.com/leandrolsouza/cm0304-save-editor/releases/tag/v1.1.0) · [Release procedure](DEVELOPMENT.md#release-on-github)

Starting with **v1.1.1**, the editor defaults to English when no language has been selected. Existing explicit choices still apply. The installation guide and public documentation are in English.

## Next validation work

- Run the editing workflow on an independent career with a different season or league setup. Verify whether loading, advancing and resaving preserve the intended result in the game.
- Connect profile filters, comparison and watchlists into one scouting workflow. Verify positional Scout Rating against the reference before using it for result ordering.
- Keep unverified record variants in research. Capture the precise field or link involved, the expected and observed values, and evidence that the source was preserved.

[Roadmap](../ROADMAP.md) · [Historical handover (Portuguese)](NEXT.pt-BR.md) · [Save format research (Portuguese)](../research/FORMAT.md)
