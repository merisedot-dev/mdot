# Some extra commands to ensure we have everything needed

mkdev: # build dev binary
    meson setup build --reconfigure --prefix=~/.local && meson install -C build
