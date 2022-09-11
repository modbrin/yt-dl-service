# Maintainer: Maksim Surkov <modbrin@live.com>
pkgname='yt-dl-service-git'
_pkgname="yt-dl-service"
pkgver=r11.861906d
pkgrel=1
pkgdesc="Service-style wrapper for yt-dlp"
arch=('x86_64')
url="https://github.com/modbrin/yt-dl-service"
license=('UNLICENSE')
depends=('yt-dlp')
makedepends=('rust' 'cargo' 'git' 'sed')
source=("$_pkgname::git+https://github.com/modbrin/yt-dl-service.git")
install='yt-dl-service.install'
md5sums=('SKIP')

pkgver() {
	cd "$_pkgname"
	printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
  cd "$_pkgname"
  env CARGO_INCREMENTAL=0 cargo build --release --locked
}

check() {
  cd "$_pkgname"
  env CARGO_INCREMENTAL=0 cargo test --release
}

package() {
	cd "$_pkgname"
	install -Dm755 "$srcdir/$_pkgname/target/release/yt-dl-service" "$pkgdir/usr/bin/yt-dl-service"
	install -Dm644 "$srcdir/$_pkgname/templates/settings.json" "$pkgdir/usr/share/yt-dl-service/settings.json"
	touch "$srcdir/$_pkgname/templates/yt-dl-service.log"
	install -Dm644 "$srcdir/$_pkgname/templates/yt-dl-service.log" "$pkgdir/usr/share/yt-dl-service/yt-dl-service.log"
	install -Dm644 "$srcdir/$_pkgname/templates/yt-dl-service.service" "$pkgdir/usr/lib/systemd/system/yt-dl-service.service"
	install -Dm644 "$srcdir/$_pkgname/templates/yt-dl-service.timer" "$pkgdir/usr/lib/systemd/system/yt-dl-service.timer"
}
