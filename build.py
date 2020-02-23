import os
import colorama
import shutil
import subprocess

GREEN = colorama.Fore.GREEN
BLUE = colorama.Fore.BLUE
RESET = colorama.Style.RESET_ALL

def exec(cmd):
  print(f'\n  {BLUE}> {GREEN}{" ".join(cmd)}{RESET}\n')
  subprocess.run(cmd)

def build():
  colorama.init()

  cleanup = True
  build_depot_downloader = True
  build_crate = True

  if cleanup:
    if os.path.exists('build'):
      shutil.rmtree('build', ignore_errors=True)
    os.mkdir('build')
    os.mkdir('build/tmp')
    os.mkdir('build/package')
  
  if build_depot_downloader:
    os.chdir('build/tmp')
    exec(['git', 'clone', 'https://github.com/veeenu/DepotDownloader'])
    os.chdir('DepotDownloader')
    exec(['dotnet', 'restore'])
    exec(['dotnet', 'publish', '-c', 'Release', '-r', 'win-x64', '--self-contained', 'true'])
    os.chdir('../..')
    shutil.copytree('tmp/DepotDownloader/DepotDownloader/bin/x64/Release/netcoreapp2.0/win-x64/publish', 'package/DepotDownloader')
    os.chdir('..')

  if build_crate:
    exec(['cargo', 'build', '--release'])
    shutil.copy('target/release/darksouls3-downloader.exe', 'build/package')


if __name__ == '__main__':
  build()
