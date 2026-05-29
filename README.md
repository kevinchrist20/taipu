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

Download the latest build for your platform from the [Releases](https://github.com/kevinchrist20/taipu/releases) page.

Important: current builds may be unsigned or not notarized yet. Only install artifacts from the official releases page.

### Verify Downloads (Recommended)

Before installing, verify the checksum of the downloaded file against the release checksum file.

macOS / Linux:

```bash
shasum -a 256 Taipu*.dmg
# or
sha256sum Taipu*
```

Windows (PowerShell):

```powershell
Get-FileHash .\Taipu* -Algorithm SHA256
```

Match the output with the checksum published in the release assets/notes.

### macOS (Apple Silicon + Intel)

Choose the correct build:

- Apple Silicon (M1/M2/M3): `aarch64-apple-darwin`
- Intel: `x86_64-apple-darwin`

Install:

1. Download the `.dmg` for your architecture.
2. Open it and drag `Taipu.app` into `Applications`.
3. Try launching once from `Applications`.

If macOS blocks it because it is not signed/notarized yet:

1. Open `System Settings` -> `Privacy & Security`.
2. In the security section, click `Open Anyway` for Taipu.
3. Confirm by clicking `Open`.

Terminal alternative (remove quarantine flag):

```bash
sudo xattr -dr com.apple.quarantine /Applications/Taipu.app
open -a /Applications/Taipu.app
```

### Windows

1. Download the `.msi` or `.exe` installer.
2. Launch the installer.

If Windows SmartScreen appears:

1. Click `More info`.
2. Click `Run anyway`.

If the downloaded file is blocked, you can unblock it first:

```powershell
Unblock-File .\Taipu*.msi
# or
Unblock-File .\Taipu*.exe
```

### Linux

Choose the artifact for your distro:

- `.AppImage` (portable, most distros)
- `.deb` (Debian/Ubuntu)
- `.rpm` (Fedora/RHEL/openSUSE)

AppImage:

```bash
chmod +x Taipu*.AppImage
./Taipu*.AppImage
```

Debian/Ubuntu:

```bash
sudo apt install ./taipu*.deb
```

Fedora/RHEL/openSUSE:

```bash
# Fedora/RHEL
sudo dnf install ./taipu*.rpm

# openSUSE
sudo zypper install ./taipu*.rpm
```

If your desktop asks for confirmation because the binary is from the internet, allow execution and continue.

## Troubleshooting

### macOS says "app is damaged" or still won’t open

1. Confirm the app came from the official Releases page.
2. Remove quarantine and reopen:

```bash
sudo xattr -dr com.apple.quarantine /Applications/Taipu.app
open -a /Applications/Taipu.app
```

### Linux AppImage does not run

1. Ensure execute permission is set:

```bash
chmod +x Taipu*.AppImage
```

2. If your distro requires FUSE support, install FUSE packages and retry.

### Windows warns about unknown publisher / SmartScreen

1. Use `More info` -> `Run anyway` for official release binaries.
2. If needed, unblock the file first:

```powershell
Unblock-File .\Taipu*.msi
# or
Unblock-File .\Taipu*.exe
```

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

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for the contribution workflow, PR checklist, and local validation steps.

## License

[MIT](LICENSE)
