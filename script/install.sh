#!/usr/bin/env sh
set -eu

# Unpacks a locally built Linux bundle (ZED_BUNDLE_PATH, as produced by
# script/bundle-linux) into ~/.local/. It does not download anything; see
# script/install-linux.

main() {
    platform="$(uname -s)"
    arch="$(uname -m)"
    channel="${ZED_CHANNEL:-stable}"
    # Use TMPDIR if available (for environments with non-standard temp directories)
    if [ -n "${TMPDIR:-}" ] && [ -d "${TMPDIR}" ]; then
        temp="$(mktemp -d "$TMPDIR/kurocto-XXXXXX")"
    else
        temp="$(mktemp -d "/tmp/kurocto-XXXXXX")"
    fi

    if [ "$platform" != "Linux" ]; then
        echo "Unsupported platform $platform: this script only installs a locally built Linux bundle"
        exit 1
    fi

    case "$arch" in
        arm64* | aarch64)
            arch="aarch64"
            ;;
        x86*)
            arch="x86_64"
            ;;
        *)
            echo "Unsupported architecture"
            exit 1
            ;;
    esac

    linux "$@"

    if [ "$(command -v kurocto)" = "$HOME/.local/bin/kurocto" ]; then
        echo "Kurocto has been installed. Run with 'kurocto'"
    else
        echo "To run Kurocto from your terminal, you must add ~/.local/bin to your PATH"
        echo "Run:"

        case "$SHELL" in
            *zsh)
                echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.zshrc"
                echo "   source ~/.zshrc"
                ;;
            *fish)
                echo "   fish_add_path -U $HOME/.local/bin"
                ;;
            *)
                echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.bashrc"
                echo "   source ~/.bashrc"
                ;;
        esac

        echo "To run Kurocto now, '~/.local/bin/kurocto'"
    fi
}

linux() {
    if [ -z "${ZED_BUNDLE_PATH:-}" ]; then
        echo "ZED_BUNDLE_PATH is not set. Build a bundle with script/bundle-linux and set ZED_BUNDLE_PATH to the resulting tarball, or run script/install-linux."
        exit 1
    fi
    cp "$ZED_BUNDLE_PATH" "$temp/kurocto-linux-$arch.tar.gz"

    suffix=""
    if [ "$channel" != "stable" ]; then
        suffix="-$channel"
    fi

    appid=""
    case "$channel" in
      stable)
        appid="dev.kurocto.Kurocto"
        ;;
      nightly)
        appid="dev.kurocto.Kurocto-Nightly"
        ;;
      preview)
        appid="dev.kurocto.Kurocto-Preview"
        ;;
      dev)
        appid="dev.kurocto.Kurocto-Dev"
        ;;
      *)
        echo "Unknown release channel: ${channel}. Using stable app ID."
        appid="dev.kurocto.Kurocto"
        ;;
    esac

    # Unpack
    rm -rf "$HOME/.local/kurocto$suffix.app"
    mkdir -p "$HOME/.local/kurocto$suffix.app"
    tar -xzf "$temp/kurocto-linux-$arch.tar.gz" -C "$HOME/.local/"

    zed_editor="$HOME/.local/kurocto$suffix.app/libexec/kurocto-editor"
    if [ -f "$zed_editor" ] && command -v ldd >/dev/null 2>&1; then
        missing="$(ldd "$zed_editor" 2>/dev/null | sed -n 's/^[[:space:]]*\(.*\) => not found$/\1/p')"
        if [ -n "$missing" ]; then
            echo "Warning: your system is missing libraries that Kurocto needs:"
            echo "$missing" | sed 's/^/    /'
            echo "Install them with your package manager, or Kurocto will fail to start."
        fi
    fi

    # Setup ~/.local directories
    mkdir -p "$HOME/.local/bin" "$HOME/.local/share/applications"

    # Link the binary
    if [ -f "$HOME/.local/kurocto$suffix.app/bin/kurocto" ]; then
        ln -sf "$HOME/.local/kurocto$suffix.app/bin/kurocto" "$HOME/.local/bin/kurocto"
    else
        # support for versions before 0.139.x.
        ln -sf "$HOME/.local/kurocto$suffix.app/bin/cli" "$HOME/.local/bin/kurocto"
    fi

    # Copy .desktop file
    desktop_file_path="$HOME/.local/share/applications/${appid}.desktop"
    src_dir="$HOME/.local/kurocto$suffix.app/share/applications"
    if [ -f "$src_dir/${appid}.desktop" ]; then
        cp "$src_dir/${appid}.desktop" "${desktop_file_path}"
    else
        # Fallback for older tarballs
        cp "$src_dir/kurocto$suffix.desktop" "${desktop_file_path}"
    fi
    sed -i "s|Icon=kurocto|Icon=$HOME/.local/kurocto$suffix.app/share/icons/hicolor/512x512/apps/kurocto.png|g" "${desktop_file_path}"
    sed -i "s|Exec=kurocto|Exec=$HOME/.local/kurocto$suffix.app/bin/kurocto|g" "${desktop_file_path}"
}

main "$@"
