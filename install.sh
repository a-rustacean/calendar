cargo build --release
rm -rf $HOME/.local/bin/cal
ln -s $(pwd)/target/release/cal $HOME/.local/bin/cal
chmod u+x $HOME/.local/bin/cal
