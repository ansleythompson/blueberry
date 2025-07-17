# WOR Fork
Look at [WOR Repo](https://github.com/NumberOneGit/rpi5-uefi) for details.
# Building
This process assumes a Linux machine. On Windows, use WSL.

1. Install required packages:

   For Ubuntu/Debian:
   ```bash
   sudo apt install git gcc g++ build-essential gcc-aarch64-linux-gnu iasl python3-pyelftools uuid-dev
   ```
   For Arch Linux:
   ```bash
   sudo pacman -Syu
   sudo pacman -S git base-devel gcc dtc aarch64-linux-gnu-binutils aarch64-linux-gnu-gcc aarch64-linux-gnu-glibc python python-pyelftools iasl --needed
   ```

2. Clone the repository:
   ```bash
   git clone --recurse-submodules https://github.com/worproject/rpi5-uefi.git
   cd rpi5-uefi
   ```

3. Build the image:
   ```bash
   ./build.sh
   ```
   Append `--help` for more details.

If you get build errors, it is very likely that you're still missing some dependencies. The list of packages above is not complete and depending on the distro you may need to install additional ones. In most cases, looking up the error messages on the internet will point you at the right packages.

## Licenses
Most files are licensed under the default EDK2 license, [BSD-2-Clause-Patent](https://github.com/tianocore/edk2/blob/master/License.txt).

For TF-A, see: <https://github.com/ARM-software/arm-trusted-firmware/blob/master/docs/license.rst>
