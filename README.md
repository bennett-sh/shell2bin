# 🐚 shell2bin
#### A simple program to package shell scripts (sh, bat, ...) as executables

# ✏️ Usage
```shell2bin <script> [<output name without extension>]```

# 📦 Requirements
- Rust (with cargo & rustc in path)

# 💠 Installation
1. [Install requirements](#📦-requirements)
2. Run ```cargo install shell2bin```
3. Profit.

# 📂 Supported languages
Most script languages should be supported when installed. You will need to specify the language using either a shebang or using a S2B-annotation. Both shebangs and S2B-annotations must be on the first line of the file. The only exception to this are batch files. Here you can add an ```@echo off``` on the line before a S2B-annotation. The syntax of S2B-annotations is the following: ```[Line comment in language; supported: #, "rem " and //]S2B:[program] [<args>]```

Examples:
```bash
#!/bin/bash
ls
```
```bat
@echo off
rem S2B:cmd.exe /C
ls
```
```bat
rem S2B:cmd.exe /C
ls
```
```py
#S2B:python
print("Hello world!")
```

# 🐛 Known Issues
Currently, there are no issues known. Feel free to report any issues using the Issues-tab.
