class blueengine{
    self.title = "Multiple sprites and animations example"
    self.renderwidth = 1920
    self.renderheight = 1080
    self.powermode = false
    self.vsync = true
    self.render = "Vulkan"
    
}

class mysprites{
    func construct(){
        if self != "mysprites"{

            self.collision = true
            removecollisionpoint(self)
           // print(cat("new:",getcollisionpoint(self.collisionpoint)))

            //self.collisionpoint = ""
            spriteload(self,"resources/characters/cammy")
            self.getallanims = spritegetanimations(self)
            max = self.getallanims[?] - 1
            pickrandom = random(1,subtract(self.getallanims[?],1),0)
            spritesetanimation(self,self.getallanims[pickrandom])

            self.z = random(-20.0,-17.0,3)
            self.x = random(-27.0,27.0,3)
            self.y = random(-17.0,17.0,3)
            nodesetposition(self,self.x,self.y,self.z)
            print(cat("spawned:",self," colission:",self.collisionpoint))
            print(cat("x",self.x," y",self.y," z",self.z))
        }
    }

    func destruct(){
        spritedelete(self)
    }

}

// so we create a array of playersprites_1 till _13
// and instantiate the object to implement mysprites, 
//object will automaticly trigger the last defined construct function (can be redefined by another implement)
uniquenamearray = identifierarray("playersprite_",10)
maxspawnings = uniquenamearray[?]
func spawn(){
    spawncounter = 0
    coroutine "spawner"{
        obj uniquenamearray[spawncounter] : mysprites
        spawncounter ++
        if spawncounter >= maxspawnings{
            break "spawner"
        }
    }
}

func spawninstant(){
    for x in uniquenamearray{
        if x != ""{
            obj x : mysprites
        }
        
    }
}

func onhit(projectile,hitnode){
    print(cat("hitnode ",hitnode, " ", projectile))
    nodesetcolor(projectile,"0.70357174,0,0,1")
    nodesetcolor(hitnode,"0.70357174,0,0,1")
    print(cat("hit:",hitnode," colission:",*hitnode.collisionpoint))
    print(cat("x",*hitnode.x," y",*hitnode.y," z",*hitnode.z))
    print(cat("hit:",projectile," colission:",*projectile.collisionpoint))
    print(cat("x",*projectile.x," y",*projectile.y," z",*projectile.z))
}

spritesetanimation(mynode,"anim_runleft")
nodesetposition(mynode,2.0,0.0,-2.0)
mynode = spriteload("mynode","resources/characters/dude1")
spritesetanimation(mynode,"anim_idleleft")

mynode2 = spriteload("mynode2","resources/characters/dude1")
spritesetanimation(mynode2,"anim_left")
nodesetposition(mynode2,2.0,0.0,0.0)
pngfontobj = pngfontload("resources/pngfont/nscriptwhite/")
textnode("fps","0",-0.9,0.9,40.0,pngfontobj) 
fpsupdatetimer = timerinit()

mytexture = textureload("resources/BlueLogoDiscord.png")

shoottimer = timerinit()
//camerasetposition(camera.x,camera.y,camera.z,camera.x,camera.y,math("camera.z - 10"))
coroutine "gameloop"{
    diff = timerdiff(fpsupdatetimer)
    if diff > 999{
        textnodeupdate("fps",cat("fps:",blueengine.fps),-0.9,0.9,40.0,pngfontobj) 
        fpsupdatetimer = timerinit()
    }
    if key.event == true {
        if key.p == "down"{
            // because the objects identiefier is inside variable x you gotta reflect it using * 
            // otherwise it would destruct object x instead, but there is no object x
            // so to get the value use * to point to the data instead
            for x in uniquenamearray{
                *x.destruct()
            }
        }
        if key.space == "down"{
            if timerdiff(shoottimer) > 100 {
                shoottimer = timerinit()
                xprojectilei ++
                if xprojectilei == 10 {
                    xprojectilei = 0
                }
                projectileID = cat "xprojectile_" xprojectilei
                targetID = cat "playersprite_" xprojectilei
                rx = random(-10.0,10.0,3)
                ry = random(-10.0,10.0,3)
                xproj = nodeprojectile(projectileID,mytexture,0.0,0.0,0.0,*targetID.x,*targetID.y,*targetID.z,0.25)
                *xproj.onhit = "onhit"
                *xproj.collision = true
                *xproj.collisionpoint = ""
                
                print(xproj)
                print(cat(targetID,"=> ", *targetID.collisionpoint))
            }

        }
        if key.c == "down"{
            if timerdiff(shoottimer) > 100 {
                shoottimer = timerinit()
                xprojectilei ++
                if xprojectilei == 10 {
                    xprojectilei = 0
                }
                projectileID = cat "xprojectile_" xprojectilei
                targetID = cat "playersprite_" xprojectilei
                rx = random(-10.0,10.0,3)
                ry = random(-10.0,10.0,3)
                xproj = nodeprojectile(projectileID,mytexture,0.0,0.0,0.0,rx,ry,-30.0,0.25)
                *xproj.onhit = "onhit"
                *xproj.collision = true
                *xproj.collisionpoint = ""
                
                print(xproj)
            }

        }
        if key.o == "down" {
            spawn()
        }
         if key.i == "down" {
            colpos = split(playersprite_1.collisionpoint,"_")
            oldposref = playersprite_1.collisionpoint
            getold = getcollisionpoint(colpos[1],colpos[2],colpos[3])
            spawninstant()
            print(cat("playersprite_1,=> ", playersprite_1.collisionpoint))
            print(cat(oldposref," => @ ",getold))
            //getold = getcollisionpoint(colpos[1],colpos[2],colpos[3])
            print(cat(oldposref," =>new @ ",getcollisionpoint(colpos[1],colpos[2],colpos[3])))
        }
        if key.a == "down"{
            spritesetanimation(mynode,"anim_idleleft")
            nodesetposition(mynode2,-1.0)
            spritedelete("idle")
            nodedelete("idle")
        }
        if key.d == "down"{
            spritesetanimation(mynode,"anim_idleright")
            nodesetposition(mynode2,1.0)
        }
        if key.esc == "down"{
            run("./bluenc run examples/newgame.nc")
            break "gameloop"
        }
        if key.a == "down"{
            camera.x = camera.x - 0.1
            random(-10.0,10.0,3)
            
        }
        if key.d == "down"{
            camera.x = camera.x + 0.1
        }
        if key.w == "down"{
            camera.y = camera.y - 0.1
        }
        if key.s == "down"{
            camera.y = camera.y + 0.1
        }
        if key.up == "down"{
            camera.z = camera.z - 0.1
        }
        if key.down == "down"{
            camera.z = camera.z + 0.1
        }
        camerasetposition(camera.x,camera.y,camera.z,camera.x,camera.y,math("camera.z - 10"))
    }
}


