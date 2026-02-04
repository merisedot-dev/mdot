project('mdot', 'rust', version: '0.1.0', meson_version: '>= 1.4')

# imports
gnome = import('gnome')

# Application ID
base_id = 'com.github.merisedot-dev/mdot'

# build variables
bindir = get_option('prefix') / get_option('bindir')
localedir = get_option('prefix') / get_option('localedir')
datadir = get_option('prefix') / get_option('datadir')
pkgdatadir = datadir / meson.project_name()

# build subdirectories
subdir('src') # build the damn code
subdir('data') # fetch extra build info

# Post-Install phase
gnome.post_install(
  gtk_update_icon_cache: true,
  glib_compile_schemas: true,
  update_desktop_database: true,
)
