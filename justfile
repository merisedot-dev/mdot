# Some extra commands to ensure we have everything needed

# ------------ BUILD COMMANDS ------------

mkdev: # build dev binary
    meson setup build --reconfigure -Dprofile=development --prefix=~/.local && meson install -C build

mkprod: # build production-ready binary
    meson setup build --reconfigure --prefix=~/.local && meson install -C build

# ------------ I18N COMMANDS -------------

fetchtxt: # define a potfile from sources
    xgettext --files-from=po/POTFILES.in --output=po/merisedot.pot

translate lang: # define a po translation file from pot file
    msginit -i po/merisedot.pot -o po/{{lang}}.po -l {{lang}}.UTF-8

uplang lang: # update an existing translation file
    msgmerge -U po/{{lang}}.po po/merisedot.pot
