# Maintainer: amiroof <amiroo.f54@gmail.com>
pkgname=svg-pallet-changer
pkgver=1.1.0
pkgrel=1
pkgdesc="A simple application for replacing and permutating colors of an existing svg file."
arch=('any')
url="https://github.com/amiroo54/svg-pallet-changer"
license=('MIT')
depends=('cairo' 'desktop-file-utils' 'gdk-pixbuf2' 'glib2' 'gtk3' 'hicolor-icon-theme' 'pango' 'webkit2gtk')
makedepends=(
    "npm"
    "rustup"
    "git"
    "webkit2gtk"
    "base-devel"
    "curl"
    "wget"
    "openssl"
    "gtk3"
    "libappindicator-gtk3"
    "librsvg"
)
provides=("svg-pallet-changer")
conflicts=("svg-pallet-changer")
source=("$pkgname::git+https://github.com/amiroo54/svg-pallet-changer.git")
sha256sums=('SKIP')

build() {
    PURPLE=$(tput setaf 201)
    WHITE=$(tput setaf 255)
    END="\e[0m"
    cd "$pkgname"

    echo
    echo -e "${PURPLE}|============================|${END}"
    echo -e "${WHITE}   Installing Rust Nightly     ${END}"
    echo -e "${PURPLE}|============================|${END}"

    rustup toolchain install nightly
    rustup default nightly

    echo
    echo -e "${PURPLE}|=============================|${END}"
    echo -e "${WHITE}   Gathering UI Dependencies     ${END}"
    echo -e "${PURPLE}|=============================|${END}"

    npm install

    echo
    echo -e "${PURPLE}|=============================|${END}"
    echo -e "${WHITE}     Compiling Application     ${END}"
    echo -e "${PURPLE}|=============================|${END}"
    
    cargo tauri build
}


package() {
    cd "${pkgname}/src-tauri/target/release/bundle/deb/${pkgname}_${pkgver}_amd64/data"
    for size in 128x128 256x256@2; do
        sudo install -Dm644 "usr/share/icons/hicolor/${size}/apps/${pkgname}.png" "/usr/share/icons/hicolor/${size}/apps/${pkgname}.png"
    done

    sudo install -Dm644 "usr/share/applications/${pkgname}.desktop" "/usr/share/applications/${pkgname}.desktop"

    sudo install -Dm755 "usr/bin/${pkgname}" "/usr/bin/${pkgname}"

    echo
    echo -e "${PURPLE}|=============================|${END}"
    echo -e "${WHITE}     Packaging Complete     ${END}"
    echo -e "${PURPLE}|=============================|${END}"
    echo
}