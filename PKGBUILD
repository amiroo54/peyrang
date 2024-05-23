# Maintainer: amiroof <amiroo.f54@gmail.com>
pkgname=svg-pallet-changer
pkgver=1.0.0
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
    
    NO_STRIP=true cargo tauri build
}


package() {
    cd "${pkgname}/src-tauri/target/release/bundle/deb/${pkgname}_${pkgver}_amd64/data"

    for size in 128x128 256x256@2 512x512; do
        install -Dm644 "usr/share/icons/hicolor/${size}/apps/${pkgname}.png" "${pkgdir}/usr/share/icons/hicolor/${size}/apps/${pkgname}.png"
    done

    install -Dm644 "usr/share/desktop/${pkgname}.desktop" "${pkgdir}/usr/share/applications/${pkgname}.desktop"

    install -Dm755 "usr/bin/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"

    install -Dm755 "usr/share/scripts/update_system.sh" "${pkgdir}/usr/share/${pkgname}/scripts/update_system.sh"

    echo
    echo -e "${PURPLE}|=============================|${END}"
    echo -e "${WHITE}     Packaging Complete     ${END}"
    echo -e "${PURPLE}|=============================|${END}"
    echo
}