class blueengine{
    self.title = "Multiple sprites and animations example"
    self.renderwidth = 1920
    self.renderheight = 1080
    self.powermode = false
    self.vsync = false
    self.render = "Vulkan"
}

class mysprites{
    func construct(){
        if self != "mysprites"{
            spriteload(self,"resources/characters/cammy")
            self.getallanims = spritegetanimations(self)
            max = self.getallanims[?] - 1
            pickrandom = random(1,subtract(self.getallanims[?],1),0)
            spritesetanimation(self,self.getallanims[pickrandom])
            self.x = random(-27.0,27.0,3)
            self.y = random(-17.0,17.0,3)
            self.z = random(-20.0,-17.0,3)
            nodesetposition(self,self.x,self.y,self.z)
            print(cat("spawned:",self))
        }
    }

    func destruct(){
        spritedelete(self)
    }
}

// so we create a array of playersprites_1 till _13
// and instantiate the object to implement mysprites, 
//object will automaticly trigger the last defined construct function (can be redefined by another implement)
uniquenamearray = identifierarray("playersprite_",150)
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
        if key.o == "down" {
            spawn()
        }
         if key.i == "down" {
            spawninstant()
        }
        if key.a == "down"{
            spritesetanimation(mynode,"anim_idleleft")
            nodesetposition(mynode2,-2.0)
            spritedelete("idle")
            nodedelete("idle")
        }
        if key.d == "down"{
            spritesetanimation(mynode,"anim_idleright")
            nodesetposition(mynode2,2.0)
        }
        if key.esc == "down"{
            run("./bluenc run examples/newgame.nc")
            break "gameloop"
        }
    }
}