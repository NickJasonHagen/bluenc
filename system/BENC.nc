class init{
    func construct(){
        print(cat("1","2","3"),"bb")
        exec(cat(@scriptdir,"/system/Engine.nc"))
        blueengine.constructer()
        blueengine.pr("jemoeder")
        print(iscode("blueengine__pr"))
    }

}
class blueengine_textures{
    // used by engine
    //blueengine_textures = self
}

class sprite{
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

class key{
    key = "key"
}

class player{
    //player = "player"
    self.x = -1
    self.y = 0
    self.z = 0.1
    self.sx = 100.0
    self.sy = 100.0
    self.sz = -100.0
}

class camera{
    self.x = 0.0
    self.y = 0.0
    self.z = 20.0
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
exec(combine(@scriptdir,"system/devtools.nc"))
//print(blueengine.textureload_q,"purple")
spawningtimer = timerinit()
testp = 2.0
coroutine "gameloop" {

    if key.event = "true"{
        if key.g == "down" {
            if timerdiff(spawningtimer) > 100{
                testp = testp + 15
                blueengine.setposition("fonttest",0.0,2.0,testp)
                .settexture("fonttest","blueengine_textures.resources_bmpfont_rngwhite_b_png").setscale("fonttest",1.1,1.1,0.1)
                //sprite.load("a","blueengine_textures.resources_bmpfont_rngwhite_b_png")
                spawningtimer = timerinit()
            }
        }
        if key.h == "down" {
            if timerdiff(spawningtimer) > 100{
                testp = testp - 15
                blueengine.setposition("fonttest",0.0,2.0,testp)

                spawningtimer = timerinit()
            }
        }
        if key.i == "down"{
            bta = inobj("blueengine_textures")
            toprint = combine "in texture obj=" bta "<"
            print(toprint,"r")
        }
        if key.left = "down"{
            camera.x = math camera.x - 0.9
            cmoved = "true"
        }
        if key.right = "down"{
            camera.x = math camera.x + 0.9
            cmoved = "true"
        }
        if key.up = "down"{
            camera.z = math camera.z - 0.9
            cmoved = "true"
        }
        if key.down = "down"{
            camera.z = math camera.z + 0.9
            cmoved = "true"
        }
        if key.n = "down"{
            camera.y = math camera.y - 0.9
            cmoved = "true"
        }
        if key.m = "down"{
            camera.y = math camera.y + 0.9
            cmoved = "true"
        }
        if cmoved == "true" {
            blueengine.setcamerapos(camera.x,camera.y,camera.z)
            cmoved = "false"
            temp = cat camera.x "," camera.y "," camera.z
            //print(temp)
        }

        if key.k == "down"{
            blueengine.settexture("square_2",combine("blueengine_textures.",stringtoeval("resources/testimg.png")))
        }

        if key.l == "down"{
            blueengine.settexture("square_2",combine("blueengine_textures.",stringtoeval("resources/testimg2.png")))
        }

        if key.a == "down"{
            player.x = math player.x - 0.9
            moved = "true"
        }
        if key.d == "down"{
            player.x = math player.x + 0.9
            moved = "true"
        }
        if key.w == "down"{
            player.y = math player.y + 0.9
            moved = "true"
        }
        if key.s == "down"{
            player.y = math player.y - 0.9
            moved = "true"
        }
        if key.q == "down"{
            player.z = math player.z - 0.19
            moved = "true"
        }
        if key.e == "down"{
            player.z = math player.z + 0.9
            moved = "true"
        }
        if key.p == "down"{
            player.sx = math player.sx + 0.9
            blueengine.setscale("main",player.sx,player.sy,player.sy)
        }
        if key.o == "down"{
            player.sx = math player.sx - 0.9
            blueengine.setscale("main",player.sx,player.sy,player.sy)
        }
        if key.x == "down"{
            devtools.runcode()
        }
        if moved == "true"{
            blueengine.setposition("cursor",player.x,player.y,player.z)
            camera.x = player.x
            camera.y = math player.y - 0
            blueengine.setcamerapos(camera.x,camera.y,camera.z)
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
        }
    }

    if debugfps == "true"{
        fps ++
        if timerdiff(fpstimer) > 9999{
            cwrite(combine("fps=",math("fps / 10"),key.event))
            fps = 0
            fpstimer = timerinit()

        }

    }



}
