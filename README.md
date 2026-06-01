# WhatsApp Ultra Fast

A lightweight WhatsApp Web client built with Tauri and Rust for Linux.

> **Note:** This app uses WebKit2GTK as its rendering engine. RAM usage is typically 1.2–1.5 GB due to WhatsApp Web's own resource requirements, not the app itself. WhatsApp Web loads all chat history, thumbnails, and service workers locally regardless of the client used.

---

## Features

- WhatsApp Web wrapped in a native Linux window
- GPU hardware acceleration via WebKit2GTK
- Minimize to system tray on close
- Multi-session support (opens new window per session)
- Cache clearing from within the app
- No Chromium bundled — significantly smaller binary than Electron-based clients
- Custom user agent spoofing for better compatibility

---

## RAM Usage

| Process | Memory |
|---|---|
| WebKitWebProcess (main) | ~700 MB |
| WebKitWebProcess (secondary) | ~270 MB |
| WhatsappUltraFast (binary) | ~240 MB |
| WebKitNetworkProcess | ~95 MB |
| **Total** | **~1.3 GB** |

This is expected behavior. WhatsApp Web itself is a heavy React application. The numbers are comparable to running WhatsApp Web in a browser tab. There is no optimization possible on the client side beyond what is already implemented.

---

## Requirements

- `webkit2gtk-4.1`
- `gtk3`
- `libayatana-appindicator` or `libappindicator-gtk3` (for tray icon)

---

## Installing Dependencies

### Arch Linux / Manjaro

```bash
sudo pacman -S webkit2gtk-4.1 gtk3 libayatana-appindicator
```

If `debtap` is not installed (needed to convert `.deb` to Arch package):

```bash
# Install yay first if not available
sudo pacman -S --needed git base-devel
git clone https://aur.archlinux.org/yay.git
cd yay && makepkg -si

# Then install debtap
yay -S debtap
sudo debtap -u
```

### Debian / Ubuntu / Linux Mint

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-0 libgtk-3-0 libayatana-appindicator3-1
```

On older Ubuntu (20.04 / 22.04) that does not have `webkit2gtk-4.1`:

```bash
sudo apt install libwebkit2gtk-4.0-37 libgtk-3-0 libayatana-appindicator3-1
```

### Fedora

```bash
sudo dnf install webkit2gtk4.1 gtk3 libayatana-appindicator
```

### openSUSE

```bash
sudo zypper install libwebkit2gtk-4_1-0 gtk3 libayatana-appindicator3-1
```

---

## Installation

### Arch Linux (recommended)

Download the latest `.deb` from [Releases](https://github.com/ramdanolii14/WhatsappUltraFast/releases), then convert and install:

```bash
sudo debtap -u
sudo debtap WhatsappUltraFast_0.1.0_amd64.deb
sudo pacman -U --nodeps --nodeps whatsapp-ultra-fast-0.1.0-1-x86_64.pkg.tar.zst
```

### Debian / Ubuntu

```bash
sudo dpkg -i WhatsappUltraFast_0.1.0_amd64.deb
sudo apt-get install -f
```

### RPM (Fedora / openSUSE)

```bash
sudo rpm -i whatsappultrafast-0.1.0-1.x86_64.rpm
```

---

## Build from Source

### Prerequisites

```bash
# Arch Linux
sudo pacman -S rust webkit2gtk-4.1 gtk3 base-devel

# Install Tauri CLI
cargo install tauri-cli
```

### Build

```bash
git clone https://github.com/ramdanolii14/WhatsappUltraFast
cd WhatsappUltraFast
cargo tauri build
```

The output files will be at:

```
src-tauri/target/release/bundle/deb/
src-tauri/target/release/bundle/rpm/
src-tauri/target/release/WhatsappUltraFast  (binary)
```

---

## Running

After installation:

```bash
WhatsappUltraFast
```

Or run directly from the build output without installing:

```bash
./src-tauri/target/release/WhatsappUltraFast
```

For better GPU performance:

```bash
WEBKIT_DISABLE_COMPOSITING_MODE=0 WPE_BACKEND=default WhatsappUltraFast
```

---

## Tray Icon

Closing the window does not exit the app. It minimizes to the system tray. To fully quit, right-click the tray icon and select Keluar (Quit).

---

## Cache Management

WhatsApp Web stores data locally at:

```
~/.local/share/com.ramdanolii14.whatsappultrafast/
```

To clear HTTP cache without logging out:

```bash
rm -rf ~/.local/share/com.ramdanolii14.whatsappultrafast/WebKitCache
rm -rf ~/.local/share/com.ramdanolii14.whatsappultrafast/CacheStorage
```

Do not delete the `databases`, `cookies`, or `localstorage` folders — those contain your login session and chat data.

---

## Project Structure

```
whatsappultrafast/
├── src/
│   └── index.html
├── src-tauri/
│   ├── src/
│   │   ├── main.rs
│   │   └── lib.rs
│   ├── icons/
│   ├── Cargo.toml
│   ├── build.rs
│   └── tauri.conf.json
└── README.md
```

---

## Known Limitations

- Voice and video calls may not work. WebRTC support in WebKit2GTK on Linux is incomplete.
- RAM usage is high by nature of WhatsApp Web, not the wrapper.
- WebKit2GTK renders slower than Chromium-based clients for JavaScript-heavy pages.

---

## Star History

<a href="https://www.star-history.com/?repos=ramdanolii14%2FWhatsappUltraFast&type=timeline&logscale=&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=ramdanolii14/WhatsappUltraFast&type=timeline&theme=dark&logscale&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=ramdanolii14/WhatsappUltraFast&type=timeline&logscale&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=ramdanolii14/WhatsappUltraFast&type=timeline&logscale&legend=top-left" />
 </picture>
</a>  

---

## Developer

**Ramdan Olii**  
Website: [ramdanolii.my.id](https://ramdanolii.my.id)  
GitHub: [github.com/ramdanolii14](https://github.com/ramdanolii14)

---

## License

GPL-3.0
