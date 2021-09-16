import io
import os
import colorama
import shutil
import tqdm
import requests
import subprocess
import zipfile

GREEN = colorama.Fore.GREEN
BLUE = colorama.Fore.BLUE
RESET = colorama.Style.RESET_ALL


def exec(cmd):
  print(f'\n  {BLUE}> {GREEN}{" ".join(cmd)}{RESET}\n')
  subprocess.run(cmd)

def build_depotdownloader():
  exec([
    'docker', 'run', '--rm', '-it',
    '--volume', f'{os.getcwd()}/DepotDownloader:/artifacts',
    'mcr.microsoft.com/dotnet/sdk:5.0',
    'bash', '-c',
    '''
        git clone https://github.com/SteamRE/DepotDownloader && 
        cd DepotDownloader &&
        dotnet publish -r win-x64 --self-contained true -o /artifacts
    '''
  ])

def build():
  colorama.init()

  cleanup = True
  fetch_tools = True
  build_crate = True

  if cleanup:
    if os.path.exists('build'):
      shutil.rmtree('build', ignore_errors=True)
    os.mkdir('build')
    os.mkdir('build/tmp')
    os.mkdir('build/package')
  
  if fetch_tools:
    os.chdir('build/tmp')

    print(f'\n  {BLUE}* {GREEN}Fetching and building DepotDownloader...{RESET}\n')

    exec([
      'docker', 'run', '--rm', '-it',
      '--volume', f'{os.getcwd()}/DepotDownloader:/artifacts',
      'mcr.microsoft.com/dotnet/sdk:5.0',
      'bash', '-c',
      'git clone https://github.com/SteamRE/DepotDownloader && ' +
      'cd DepotDownloader && '+
      'dotnet restore && '+
      'dotnet publish -r win-x64 -c Release --self-contained true -o /artifacts ' +
      '/p:PublishTrimmed=true /p:TrimMode=link'
    ]) 


    print(f'\n  {BLUE}* {GREEN}Fetching ManifestPatcher...{RESET}\n')
    resp = requests.get('https://api.github.com/repos/fifty-six/zig.SteamManifestPatcher/releases').json()
    asset = resp[0]['assets'][0]
    filename = asset['name']
    smp_exe = requests.get(asset['browser_download_url'], stream=True)
    with open('SteamDepotDownpatcher.exe', 'wb') as fp:
        for chunk in tqdm.tqdm(smp_exe.iter_content(chunk_size=128*1024)):
            if chunk:
                fp.write(chunk)

    os.chdir('..')
    shutil.copytree('tmp/DepotDownloader', 'package/darksouls3-downloader-tools')
    shutil.copy('tmp/SteamDepotDownpatcher.exe', 'package/darksouls3-downloader-tools')
    os.chdir('..')

  if build_crate:
    exec(['cargo', 'build', '--release'])
    shutil.copy('target/release/darksouls3-downloader.exe', 'build/package')

  shutil.make_archive('build/darksouls3-downloader', 'zip', 'build/package')

if __name__ == '__main__':
  build()
