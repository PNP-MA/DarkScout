# DarkScout

[![Forks][forks-shield]][forks-url][![Stargazers][stars-shield]][stars-url][![Issues][issues-shield]][issues-url]

DarkScout is a professional, high-performance subdomain enumeration framework written in Rust. It combines passive OSINT discovery with ultra-fast asynchronous brute-forcing to provide a comprehensive reconnaissance solution for security professionals and bug bounty hunters.

## ✨ Key Features

-   **Modular Plugin System**: Easily extensible architecture for adding new OSINT sources.
-   **Passive OSINT Discovery**: Leverages multiple online sources including Anubis, HackerTarget, CrtSH, and Wayback Machine.
-   **Asynchronous Brute-Forcing**: High-performance DNS resolution using `trust-dns-resolver` with controlled concurrency.
-   **Smart Deduplication**: Automatic normalization, sorting, and deduplication of results across all discovery phases.
-   **Professional CLI**: Modern interface with concurrent progress bars, colored output, and detailed execution metadata.
-   **Flexible Input**: Support for single wordlists, multiple files, or entire directories of subdomains.

## 🔌 Supported Plugins

Use `-l` to see the current status and descriptions of all plugins:

-   **Alienvault**: OTX passive DNS (Supports optional API key).
-   **Anubis**: High-signal OSINT from jonlu.ca.
-   **CrtSH**: Certificate transparency logs with automatic retry logic.
-   **HackerTarget**: Reliable host search API.
-   **WaybackArchive**: Comprehensive history from the Internet Archive.
-   **BeVigil**: OSINT API (Requires API key).
-   **ThreatMiner**: Passive DNS (Currently disabled).

## 🚀 Installation

### Build from source
```bash
git clone https://github.com/DarkSuite/DarkScout
cd DarkScout
cargo build --release
./target/release/DarkScout --help
```

## 📖 Usage Examples

### 1. Passive Discovery (Default)
```bash
./DarkScout -t hackthissite.org
```

### 2. Specific Plugin Execution
```bash
./DarkScout -t hackthissite.org -p Anubis,Crtsh
```

### 3. High-Performance Brute Force
```bash
./DarkScout -t hackthissite.org -w subdomains.txt -c 500
```

### 4. Hybrid Mode (OSINT + Brute Force)
```bash
./DarkScout -t hackthissite.org -p HackerTarget -w ./wordlists/ -o results.txt
```

### 5. List All Plugins
```bash
./DarkScout -l
```

## 🛠️ Configuration
Some plugins perform better with API keys. Create a `.env` file in the project root:
```env
ALIENVAULT_API_KEY=your_key_here
BEVIGIL_API_KEY=your_key_here
```

## 🤝 Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. **Fork** the Project
2. **Create** your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. **Commit** your Changes (`git commit -m 'Add some AmazingFeature'`)
4. **Push** to the Branch (`git push origin feature/AmazingFeature`)
5. **Open** a Pull Request

## ⚖️ License
Distributed under the MIT License. See `LICENSE` for more information.

<!-- MARKDOWN LINKS & IMAGES -->
[forks-shield]: https://img.shields.io/github/forks/DarkSuite/DarkScout.svg?style=for-the-badge
[forks-url]: https://github.com/DarkSuite/DarkScout/network/members
[stars-shield]: https://img.shields.io/github/stars/DarkSuite/DarkScout.svg?style=for-the-badge
[stars-url]: https://github.com/DarkSuite/DarkScout/stargazers
[issues-shield]: https://img.shields.io/github/issues/DarkSuite/DarkScout.svg?style=for-the-badge
[issues-url]: https://github.com/DarkSuite/DarkScout/issues
