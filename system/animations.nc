class animationhandler{
    // system updates the sprites
    func construct(){
        coroutine animationhandler{
            break self
            return
            //print(cat("coroutine:",self))
            
            if timerdiff(self.animatetimer) > 24{

                self.animatetimer = timerinit()

                for chunk to self.proccesspritesperframe {
                    self.arraychunk ++
                    if self.arraychunk = animationhandler.allsprites[?] {
                        self.arraychunk = 0
                    }
                    thissprite = animationhandler.allsprites[self.arraychunk]
                    if thissprite != "" {
                        if timerdiff(*thissprite.animationtimer) > 100 {
                            //print(thissprite,"g")
                            reflectprop = *thissprite.currentanim

                            thisframe = cat("blueengine_textures.resources_characters_",*thissprite.name,"_",*thissprite.*reflectprop[*thissprite.currentframe],"_png")

                            //print(cat("setting ",thisframe),"y")
                            if thisframe != *thissprite.currentframe {
                                blueengine.settexture(thissprite,thisframe)
                            }
                            *thissprite.animationtimer = timerinit()

                            *thissprite.currentframe ++
                            if *thissprite.currentframe == *thissprite.*reflectprop[?]{
                                *thissprite.currentframe = 0
                            }
                        }
                    }
                }
            }
            

        }
    }
    func set(issprite){
        self.allsprites = arraypush(self.allsprites,issprite)
    }
    self.arraychunk = 0
    self.animatetimer = timerinit()
    self.allsprites = [""]
    self.proccesspritesperframe = 24 // ammount of sprites changed per tick
}

class animationhandler_testing{
    func construct(){
        self.testtimer = timerinit()
        self.proccesspritesperframe = 30 // ammount of sprites changed per tick
         coroutine self{
            //print(cat("coroutine:",self))
            
            if timerdiff(self.testtimer) > 3000{
                
                self.testtimer = timerinit()
                for chunk to self.proccesspritesperframe {
                    self.arraychunk ++
                    if self.arraychunk = animationhandler.allsprites[?] {
                        self.arraychunk = 0
                    }
                    thissprite = animationhandler.allsprites[self.arraychunk]
                    //print(thissprite)
                    if thissprite != "" {

                        *thissprite.random2 = *thissprite.random2 + random(1,2,0)                       
                        *thissprite.atime = timerinit()
                        if *thissprite.random2 == *thissprite.animationlist[?]{
                            *thissprite.random2 = 0
                        }

                        //print(cat("animhandler:changing:",thissprite," ran:",*thissprite.random2," to:",*thissprite.animationlist[*thissprite.random2]," list:",*thissprite.animationlist),"g")
                        if *thissprite.animationlist[*thissprite.random2] != ""{
                            //print(*thissprite.animationlist[*thissprite.random2])
                            *thissprite.setanim(*thissprite.animationlist[*thissprite.random2])
                        }     
                    }
                }
            }
        }
    }

}

class sprite{
    func construct(){
        if self != "sprite"{    
            spr = self 
            //print(cat("constructor! :",self))
            self.atime = timerinit()
            //print(cat("sprite:",self),"g")
            self.animationtimer = timerinit()
            self.animationlist = arraysearch(inobj(self),"anim_")   

            for eachanim in self.animationlist{
                prop = replace(eachanim,"anim_","texturearray_")
                self.*prop = "" // buffer array of the required frames to give to the engine to render
                checki = 0

                for eachframe in self.*eachanim{
                    if checki == 0{
                        self.*prop = cat self "|" eachframe
                    }
                    else{           
                        self.*prop = cat(self.*prop,"|","blueengine_textures.resources_characters_",self.name,"_",eachframe,"_png"))
                        
                    }
                    checki ++
                }
                //print(self.*prop,"r")
            }
            animationhandler.set(self)  
            blueengine.addsquare(spr).setposition(spr,1,-2,1).setrotation(spr,42,"x")   // check for debug !!! trow it up see if self changes whithin this scope to blueengine 
            
            return self
        }   
    }

    func load(dirname){
        self.path = cat("resources/characters/",dirname)
        self.name = dirname
        for x in listdir(self.path){
            if x != ""{
                if instring(x,".png") == 1 {
                    x2 = cat("resources/characters/",dirname,x)
                    blueengine.addtexture(x2)
                }
            }
        }
        exec(cat(@scriptdir,"resources/characters/",dirname,"/config.nc"))
        return dirname //as object reference
    }
    func setanim(animationname){
        if self.currentanim != animationname{
            self.currentanim = animationname
            self.currentframe = 0          
            prop = replace(animationname,"anim_","texturearray_")
            blueengine.setanim(self.*prop)
        }

    }
    func animationlist(){
        self.animationlist = arraysearch(inobj(self.name),"anim_")
        return self.animationlist 
    }
    func move(tox,toy,toz){
        
    }
    func setpos(tox,toy,toz){
        
        self.x = tox
        self.y = toy
        self.z = toz
        this = self
        blueengine.setposition(this,tox,toy,toz)
        
    }
    self.currentanim = "anim_unset"
    self.currentframe = 0 //<- arrayId not imagename
    self.animationtimer = timerinit()



}


charactersdir = listdir(cat(@scriptdir,"resources/characters/"))
for character in charactersdir{
    if character != ""{
        //print(cat("loadingchar:",character),"m")
        chr = sprite.load(character)
        
        randomx = random(-10,10)
        randomy = random(-10,3)
        *chr.setpos(randomx,randomy,0.8)
        *chr.atime = timerinit()

        //*chr.testroutine()
    //print(cat("in obj:",chr,@lf,inobj(chr)))

    }

}
ranchar = ["dude1","dude2","girl1"]
rani = 0
for xr to 250{
    //return
    cobj = cat("char_",xr)
    obj cobj : sprite
    obj *cobj : ranchar[rani]
    *cobj.construct()
    *cobj.animationlist = *cobj.animationlist()

    *cobj.animationlist = arrayshuffle(*cobj.animationlist)
    *cobj.setanim(*cobj.animationlist[0])
    *cobj.setpos(random(-25,50),random(-40,20),0.8)
    *cobj.atime = timerinit()
    //*cobj.testroutine()
    //print(cat("in obj:",cobj,@lf,inobj(cobj)))
    rani ++
    if rani == ranchar[?] {
       rani = 0
    }
}
func aaa(){
    return 123
}
aaa()