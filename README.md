# Steam Blocker
A simple Windows app for blocking outgoing network traffic from Steam. <br>
It works by modifying windows firewall rules. <br>
Done as a GUI and C++ learning project.

# Features
- Blocking outgoing network traffic
- Support for both x86 and x64
- Log console
- Dark theme

# Usage
- Run the app as an administrator
- Check the log console for errors
- Click "Toggle" to enable or disable the blocker.

# Compilation
Just make sure you have `SBFwEditor.dll` in the `bin` folder. <br>
Anything else is straightforward.

# Errors
**SBFwEditor.dll broken or missing:** <br>
Program can't find `SBFwEditor.dll` in the `bin` folder, or can't call the `Enable` or `IsEnabled` functions.

**Can't find Steam path:** <br>
Program can't find where steam has been installed. <br>
If you don't know what to do, just add a value to the registry. <br>

**Missing admin privileges:** <br>
You need to start this app "as an administrator".

**Critical error:** <br>
Something awful happened, and there is no easy fix.

**Unknown defined error:** <br>
I fucked up something with the `SBFwEditor.dll`

**Unknown unexpected error:** <br>
Shouldn't happen if you won't mess with the `SBFwEditor.dll`

# Fix missing steam path
- Go to the registry key shown in the log console
- Add REG_SZ value "InstallPath"
- Provide a path to a directory where steam.exe is located

# Fix missing SBFwEditor.dll
- Go [here]()
- Download version corresponding to the Steam Blocker one <br> *(v0.x.x SBFwEditor.dll = v0.y.y SteamBlocker.exe)*
- Put the `SBFwEditor.dll` to the `bin` folder which should be next to the SteamBlocker.exe
