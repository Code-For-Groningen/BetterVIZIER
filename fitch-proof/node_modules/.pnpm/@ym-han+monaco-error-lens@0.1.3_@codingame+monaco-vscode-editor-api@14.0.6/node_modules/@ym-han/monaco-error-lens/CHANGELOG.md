# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **BREAKING**: Constructor signature updated to require Monaco module reference
- Removed dependency on global `window.monaco` object
- All factory functions now require `monaco` parameter

### Added
- `MonacoModule` interface for Monaco namespace typing
- API documentation for `MonacoEditor` vs `MonacoModule` interfaces
- Better dependency injection pattern
- Improved testability with explicit Monaco reference

### Technical
- Constructor changed from `new MonacoErrorLens(editor, options)` to `new MonacoErrorLens(editor, monaco, options)`
- Factory functions updated: `setupErrorLens(editor, monaco, options)`, `createMinimalErrorLens(editor, monaco, options)`
- All examples and documentation updated to reflect new API
- Enhanced type safety with proper Monaco API typing

### Features
- **Inline Messages**: Show diagnostic text at the end of problematic lines
- **Line Highlighting**: Color entire lines containing problems with configurable background colors
- **Gutter Icons**: Display severity icons (✖ ⚠ ⓘ ●) in the editor gutter
- **Theme Support**: Built-in light, dark, and high-contrast themes
- **Custom Themes**: Create and apply custom color schemes
- **Performance**: Debounced updates
- **Accessibility**: High contrast mode support
- **TypeScript**: Strict type checking

### Technical
- ESM-only package with Vite build
- Test coverage with Vitest
- Peer dependency on Monaco Editor
- CSS-in-JS theming system

## [1.0.0] - 2024-12-08

### Added
- Initial release of Monaco Error Lens
- Core diagnostic visualization features
- Modern npm package structure
- Comprehensive documentation