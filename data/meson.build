# load resources
subdir('resources')

# Linux-specific deployment
if host_machine.system() == 'linux'
  # load icons
  subdir('icons')

  # creating desktop config
  desktop_conf = configuration_data()
  desktop_conf.set('APP_ID', application_id)
  configure_file(
    input: '@0@.desktop.in'.format(base_id),
    output: '@0@.desktop'.format(application_id),
    configuration: desktop_conf,
    install: true,
    install_dir: datadir / 'applications',
  )

  # service configuration
  service_conf = configuration_data()
  service_conf.set('APP_ID', application_id)
  service_conf.set('BINDIR', bindir)
  configure_file(
    input: '@0@.service.in'.format(base_id),
    output: '@0@.service'.format(application_id),
    configuration: service_conf,
    install: true,
    install_dir: datadir / 'dbus-1' / 'services',
  )
endif
