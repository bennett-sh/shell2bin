# 🐚 shell2bin
#### A (very) simple program to package shell scripts (sh, bat, ...) as executables

# ✏️ Usage
```shell2bin <script> [<output file>]```

# 📦 Requirements
- g++ (install using MinGW on Windows)

# 💠 Installation
1. [Download a shell2bin executable](https://github.com/bennett-sh/shell2bin/releases/latest) or [build one from source](#%EF%B8%8F-building-from-source)
2. Add it to PATH
3. Done

# 🛠️ Building from source
To build this project from source you can either run ```/build.sh``` (uses g++) or simply compile ```/src/main.cpp``` using any compiler like g++.

# 🐛 Known Issues
- [ ] Quotes (") currently don't work unless escaped (resolved in Rust rewrite)
