### Install Binaryen (wasm-opt):
```
cd ~
curl -L https://github.com/WebAssembly/binaryen/releases/download/version_117/binaryen-version_117-x86_64-linux.tar.gz | tar -xzf -
export PATH="~/binaryen-version_117/bin":$PATH
. ~/.bashrc
wasm-opt --version
```
