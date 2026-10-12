#!/usr/bin/env sh
set -eu

# Uninstalls Kurocto that was installed using the install.sh script

check_remaining_installations() {
    platform="$(uname -s)"
    if [ "$platform" = "Darwin" ]; then
        # Check for any Kurocto variants in /Applications
        remaining=$(ls -d /Applications/Kurocto*.app 2>/dev/null | wc -l)
        [ "$remaining" -eq 0 ]
    else
        # Check for any Kurocto variants in ~/.local
        remaining=$(ls -d "$HOME/.local/kurocto"*.app 2>/dev/null | wc -l)
        [ "$remaining" -eq 0 ]
    fi
}

prompt_remove_preferences() {
    printf "Do you want to keep your Kurocto preferences? [Y/n] "
    read -r response
    case "$response" in
        [nN]|[nN][oO])
            rm -rf "$HOME/.config/kurocto"
            echo "Preferences removed."
            ;;
        *)
            echo "Preferences kept."
            ;;
    esac
}

main() {
    platform="$(uname -s)"
    channel="${ZED_CHANNEL:-stable}"

    if [ "$platform" = "Darwin" ]; then
        platform="macos"
    elif [ "$platform" = "Linux" ]; then
        platform="linux"
    else
        echo "Unsupported platform $platform"
        exit 1
    fi

    "$platform"

    echo "Kurocto has been uninstalled"
}

linux() {
    suffix=""
    if [ "$channel" != "stable" ]; then
        suffix="-$channel"
    fi

    appid=""
    db_suffix="stable"
    case "$channel" in
      stable)
        appid="dev.kurocto.Kurocto"
        db_suffix="stable"
        ;;
      nightly)
        appid="dev.kurocto.Kurocto-Nightly"
        db_suffix="nightly"
        ;;
      preview)
        appid="dev.kurocto.Kurocto-Preview"
        db_suffix="preview"
        ;;
      dev)
        appid="dev.kurocto.Kurocto-Dev"
        db_suffix="dev"
        ;;
      *)
        echo "Unknown release channel: ${channel}. Using stable app ID."
        appid="dev.kurocto.Kurocto"
        db_suffix="stable"
        ;;
    esac

    # Remove the app directory
    rm -rf "$HOME/.local/kurocto$suffix.app"

    # Remove the binary symlink
    rm -f "$HOME/.local/bin/kurocto"

    # Remove the .desktop file
    rm -f "$HOME/.local/share/applications/${appid}.desktop"

    # Remove the database directory for this channel
    rm -rf "$HOME/.local/share/kurocto/db/0-$db_suffix"

    # Remove socket file
    rm -f "$HOME/.local/share/kurocto/kurocto-$db_suffix.sock"

    # Remove the entire Kurocto directory if no installations remain
    if check_remaining_installations; then
        rm -rf "$HOME/.local/share/kurocto"
        prompt_remove_preferences
    fi

    rm -rf $HOME/.kurocto_server
}

macos() {
    app="Kurocto.app"
    db_suffix="stable"
    app_id="dev.kurocto.Kurocto"
    case "$channel" in
      nightly)
        app="Kurocto Nightly.app"
        db_suffix="nightly"
        app_id="dev.kurocto.Kurocto-Nightly"
        ;;
      preview)
        app="Kurocto Preview.app"
        db_suffix="preview"
        app_id="dev.kurocto.Kurocto-Preview"
        ;;
      dev)
        app="Kurocto Dev.app"
        db_suffix="dev"
        app_id="dev.kurocto.Kurocto-Dev"
        ;;
    esac

    # Remove the app bundle
    if [ -d "/Applications/$app" ]; then
        rm -rf "/Applications/$app"
    fi

    # Remove the binary symlink
    rm -f "$HOME/.local/bin/kurocto"

    # Remove the database directory for this channel
    rm -rf "$HOME/Library/Application Support/Kurocto/db/0-$db_suffix"

    # Remove app-specific files and directories
    rm -rf "$HOME/Library/Application Support/com.apple.sharedfilelist/com.apple.LSSharedFileList.ApplicationRecentDocuments/$app_id.sfl"*
    rm -rf "$HOME/Library/Caches/$app_id"
    rm -rf "$HOME/Library/HTTPStorages/$app_id"
    rm -rf "$HOME/Library/Preferences/$app_id.plist"
    rm -rf "$HOME/Library/Saved Application State/$app_id.savedState"

    # Remove the entire Kurocto directory if no installations remain
    if check_remaining_installations; then
        rm -rf "$HOME/Library/Application Support/Kurocto"
        rm -rf "$HOME/Library/Logs/Kurocto"

        prompt_remove_preferences
    fi

    rm -rf $HOME/.kurocto_server
}

main "$@"
