# pe-fe

- [Description](#description)
- [Usage](#usage)
- [License](#license)

## Description

Analyze a PE file, written in Rust.

This CLI program will analyze a PE file and relay important information needed to analyze the file. The DOS, COFF, Optional, Detail, Data Directories, and Section headers are parsed to view to output to the screen.

This program was created to offer to the DFIR community another PE analyzer, written in Rust, to helpfully be used during analysis.  This project was also a form of practice to help learn the Rust language.


---
## Usage

Display the help:
```
pe-fe.exe -h
```
Analyze a file:
```
pe-fe.exe (Filename)
```
Analyze a file and show all parsed headers:
```
pe-fe.exe -a (Filename)
```

---
## License

Released under the MIT License