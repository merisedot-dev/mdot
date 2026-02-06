project('merisedot', 'rust', version: '0.1.0', meson_version: '>= 1.4')

# imports
gnome = import('gnome')

# Application ID (basic level)
base_id = 'com.github.merisedotdev.mdot'

# tweak variables depending of the type of build
is_devbuild = get_option('profile') == 'development'
if is_devbuild
  profile = 'Devel'
  application_id = '@0@.@1@'.format(base_id, profile)
else
  profile = ''
  application_id = base_id
endif

# build variables
bindir = get_option('prefix') / get_option('bindir')
localedir = get_option('prefix') / get_option('localedir')
datadir = get_option('prefix') / get_option('datadir')
pkgdatadir = datadir / meson.project_name()

# build subdirectories
subdir('data') # fetch extra build info
subdir('src') # build the damn code
# TODO loading translations

# Post-Install phase
gnome.post_install(
  gtk_update_icon_cache: true,
  glib_compile_schemas: true,
  update_desktop_database: true,
)
