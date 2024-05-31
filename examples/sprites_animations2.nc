class blueengine{
    self.title = "Multiple sprites and animations example"
    self.renderwidth = 1280
    self.renderheight = 720
    self.powermode = true
    self.vsync = false
}

class mysprites{
    func construct(){
        if self != "mysprites"{
            spriteload(self,"resources/characters/dude1")
            self.getallanims = spritegetanimations(self)
            max = self.getallanims[?] - 1
            pickrandom = random(2,max,0)
            spritesetanimation(self,self.getallanims[pickrandom])
            ranx = random(-27.0,27.0,3)
            rany = random(-17.0,17.0,3)
            ranz = random(-20.0,-17.0,3)
            nodesetposition(self,ranx,rany,ranz)
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
uniquenamearray = identifierarray("playersprite_",500)
for x in uniquenamearray{
    obj x : mysprites
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
    }
}