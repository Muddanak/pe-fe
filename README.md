# pe-fe
Analyze a PE file, written in Rust.
<br><br>
This CLI program will analyze a PE file and relay important information needed to analyze the file.
<br><br>
The DOS, COFF, Optional, Detail, Data Directories, and Section headers are parsed to view to output to the screen.
---
# Usage
<br>
Display the help:<br>
<blockquote>pe-fe.exe -h</blockquote>
Analyze a file:<br>
<blockquote>pe-fe.exe -f (Filename)</blockquote>
Analyze a file and show all parsed headers:<br>
<blockquote>pe-fe.exe -a -f (Filename)</blockquote>