# MeriseDot

A RAD tool inspired by the likes of [Cambalache] and [Gaphor] and also built in GTK4. It was made for the *Merise* methodology of building databases.

## Installation

### From source

Before building, ensure you have the following dependencies installed (how, I don't care, they just need to be installed on your system) :

- [meson]
- [ninja]
- [Rust]
- [GTK4]
- [libadwaita]
- [gettext] due to a `gettext-rs` quirk

Then, clone this repository. Next, run one of these commands (as of now, directory creation isn't handled, this is still in progress) :

```shell
sudo just mkProd # if you have the just command installed
meson setup build --prefix=~./local && meson install -C build
```

[Cambalache]: https://github.com/xjuan/cambalache
[Gaphor]: https://github.com/gaphor/gaphor
[meson]: https://mesonbuild.com/
[ninja]: https://ninja-build.org/
[Rust]: https://rust-lang.org/fr/
[GTK4]: https://docs.gtk.org/gtk4/
[libadwaita]: https://gnome.pages.gitlab.gnome.org/libadwaita/doc/
[gettext]: https://www.gnu.org/software/gettext/
