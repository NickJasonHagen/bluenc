class animationhandler{
    // system updates the sprites
    func construct(){

    }
    func set(issprite){
        self.allsprites = pooladd(self.allsprites,issprite)
    }
    self.arraychunk = 0
    self.animatetimer = timerinit()
    self.allsprites = [""]
    self.proccesspritesperframe = 24 // ammount of sprites changed per tick
}

class animationhandler_testing{
    func constructa(){
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
            self.animationswitchtimer = 0//timerinit() 
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
            blueengine.addsquare(spr).setposition(spr,1,-2,0.25).setscale(spr,7.5,7.5,0).setrotation(spr,42,"x")   // check for debug !!! trow it up see if self changes whithin this scope to blueengine 
            
            return self
        }   
    }

    func load(dirname){

        self.path = print(cat("resources/characters/",dirname))
        self.name = dirname
        for self.xr in listdir(self.path){
            if self.xr != ""{
                if instring(self.xr,".png") == true {
                    x2 = cat("resources/characters/",dirname,self.xr)
                    print(x2)
                    blueengine.addtexture(x2)
                }
            }
        }
        exec(cat(@scriptdir,"resources/characters/",dirname,"/config.nc"))
        return dirname
    }
    func setanim(animationname){
        if self.currentanim != animationname && timerdiff(self.animationswitchtimer) > 440 {
            self.currentanim = animationname
            self.currentframe = 0
            self.animationswitchtimer = timerinit()          
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
        //this = self
        colissions.set(self,tox,toy,toz)
        nodesetposition(self,tox,toy,toz)        
    }
    self.x = 0.0
    self.y = 0.0
    self.z = 0.0
    self.currentanim = "anim_unset"
    self.currentframe = 0 //<- arrayId not imagename
    self.animationtimer 0//= timerinit()
}


    charactersdir = listdir(cat(@scriptdir,"resources/characters/"))
    for character in charactersdir{
        if character != "" {
            print(cat("loadingchar:",character),"m")
            chr = sprite.load(character)
            print(cat("loadedsprite:",chr))
            *chr.animationlist = *chr.animationlist()
            //blueengine
            //*chr.animationlist = arrayshuffle(*chr.animationlist)
            *chr.setanim(*chr.animationlist[1])          
            randomx = random(-10,10)
            randomy = random(-10,3)
            *chr.setpos(randomx,randomy,0.4)
            //blueengine.setrotation(chr,42,"x")
            *chr.atime = timerinit()

            //*chr.testroutine()
        print(cat("in obj:",chr,@lf,inobj(chr)))

        }

    }
    ranchar = ["dude1","dude2","girl1"]
    rani = 1
    for xr to 5{
        //return
        cobj = cat("char_",xr)
        obj cobj : sprite
        obj *cobj : ranchar[rani]
        //*cobj.construct()
        *cobj.animationlist = *cobj.animationlist()
        //blueengine
        *cobj.animationlist = arrayshuffle(*cobj.animationlist)
        *cobj.setanim(*cobj.animationlist[0])
        *cobj.setpos(random(-25,50),random(-40,20),0.4)//.setrotation(cobj,42,"x")
        *cobj.atime = timerinit()
        //*cobj.testroutine()
        //print(cat("in obj:",cobj,@lf,inobj(cobj)))
        rani ++
        if rani == ranchar[?] {
        rani = 0
        }
    }
