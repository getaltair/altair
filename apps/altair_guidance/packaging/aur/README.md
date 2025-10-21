# AUR Package for Altair Guidance

This directory contains the PKGBUILD and related files for packaging Altair Guidance for the Arch User Repository (AUR).

## For Users

### Installation from AUR

The easiest way to install Altair Guidance on Arch Linux is using an AUR helper:

**Using yay:**

```bash
yay -S altair-guidance
```

**Using paru:**

```bash
paru -S altair-guidance
```

**Manual installation:**

```bash
# Clone the AUR repository
git clone https://aur.archlinux.org/altair-guidance.git
cd altair-guidance

# Review the PKGBUILD (always review AUR packages!)
less PKGBUILD

# Build and install
makepkg -si
```

### Updating

```bash
# Using AUR helper
yay -Syu altair-guidance
# or
paru -Syu altair-guidance

# Manual update
cd altair-guidance
git pull
makepkg -si
```

### Uninstallation

```bash
sudo pacman -R altair-guidance
```

## For Maintainers

### Prerequisites

- Arch Linux or Arch-based distribution
- `base-devel` package group
- `flutter` package from AUR or official repos
- AUR account (for publishing)

### Testing the PKGBUILD Locally

```bash
# Navigate to this directory
cd apps/altair_guidance/packaging/aur

# Build the package
makepkg -f

# Install for testing
makepkg -fi

# Test the application
altair-guidance
```

### Updating the Package

When releasing a new version:

1. **Update version and checksums:**

   ```bash
   # Edit PKGBUILD
   pkgver=0.2.0

   # Download source tarball
   wget https://github.com/getaltair/altair/archive/v0.2.0.tar.gz

   # Generate checksum
   sha256sum v0.2.0.tar.gz

   # Update sha256sums in PKGBUILD
   sha256sums=('actual_checksum_here')
   ```

2. **Test the build:**

   ```bash
   makepkg -f
   ```

3. **Generate .SRCINFO:**

   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```

4. **Commit and push to AUR:**

   ```bash
   git add PKGBUILD .SRCINFO
   git commit -m "Update to version 0.2.0"
   git push
   ```

### Publishing to AUR (First Time)

1. **Create AUR SSH key (if not already done):**

   ```bash
   ssh-keygen -t ed25519 -C "your_email@example.com"
   ```

2. **Add SSH key to AUR account:**
   - Go to <https://aur.archlinux.org/account/>
   - Add your public key

3. **Clone the AUR repository:**

   ```bash
   git clone ssh://aur@aur.archlinux.org/altair-guidance.git aur-altair-guidance
   cd aur-altair-guidance
   ```

4. **Copy files and commit:**

   ```bash
   cp /path/to/altair/apps/altair_guidance/packaging/aur/PKGBUILD .
   makepkg --printsrcinfo > .SRCINFO
   git add PKGBUILD .SRCINFO
   git commit -m "Initial import of altair-guidance"
   git push
   ```

### AUR Package Guidelines

Follow the [AUR submission guidelines](https://wiki.archlinux.org/title/AUR_submission_guidelines):

- **Naming**: Use lowercase with hyphens (altair-guidance)
- **PKGBUILD**: Follow Arch packaging standards
- **Dependencies**: Only list runtime dependencies
- **Testing**: Always test builds before pushing
- **Updates**: Update promptly when new versions are released
- **Orphaning**: If you can't maintain, orphan the package

### Common Issues

**Flutter not found:**

```bash
# Install Flutter from AUR
yay -S flutter
```

**Build fails with GTK errors:**

```bash
# Install GTK development packages
sudo pacman -S gtk3 libsecret
```

**Permission denied on /opt:**

```bash
# Use makepkg without sudo
makepkg -si  # Will ask for sudo password when needed
```

## Package Structure

After installation, files are organized as:

```
/opt/altair-guidance/          # Application bundle
├── altair_guidance            # Main executable
├── lib/                       # Shared libraries
└── data/                      # Application data

/usr/bin/altair-guidance       # Wrapper script (in PATH)

/usr/share/applications/       # Desktop entry
└── altair-guidance.desktop

/usr/share/licenses/           # License file
└── altair-guidance/LICENSE
```

## Troubleshooting

### Package won't build

1. **Clear build cache:**

   ```bash
   rm -rf pkg/ src/
   makepkg -f
   ```

2. **Update Flutter:**

   ```bash
   yay -Syu flutter
   ```

3. **Check dependencies:**

   ```bash
   pacman -Q gtk3 libsecret cmake ninja clang
   ```

### Application won't start

1. **Check library dependencies:**

   ```bash
   ldd /opt/altair-guidance/altair_guidance
   ```

2. **Run with debug output:**

   ```bash
   /opt/altair-guidance/altair_guidance --verbose
   ```

3. **Check for missing GTK libraries:**

   ```bash
   sudo pacman -S gtk3 libsecret
   ```

## Additional Resources

- [Arch Wiki - PKGBUILD](https://wiki.archlinux.org/title/PKGBUILD)
- [Arch Wiki - AUR](https://wiki.archlinux.org/title/Arch_User_Repository)
- [AUR Package Guidelines](https://wiki.archlinux.org/title/AUR_submission_guidelines)
- [Flutter Desktop Documentation](https://docs.flutter.dev/platform-integration/linux/install-linux)

---

**Maintained by:** Altair Development Team
**AUR Package:** <https://aur.archlinux.org/packages/altair-guidance>
**Bug Reports:** <https://github.com/getaltair/altair/issues>
