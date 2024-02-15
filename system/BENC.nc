class init{
    func construct(){
        exec(cat(@scriptdir,"/system/devtools.nc"))
        exec(cat(@scriptdir,"/system/engine/core.nc"))
        exec(cat(@scriptdir,"/system/engine/camera.nc"))
        exec(cat(@scriptdir,"/system/engine/gui.nc"))
        exec(cat(@scriptdir,"/system/engine/colission.nc"))
        exec(cat(@scriptdir,"/system/engine/animations.nc"))
        exec(cat(@scriptdir,"/system/engine/tools/modeleditor.nc"))
        exec(cat(@scriptdir,"/system/engine/tools/mapeditor.nc"))
        exec(cat(@scriptdir,"/system/engine/map/map.nc"))
        exec(cat(@scriptdir,"/system/engine/controls.nc"))
    }

}

class loadmods{
    func init(){
        for mod in dirlist("./mods/")

    }
}

class blueengine_textures{
    // used by engine
    //blueengine_textures = self
}

class key{
    key = "key"
}

class player{
    //player = "player"
    self.x = 20
    self.y = -15
    self.z = 4.1
    self.sx = 100.0
    self.sy = 100.0
    self.sz = -100.0
    self.movedtimer = timerinit()
    self.movementspeed = 0.4
}



class bmpfont{

}

//blueengine.setposition("main",0.0,0.0,0.0)

timer = timerinit()
print("starting Nscript")
timer2 = timerinit()
testx = 0.0
//.add()

testmove = 0.1
fpstimer = timerinit()
fps = 0
debugfps = "true"
print("BENC.nc loaded.")

//print(blueengine.textureload_q,"purple")
spawningtimer = timerinit()
testp = 2.0
blueengine.setposition("cursor",player.x,player.y,player.z)
movementspeed = 0.5
//print(inobj("nscript_loops"),"b")
//sprite.handler()
//print(cat("anims:",animationhandler.allsprites),"g")
currentplayer = "rocketguy"
playertoset = "cammy"
player.lastmovedside = ""
co_i = 0
cmoved = false
// trigger stuff after the everything is loaded
// routine will break after the first frame!
coroutine "postRTload"{
    if co_i > 0 {
        guistart()
        modeleditor.init()
        mapeditor.init()
        print("postloader:ready!")
        break "postRTload"
    }
    co_i ++
}
map.testmap()
// set the current control mode to a function inside the controls class
controlmode = "ingame"
*currentplayer.setpos(player.x ,player.y,0.4)
player.movedtimer = timerinit()
blueengine.setposition("cursor",player.x ,player.y,player.z)
blueengine.setcamerapos(camera.x,camera.y,camera.z,player.x,math("player.y + 30"),math("player.z - 100"))
coroutine "gameloop" {

    controls.*controlmode()
    if timerdiff(player.movedtimer) > 100{
        *currentplayer.setanim(cat("anim_idle",player.lastmovedside))
    }

    if debugfps == "true"{
        fpsc ++
        if timerdiff(fpstimer) > 999{
            fps = fpsc // 10
            //cwrite(combine("fps=",fps,key.event))
            fpsc = 0
            fpstimer = timerinit()
        }
    }

}
