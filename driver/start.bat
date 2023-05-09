rem %~dp0 is the path of the directory the bat file is in.
sc create example binPath="%~dp0..\target\x86_64-pc-windows-msvc\debug\driver.sys" type=kernel
sc start example