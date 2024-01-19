
class blueengine{
    func setcamerapos(posx,posy,posz,targetx,targety,targetz){
        if posx == "" || posy == "" || posz == ""{
            return false
        }
        if targetz != ""{
            toset = cat posx "," posy "," posz "," targetx "," targety "," targetz 
        }
        else{
            toset = cat posx "," posy "," posz
        }
        
        self.camera_q = pooladd(self.camera_q,toset)
    }
    func setanim(id,animarray){
        if id == "" return
        self.anim_q = pooladd(self.anim_q,id)      
    }
    func addsquare(id,texture){
        if id == "" return
        self.square_q = pooladd(self.square_q,id)
    }
    func bmpfont(id){
        
        self.bmpfont_q = pooladd(self.bmpfont_q,id)
    }
    func addtexture(fileloc){
        if fileloc = "" return
        self.textureload_q = pooladd(self.textureload_q,fileloc)
    }

    func settexture(object,arg_texture){
        if object == "" return
        if arg_texture == "" return
        obj = combine(object,",",arg_texture)
        self.textureset_q = pooladd(self.textureset_q,obj)
    }

    func delete(object){
        if object == "" return
        self.deletion_q = pooladd(self.deletion_q,object)
    }

    func setposition(object,posx,posy,posz){
        if object = "" or object = self{
            return false
        }
        //print(cat("debug - setpos:",object),"r")
        toset = combine(object,",",posx,",",posy,",",posz)
        self.position_q = pooladd(self.position_q,toset);
        //print(toset)
    }

    func setscale(object,posx,posy,posz){
        if obj = "" return false
        toset = combine(object,",",posx,",",posy,",",posz)
        self.scale_q = pooladd(self.scale_q,toset);
    }

    func setrotation(object,rotation,axis){
        if axis != "x" && axis != "y" &&  axis != "z" || object == blueengine{
            return false
        }
        toset = combine(object,",",rotation,",",axis,",")
        self.rotation_q = pooladd(self.rotation_q,toset);
    }

    func construct(){
        self.square_q = ""
        self.textureload_q = ""
        self.textureset_q = ""
        self.scale_q = ""
        self.rotation_q = ""
        self.camera_q = ""
        self.deletion_q = ""
        self.anim_q = ""
        blueengine_textures = "blueengine_textures"
        bmpfontdir = listdir("./resources/bmpfont/rng_white/")
        for x in bmpfontdir{
            if x != ""{
                self.addtexture(cat("resources/bmpfont/rng_white/",x))
                //print(cat("bmpfonts:",x))
            }
        }
        textures = [
            "resources/img2.png",
            "resources/image.png",
            "resources/player.png",
            "resources/testimg.png",
            "resources/testimg2.png",
            "resources/grass1.png",
            "resources/grass_road_up.png",
            "resources/grass_road_side.png"
            ]
        for xt in textures {
            if xt != ""{
                self.addtexture(xt)
            }
        }
        for xt in listdir("resources/bmpfont/rngwhite") {
            if xt != ""{
                self.addtexture(cat("resources/bmpfont/rngwhite",xt))
            }
        }


        self.addsquare("main")
        self.addsquare("alt")
        self.addsquare("alt2")


        setx = 0
        sety = 0
        loadtimer = timerinit()
        tox = 2500
        cwrite(cat("loading ",tox," sqaures started"),"g")
        for x to tox{
            thisobj = cat("square_",x)
            blueengine.addsquare(thisobj)

            match setx{
                4 => {
                    ref = "blueengine_textures.resources_grass_road_up_png"
                }
                8 => {
                    ref = "blueengine_textures.resources_grass_road_up_png"
                }
                _ => {
                    ref = "blueengine_textures.resources_grass1_png"
                }

            }
            if sety == -14 or sety == - 8 {
                ref = "blueengine_textures.resources_grass_road_side_png"
            }
            if setx == 14 or setx == 8 {
                ref = "blueengine_textures.resources_grass_road_up_png"
            }
            blueengine.settexture(thisobj,ref)
            blueengine.setposition(thisobj,math("setx - 25"),math("sety + 20"))
            setx = math setx + 2
            if setx > 80 {
                setx = 0
                sety = math sety - 2
            }
        }
        loadtimerdiff = timerdiff(loadtimer)
        cwrite(cat("Loaded time:",loadtimerdiff),"g")
        blueengine.addsquare("cursor").setposition("cursor",0.0,0.0,10.0)
        ref = "blueengine_textures.resources_testimg2_png"
        blueengine.settexture("cursor",ref)
        //blueengine.bmpfont("fonttest").settexture("fonttest","blueengine_textures.resources_grass1_png").setposition("fonttest",20,10,-10).setscale("fonttest",0.1,0.1,10.0)

    }
    func pr(msg){
        print(cat("prr:","kaas",msg),"m")
    }
}
