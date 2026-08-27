# MetaGrid Arch Linux (AUR) Packages

This directory contains the PKGBUILD recipes for Arch Linux and Arch-based distributions (Manjaro, EndeavourOS, Garuda, etc.).

## Available Packages

1. **`metagrid-bin`** (Recommended): Pre-compiled binary package downloaded directly from GitHub Releases. Installs in seconds.
2. **`metagrid`**: Source-built package compiling with Rust + Vite/Svelte during installation.

## How to Publish to AUR

### 1. Prerequisite
- Register an account on [https://aur.archlinux.org](https://aur.archlinux.org)
- Add your SSH public key to your AUR profile (`~/.ssh/id_ed25519.pub`)

### 2. Initial Setup for `metagrid-bin`
```bash
git clone ssh://aur@aur.archlinux.org/metagrid-bin.git
cd metagrid-bin
cp /path/to/metagrid/aur/metagrid-bin/PKGBUILD .
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Initial release v1.2.4"
git push origin master
```

### 3. User Installation
Once pushed, users can install via any AUR helper:
```bash
yay -S metagrid-bin
# or
paru -S metagrid-bin
```
