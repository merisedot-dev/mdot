# Some extra commands to ensure we have everything needed

mkbin:
    meson setup build --reconfigure && meson install -C build
