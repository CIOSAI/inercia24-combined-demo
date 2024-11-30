# 7z must exist in path
# or swap it out for "zip" if you are on linux

import shutil
import os
import datetime
import glob

current_time = datetime.datetime.now().strftime(r"%Y%m%d%H%M%S")

COPYFROM = [r"src\music", r"src\shaders", r"target\release", "."]
COPYWHAT = ["mp3", "comp", "exe", "bat", "nfo"]

os.makedirs("dist/"+current_time)

for i in COPYFROM:
    folder = current_time+"/"+i
    if not os.path.exists("dist/"+folder):
        os.makedirs("dist/"+folder) # dist/0000000/music

    files = []
    for j in COPYWHAT:
        files += glob.glob(i + r"\*." + j) # src/music, src/shaders, target/release X mp3, comp, exe
    
    for file in files:
        filename = os.path.split(file)[1] # song.mp3
        shutil.copyfile(file, "dist/"+folder+"/"+filename) # dist/0000000/music/song.mp3

os.system("7z a -tzip dist/" + current_time + ".zip " + "dist/" + current_time + "/*")