mkdir -p ~/.starpkg

platform="$(uname -s | tr '[:upper:]' '[:lower:]')"
if [ "${platform}" = "darwin" ]; then
    # macOS
    curl -o starpkg.tar.gz -L https://github.com/nanaian/starpkg/releases/latest/download/starpkg-x86_64-apple-darwin.tar.gz
else
    # Linux
    if ldd /bin/sh | grep -i musl >/dev/null; then
        # Linux (MUSL)
        curl -o starpkg.tar.gz -L https://github.com/nanaian/starpkg/releases/latest/download/starpkg-x86_64-unknown-linux-musl.tar.gz
    else
        # Linux (GNU)
        curl -o starpkg.tar.gz -L https://github.com/nanaian/starpkg/releases/latest/download/starpkg-x86_64-unknown-linux-gnu.tar.gz
    fi
fi
tar -xf starpkg.tar.gz -C ~/.starpkg
rm starpkg.tar.gz
chmod +x ~/.starpkg/starpkg

if [ -d "$HOME/.local/bin" ]; then
    ln -s ~/.starpkg/starpkg ~/.local/bin
else
    if [ -d "$HOME/bin" ]; then
        ln -s ~/.starpkg/starpkg ~/bin
    else
        # Attempt to add to path
        [ -f "$HOME/.profile" ] && echo 'export PATH=~/.starpkg:$PATH' >> ~/.profile
        [ -f "$HOME/.bashrc" ] && echo 'export PATH=~/.starpkg:$PATH' >> ~/.bashrc
        [ -f "$HOME/.zprofile" ] && echo 'export PATH=~/.starpkg:$PATH' >> ~/.zprofile
        [ -f "$HOME/.zshrc" ] && echo 'export PATH=~/.starpkg:$PATH' >> ~/.zshrc
    fi
fi

echo "starpkg installed/updated and added to PATH"
