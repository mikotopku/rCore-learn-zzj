import os

base_address = 0x80400000
step = 0x20000

apps = os.listdir('src/bin')
apps.sort()
for app in apps:
    app = app[:app.find('.')]
    os.system('cargo build --bin %s --release' % app)
    print('[build.py] application %s' % app)
