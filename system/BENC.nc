class init{
    func construct(){
        print(cat("1","2","3"),"lb")
        exec(combine(@scriptdir,"system/devtools.nc"))
        exec(cat(@scriptdir,"/system/Engine.nc"))
        exec(cat(@scriptdir,"/system/gui.nc"))
        exec(cat(@scriptdir,"/system/animations.nc"))
        //blueengine.constructer()
        //blueengine.pr("jemoeder")
        //print(iscode("blueengine__pr"))
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
    self.x = 0
    self.y = 0
    self.z = 4.1
    self.sx = 100.0
    self.sy = 100.0
    self.sz = -100.0
    self.movedtimer = timerinit()
    self.movementspeed = 0.2
}

class camera{
    func switch(towardschar){
        debugmode(1)
        print(cat("moving to char:",towardschar),"bp")
        player.x = *towardschar.x
        player.y = *towardschar.y
        player.z = *towardschar.z
        print(cat("pos:",*towardschar.x,"-",*towardschar.y,"-",*towardschar.z),"p")
        player.movedtimer = timerinit()
        blueengine.setposition("cursor",player.x ,player.y,player.z)
        camera.x = player.x
        camera.y = player.y - 3
        blueengine.setcamerapos(camera.x,camera.y,camera.z,player.x,math("player.y + 30"),math("player.z - 100"))
        //print(cat("PlayerDebugPos:x",player.x,",y",player.y,",z",player.z))
        moved = "false"
        debugmode(0)
        return towardschar    
    }
    camera "camera"
    camera.x = 0.0
    camera.y = 0.0
    camera.z = 10.0
    camera.targety = 2
    camera.targetx = 0
    camera.targetz = -10
    //print(object.display(self))
}

class bmpfont{

}
class spritetest{
    func load(sprite_name,texture_name){
        self.spriteids ++
        thisid = cat("nsprite",self.spriteids)

        blueengine.addsquare(thisid)
        .settexture(thisid,"blueengine_textures.resources_bmpfont_rngwhite_a_png")
        .setposition(thisid,player.x,player.y,player.z)

        print(thisid)
        
        tmp = thisid
        *tmp.animationi = 0
        *tmp.timeranimation = timerinit()
        *tmp.timerrotation = timerinit()
        *tmp.closetimer = timerinit()
        *tmp.animframetime = random(100,900)
        *tmp.rotor = 0
        animsarray = [
            "blueengine_textures.resources_bmpfont_rngwhite_a_png",
            "blueengine_textures.resources_bmpfont_rngwhite_b_png",
            "blueengine_textures.resources_bmpfont_rngwhite_c_png",
            "blueengine_textures.resources_bmpfont_rngwhite_d_png",
            "blueengine_textures.resources_bmpfont_rngwhite_e_png",
            "blueengine_textures.resources_bmpfont_rngwhite_f_png",
            "blueengine_textures.resources_bmpfont_rngwhite_g_png",
            "blueengine_textures.resources_bmpfont_rngwhite_h_png",
            "blueengine_textures.resources_bmpfont_rngwhite_i_png",
            "blueengine_textures.resources_bmpfont_rngwhite_j_png",
            "blueengine_textures.resources_bmpfont_rngwhite_k_png",
            "blueengine_textures.resources_bmpfont_rngwhite_l_png",
            ]
        coroutine thisid{
            this = self
            if *this.timeranimation != ""{
                if timerdiff(*this.timeranimation) > *this.animframetime {
                    *this.timeranimation = timerinit()
                    blueengine.settexture(this,animsarray[*this.animationi])
                    *this.animationi = *this.animationi + 1
                    if *this.animationi == animsarray[?]{
                        *this.animationi = 0
                    }
                }
            }
            if timerdiff(*this.timerrotation) > 120 {
                *this.rotor = *this.rotor + 0.1
                blueengine.setrotation(this,*this.rotor,"z")
                *this.timerrotation = timerinit()
            }
           if timerdiff(*this.closetimer) > 8000 {
                
                blueengine.delete(this)
                break this
            }
        }
    }
    func move(){
    //asd
    }
    self.spriteids = 0
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
movementspeed = 0.2
//print(inobj("nscript_loops"),"b")
//sprite.handler()
//print(cat("anims:",animationhandler.allsprites),"g")
currentplayer = "dude1"
tosetplayer = "dude1"
guistart()
co_i = 0
coroutine "postRTload"{
    if co_i > 0 {
        guistart()
        break self
    }
    co_i ++
}
coroutine "gameloop" {
    //print(cat("anims:",animationhandler.allsprites),"g")

    if key.event == "true"{
        if key.g == "down" && key.left = "down" {
            
            print(mypw,"g")
            menus.open("testmenu2")
        }
        if key.h == "down" && key.left = "down" {
            menus.open("testmenu")
        }
        if key.j == "down" && key.left = "down" {
            menus.open("testmenu|testmenu2")
        }
        if key.esc == "down"{
            menus.close()
        }
        if key.i == "down"{
            bta = inobj("blueengine_textures")
            toprint = combine "in texture obj=" bta "<"
            print(toprint,"r")
        }
        if key.left = "down"{
            camera.x = math camera.x - math("player.movementspeed / 10")
            cmoved = "true"
        }
        if key.right = "down"{
            camera.x = math camera.x + math("player.movementspeed / 10")
            cmoved = "true"
        }
        if key.up = "down"{
            camera.z = math camera.z - math("player.movementspeed / 10") - math("player.movementspeed / 10")
            cmoved = "true"
        }
        if key.down = "down"{
            camera.z = math camera.z + math("player.movementspeed / 10") + math("player.movementspeed / 10")
            cmoved = "true"
        }
        if key.n = "down"{
            camera.y = math camera.y - math("player.movementspeed / 10")
            cmoved = "true"
        }
        if key.m = "down"{
            camera.y = math camera.y + math("player.movementspeed / 10")
            cmoved = "true"
        }
        if cmoved == "true" {
            blueengine.setcamerapos(camera.x,camera.y,camera.z,camera.x,math("camera.y + 30"),math("camera.z - 100"))
            cmoved = "false"
            temp = cat camera.x "," camera.y "," camera.z
            //print(temp)
        }

        if key.k == "down"{
            //blueengine.settexture("square_2",combine("blueengine_textures.",stringtoeval("resources/testimg.png")))
        }

        if key.l == "down"{
            blueengine.settexture("square_2",combine("blueengine_textures.",stringtoeval("resources/testimg2.png")))
        }

        if key.a == "down"{
            *currentplayer.setanim("anim_left")
            player.lastmovedside = "left"
            player.x = math player.x - math("player.movementspeed / 10")
            moved = "true"
        }
        if key.d == "down"{
            *currentplayer.setanim("anim_right")
            player.lastmovedside = "right"
            player.x = math player.x + math("player.movementspeed / 10")
            moved = "true"
        }
        if key.w == "down"{
            *currentplayer.setanim("anim_up")
            player.lastmovedside = "up"
            player.y = math player.y + math("player.movementspeed / 10")
            moved = "true"
        }
        if key.s == "down"{
            *currentplayer.setanim("anim_down")
            player.lastmovedside = "down"
            player.y = math player.y - math("player.movementspeed / 10")
            moved = "true"
        }
        if key.q == "down"{
            player.z = math player.z - math("player.movementspeed / 10")
            moved = "true"
        }
        if key.e == "down"{
            player.z = math player.z + math("player.movementspeed / 10")
            moved = "true"
        }
        if key.p == "down"{
            player.sx = math player.sx + math("player.movementspeed / 10")
            blueengine.setscale("main",player.sx,player.sy,player.sy)
        }
        if key.o == "down"{
            //player.sx = math player.sx - movementspeed
            //blueengine.setscale("main",player.sx,player.sy,player.sy)
        }
        if key.x == "down" && key.left = "down"{
            devtools.runcode()
        }
        if moved == "true"{
            blueengine.setposition(currentplayer,player.x ,player.y,0.8)
            player.movedtimer = timerinit()
            blueengine.setposition("cursor",player.x ,player.y,player.z)
            camera.x = player.x
            camera.y = math player.y - 3
            blueengine.setcamerapos(camera.x,camera.y,camera.z,player.x,math("player.y + 30"),math("player.z - 100"))
            //print(cat("PlayerDebugPos:x",player.x,",y",player.y,",z",player.z))
            moved = "false"
        }
      if key.f == "down"{
        print(cat(@hour,":",@min,"-----------------------------"))
            print(cat("textureload_q ",blueengine.textureload_q),"g")
            print(cat("textureset_q ",blueengine.textureset_q),"g")
            print(cat("square_q ",blueengine.square_q),"g")
            print(cat("camera_q ",blueengine.camera_q),"g")
            print(cat("scale_q ",blueengine.scale_q),"g")
            print(cat("rotation_q ",blueengine.rotation_q),"g")
            print(cat("position_q ",blueengine.position_q),"g")
            print(cat("bmpfont_q ",blueengine.bmpfont_q),"bg")
            print(cat("anim_q ",blueengine.anim_q),"bg")
        }
    }
    if timerdiff(player.movedtimer) > 100{
        dude1.setanim(cat("anim_idle",player.lastmovedside))
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
