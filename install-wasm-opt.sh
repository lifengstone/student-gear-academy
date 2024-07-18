cd ~
curl -L https://github.com/WebAssembly/binaryen/releases/download/version_117/binaryen-version_117-x86_64-linux.tar.gz | tar -xzf -
echo 'export PATH="~/binaryen-version_117/bin":$PATH' >> ~/.bashrc
source ~/.bashrc
wasm-opt --version