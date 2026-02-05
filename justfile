# Some extra commands to ensure we have everything needed

mkdev: # build dev binary
    meson setup build --reconfigure -Dprofile=development --prefix=~/.local && meson install -C build

mkprod: # build production-ready binary
    meson setup build --reconfigure --prefix=~/.local && meson install -C build
