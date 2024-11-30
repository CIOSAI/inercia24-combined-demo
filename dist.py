import shutil
import os
import datetime
import glob

current_time = datetime.datetime.now().strftime(r"%Y%m%d%H%M%S")

COPYFROM = [r"src\music", r"src\shaders", r"target\release", "."]
COPYWHAT = ["mp3", "exe", "bat", "nfo"]

os.makedirs("dist/"+current_time)

for i in COPYFROM:
    folder = current_time+"/"+i
    if not os.path.exists("dist/"+folder):
        os.makedirs("dist/"+folder) # dist/0000000/music

    files = []
    for j in COPYWHAT:
        files += glob.glob(i + r"\*." + j) # src/music, src/shaders, target/release X mp3, exe
    
    for file in files:
        filename = os.path.split(file)[1] # song.mp3
        shutil.copyfile(file, "dist/"+folder+"/"+filename) # dist/0000000/music/song.mp3
    
    # move shaders separatedly
    shaders = glob.glob(i + r"\*.comp")
    for shader in shaders:
        command = "shader_minifier"
        param = []
        param.append("--format indented")
        param.append("--preserve-externals")
        param.append("--no-renaming-list hashi,main,seed")
        param.append(shader) #src/shaders/name.comp
        param.append("-o dist/" + folder + "/" + os.path.split(shader)[1]) #-o dist/00000/src/shaders/name.comp
        command += " " + " ".join(param)
        os.system(command)

shutil.make_archive("dist/" + current_time, "zip", root_dir = "dist/" + current_time)
# os.system("7z a -tzip dist/" + current_time + ".zip " + "dist/" + current_time + "/*")