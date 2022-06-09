# Linux on drugs🧪

![](images/1.gif)

Gives custom drugs to your terminal 💊

Output random colors really fast

# Installation 📦

## Arch Linux 🐧

Linux on drugs is in the AUR

```bash
yay -S linux-on-drugs
```

## Other 🪟🐧

### With make - Linux 🐧

Build with make

```bash
# 📂 linux-on-drugs/
make
```

Move the binary

```bash
# 📂 linux-on-drugs/
make install
```

### Build from source - Linux 🐧 & Windows 🪟

**Clone this repo**

```bash
git clone https://github.com/SkwalExe/linux-on-drugs.git
```

build with [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

```bash
# 📂 linux-on-drugs/
cargo build --release
```

**[ LINUX ONLY ] :** Move the binary

```bash
# 📂 linux-on-drugs/
sudo cp target/release/linux-on-drugs /usr/bin/linux-on-drugs
```

**On windows** the executable will be `target\release\linux-on-drugs.exe` you can move it wherever you want.

# Usage 📝

![](images/2.png)

## --block-size

The `--block-size` option is used to set the size of each color block.

The number corresponds to the number of space/custom characters you want to use per color block.

```bash
linux-on-drugs --block-size 20
``` 

![](images/3.gif)

## --content

The `--content` option is used to set the content of each color block.

It is a space by default but you can use any custom characters.

```bash
linux-on-drugs --content "*"
```

![](images/4.gif)

# How it works 🔬

Very simple, the program select a color number from 1 to 255 and print a space with this background color.

# Uninstall 🗑

## With make

Run make uninstall

```bash
# 📂 linux-on-drugs/
make uninstall
```

## removing the binary

```bash
sudo rm /usr/bin/linux-on-drugs
```

# Docker 🐳

## Run the latest version

```bash
docker run --rm -it ghcr.io/skwalexe/linux-on-drugs:main
```

## Test your changes 🚧

### Build 🛠️

```bash
# 📂 linux-on-drugs/
docker build -t linux-on-drugs .
```

### Run 🏃

```bash
docker run --rm -it linux-on-drugs [OPTIONS]
```

# Change log 📝

For version updates and bug fixes, please see our [CHANGELOG](CHANGELOG.md)

# final

If you have any problem, don't hesitate to open an issue

# contributing

1. Start by [**forking** this repository](https://github.com/SkwalExe/linux-on-drugs/fork)

2. Then clone your fork to your local machine.
  ```git
  git clone https://github.com/your-username/linux-on-drugs.git
  ```

3. Create a new branch
  ```git
  git checkout -b super-cool-feature
  ```

4. Then make your changes

5. Update the changelog and version number if needed inside `Cargo.toml` (using [Semantic Versioning](https://semver.org)) and execute `cargo run` to update the version number inside `Cargo.lock`

6. Once you're done, commit your changes and push them to the remote repository.
  ```git
  git add --all
  git commit -m "Add super-cool-feature"
  git push origin super-cool-feature
  ```

7. Then, open a pull request on GitHub from your fork.
    1. Go to [this link](https://github.com/SkwalExe/linux-on-drugs/compare/)
    2. Click compare across forks
    3. On the right, on `head repository` select your fork
    4. And on `compare` select the branch you just created
    5. Click on `Create Pull Request` and submit your pull request

<a href="https://github.com/SkwalExe#ukraine"><img src="https://raw.githubusercontent.com/SkwalExe/SkwalExe/main/ukraine.jpg" width="100%" height="15px" /></a>