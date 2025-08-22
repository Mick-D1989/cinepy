### Install UV
```
curl -LsSf https://astral.sh/uv/install.sh | sh
```

#### Create a new project
Create an empty pyproject.toml file which can have dependencies added to.
```
uv init --bare
```

Add a build parameter for python 3.12.
```
echo 3.12 > .python-version
```

#### Install pyo3 build dependencies
This will initialise a .venv and install the packages.
```
uv add maturin maturin_import_hook
```

### Install rust

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Use the nightly toolchain so we can use a faster compiler "cranelift"
```
rustup component add rustc-codegen-cranelift-preview --toolchain nightly
```

### Install a c compiler
On Fedora;
```
sudo dnf install gcc make
```

On Debian;
```
sudo apt install build-essential
```

### Sync UV (If installing from git repo)
```
uv sync
```