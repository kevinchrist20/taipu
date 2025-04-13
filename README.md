# Taipu – A Modern Typing Trainer App

![Built with Tauri](https://img.shields.io/badge/Built%20with-Tauri-blue)
![Vue 3](https://img.shields.io/badge/Vue-3-brightgreen)
![TypeScript](https://img.shields.io/badge/TypeScript-blue)

Taipu is a sleek, lightweight desktop application that helps users improve their typing speed and accuracy through structured lessons and real-time performance tracking. Designed with a clean, responsive interface, Taipu guides learners from the basics of the home row to full keyboard mastery.

Built using Vue 3, TypeScript, and Tailwind CSS, and powered by Tauri for a high-performance native experience, Taipu runs smoothly across platforms with minimal system resources.

## Key Features

- 📚 Progressive typing lessons (Home Row → Full Keyboard)
- 🧠 Real-time feedback with WPM and accuracy tracking
- 📈 Typing tests to track skill development and unlock new levels
- 💾 Local data storage and offline use
- 🌙 Clean, distraction-free UI with light/dark mode
- ⚡ Fast and secure with native performance via Tauri

Perfect for learners, students, and professionals looking to sharpen their typing skills with a beautiful and focused environment.

## Installation

Download the latest version for your platform from the [Releases](https://github.com/yourusername/taipu/releases) page.

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) (v20 or newer)
- [Rust](https://www.rust-lang.org/tools/install)
- [pnpm](https://pnpm.io/installation)

### Setup

```bash
# Clone the repository
git clone https://github.com/kevinchrist20/taipu.git
cd taipu

# Install dependencies
pnpm install

# Start development server
pnpm launch
```

### Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### Building

```bash
pnpm build
pnpm tauri build
```

## License

[MIT](LICENSE)
