# Monaco Error Lens

Adaptation of  [VSCode Error Lens extension](https://github.com/usernamehw/vscode-error-lens):
makes diagnostics more prominent through inline messages, line highlighting, and gutter icons.

> **⚠️ Experimental Software**: This library is experimental and was largely LLM-generated as an adaptation of the [VSCode Error Lens extension](https://github.com/usernamehw/vscode-error-lens). Use with caution!!

<!-- [![npm version](https://badge.fury.io/js/monaco-error-lens.svg)](https://badge.fury.io/js/monaco-error-lens) -->
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **🔍 Inline Messages** - Show diagnostic text directly at the end of problematic lines
- **🎨 Line Highlighting** - Color entire lines containing problems with configurable backgrounds
- **📍 Gutter Icons** - Display severity icons in the editor gutter
- **🔧 Configurable** - Extensive customization options

## Installation

```bash
npm install monaco-error-lens
```

## Usage

### Basic Setup

```typescript
import * as monaco from 'monaco-editor';
import { MonacoErrorLens } from 'monaco-error-lens';

// Create Monaco editor
const editor = monaco.editor.create(document.getElementById('container'), {
  value: 'console.log("Hello, world!");',
  language: 'typescript',
  glyphMargin: true, // Enable gutter icons
});

// Create Error Lens instance
const errorLens = new MonacoErrorLens(editor, monaco, {
  enableInlineMessages: true,
  enableLineHighlights: true,
  enableGutterIcons: true,
});

// Add some sample diagnostics
monaco.editor.setModelMarkers(editor.getModel(), 'typescript', [
  {
    startLineNumber: 1,
    startColumn: 1,
    endLineNumber: 1,
    endColumn: 8,
    message: 'Prefer const over let',
    severity: monaco.MarkerSeverity.Warning,
    source: 'eslint'
  }
]);
```

### Advanced Configuration

```typescript
import { MonacoErrorLens, createCustomTheme } from 'monaco-error-lens';

const errorLens = new MonacoErrorLens(editor, monaco, {
  enableInlineMessages: true,
  enableLineHighlights: true,
  enableGutterIcons: true,
  followCursor: 'activeLine', // Only show diagnostics for current line
  messageTemplate: '[{source}] {message}', // Custom message format
  maxMessageLength: 100, // Truncate long messages
  updateDelay: 200, // Debounce delay in ms
  colors: {
    error: {
      background: 'rgba(255, 0, 0, 0.1)',
      foreground: '#ff4444',
    },
    warning: {
      background: 'rgba(255, 165, 0, 0.1)',
      foreground: '#ff8800',
    }
  }
});
```

### Factory Functions

For convenience, use the provided factory functions:

```typescript
import { setupErrorLens, createMinimalErrorLens } from 'monaco-error-lens';

// Quick setup with recommended defaults
const errorLens = setupErrorLens(editor, monaco, {
  // Optional overrides
  messageTemplate: '[{source}] {message}',
});

// Minimal setup for better performance
const minimalErrorLens = createMinimalErrorLens(editor, monaco, {
  // Only inline messages, no line highlights or gutter icons
  maxMessageLength: 80,
});
```

### Configuration Options

```typescript
// Update configuration dynamically
errorLens.updateOptions({
  followCursor: 'activeLine',           // Show only current line diagnostics
  messageTemplate: '[{source}] {message}', // Custom message format
  maxMessageLength: 100,                // Truncate long messages
  colors: {
    error: {
      background: 'rgba(255, 0, 0, 0.1)',
      foreground: '#ff4444'
    }
  }
});
```

### Control Methods

```typescript
errorLens.toggle();         // Enable/disable
errorLens.enable();         // Enable explicitly
errorLens.disable();        // Disable explicitly
errorLens.refresh();        // Force update
errorLens.isActive();       // Check if active
errorLens.getConfig();      // Get current config
errorLens.getEventEmitter(); // Get event emitter
errorLens.dispose();        // Clean up resources
```

### Event Handling

Monitor Error Lens events for advanced integrations:

```typescript
const errorLens = new MonacoErrorLens(editor, monaco);
const eventEmitter = errorLens.getEventEmitter();

// Listen for decoration updates
eventEmitter.on('decorations-updated', (data) => {
  console.log(`Updated ${data.decorationCount} decorations for ${data.markerCount} markers`);
});

// Listen for configuration changes
eventEmitter.on('config-updated', (data) => {
  console.log('Configuration updated:', data.config);
});

// Listen for status changes
eventEmitter.on('status-changed', (data) => {
  console.log(`Error Lens ${data.enabled ? 'enabled' : 'disabled'}`);
});

// Listen for errors
eventEmitter.on('error', (data) => {
  console.error('Error Lens error:', data.error.message);
});

// Remove specific listener
const unsubscribe = eventEmitter.on('decorations-updated', handler);
unsubscribe(); // Remove this specific listener

// Remove all listeners for an event
eventEmitter.removeAllListeners('decorations-updated');
```

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enableInlineMessages` | `boolean` | `true` | Show diagnostic messages at end of lines |
| `enableLineHighlights` | `boolean` | `true` | Highlight entire lines with problems |
| `enableGutterIcons` | `boolean` | `true` | Show icons in the gutter |
| `messageTemplate` | `string` | `'{message}'` | Template for formatting messages |
| `followCursor` | `'allLines' \| 'activeLine'` | `'allLines'` | Which lines to show diagnostics for |
| `maxMessageLength` | `number` | `200` | Maximum characters in inline messages |
| `updateDelay` | `number` | `100` | Debounce delay for updates (ms) |
| `enabled` | `boolean` | `true` | Enable or disable the extension |
| `colors` | `ErrorLensColors` | Default theme | Custom colors for each severity |

## CSS Classes

Generated CSS classes for customization:
- `.monaco-error-lens-message` - Inline message styling
- `.monaco-error-lens-line-{severity}` - Line background colors
- `.monaco-error-lens-gutter-{severity}` - Gutter icon styling

## TypeScript Support

```typescript
import type {
  ErrorLensOptions,
  ErrorLensTheme,
  ErrorLensStatus,
  DiagnosticDecoration
} from 'monaco-error-lens';
```

## Live Example

To see Monaco Error Lens in action:

```bash
# 1. Clone the repository
git clone https://github.com/smucclaw/monaco-error-lens.git
cd monaco-error-lens

# 2. Install and build
npm i
npm run build

# 3. Serve the example locally
python3 -m http.server 8080
# or use any static file server

# 4. Open http://localhost:8080/example.html in your browser
```

The example demonstrates:
- ✅ **Real package usage** - Uses the actual built `MonacoErrorLens` class from `dist/index.js`
- 🔍 **Inline error messages** at the end of problematic lines
- 🎨 **Line highlighting** with color-coded backgrounds by severity
- 📍 **Gutter icons** showing severity indicators
- 🔧 **Interactive controls** to test different configurations

## Development

```bash
# Install dependencies
npm install

# Development build with watch
npm run dev

# Run tests
npm test

# Lint and fix
npm run lint

# Build
npm run build

# Type check
npm run check
```

## License

MIT

## Acknowledgments

Adapted from the [VSCode Error Lens extension](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens).
