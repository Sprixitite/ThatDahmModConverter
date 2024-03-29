# ThatDahmModConverter

### This is a CLI application, no fancy windows here (mainly because I suck at rust)

Generates (or at least attempts to) `mod.txt` files from `base.lua` mod definitions used by Dahm/DorHud mods, for use with [DSBLT](https://github.com/Sprixitite/PAYDAY-The-Heist-DSBLT)

If `mod.txt` generation fails entirely, or the output doesn't work, file an issue with a link to the mod you're attempting to use, and I'll try to get back to you.

Build with "`cargo build --release`"

To use invoke the built "`that_dahm_mod_converter`" program from the folder containing your mod, and select the desired mod from the list, or simply invoke from the mod folder itself.