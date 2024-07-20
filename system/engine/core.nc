
class blueengine{
    self.title = "testing"
    self.renderheight = 1080
    self.renderwidth = 1920
    self.render = "Vulkan"
}
class blueengine{
    func setcamerapos(posx,posy,posz,targetx,targety,targetz){
        if posx == "" || posy == "" || posz == ""{
            return false
        }

        camerasetposition(posx,posy,posz,targetx,targety,targetz)
        //self.camera_q = pooladd(self.camera_q,toset)
    }
    func setanim(id,animarray){
        if id == "" return
        self.anim_q = pooladd(self.anim_q,id)      
    }
    func addsquare(id,texture){
        if id == "" return
        nodespawnsquare(id)
        mytexture2 = textureload(texture)
        textureset(mynode,mytexture2)
        

    }

    func bmpfont(id){     
        self.bmpfont_q = pooladd(self.bmpfont_q,id)
    }

    func addtexture(fileloc){
        eval = stringtoeval(fileloc)
        if fileloc = ""{
            print("cannot add texture filelocation empty")
            return "error"
        }
        print(cat("addingtexture:",fileloc))
        mytexture2 = textureload(fileloc)
        //self.textureload_q = pooladd(self.textureload_q,fileloc)
    }

    func settexture(object,arg_texture){
        if object == "" || arg_texture == "" return
        obj = combine(object,",",arg_texture,",")
        textureset(object,texture)
        //self.textureset_q = pooladd(self.textureset_q,obj)
    }

    func setcolor(object,argb){
        if object == "" return
        if argb == "" return
        //obj = combine(object,",",argb)
        nodesetcolor(object,argb)
        //self.color_q = pooladd(self.color_q,obj)
    }

    func delete(object){
        if object == "" return
        nodedelete(object)
        //self.deletion_q = pooladd(self.deletion_q,object)
    }

    func setposition(object,posx,posy,posz){
        if object = "" || object = self{
            return false
        }

        nodesetposition(object,posx,posy,posx)
    }

    func setscale(object,posx,posy,posz){
        if obj = "" return false
        nodesetscale(object,posx,posy,posz)
    }

    func setrotation(object,rotation,axis){
        if axis != "x" && axis != "y" &&  axis != "z" || object == blueengine{
            return false
        }

        // translate the rotation..
        //yeah to keep the .props we gotta keep translating, as the BE rotates it instead of sets it
        // but this way we can just set it.
        tempaxis = cat("r",axis)
        //tempaxis2 = cat("hr",axis)

        //torotate = math 0 - *object.*tempaxis2 + rotation
        //*object.*tempaxis2 = rotation


        //*object.*tempaxis = rotation
        toset = combine(object,",",rotation,",",axis,",")
        nodesetrotation(object,rotation,axis)
        self.rotation_q = pooladd(self.rotation_q,toset)
    }

    func construct(){
        print("constructing engine test")
        blueengine_textures = "blueengine_textures"
        self.square_q = ""
        self.textureload_q = ""
        self.textureset_q = ""
        self.scale_q = ""
        self.rotation_q = ""
        self.camera_q = ""
        self.deletion_q = ""
        self.anim_q = ""
        isdir = print(listdir("./resources/map/"),"p")
        for xt in isdir{
            if xt != ""{
                self.addtexture(cat("resources/map/",xt))
            }
        }
        
        bmpfontdir = listdir("./resources/bmpfont/rng_white/")
        for x in bmpfontdir{
            if x != ""{
                self.addtexture(cat("resources/bmpfont/rng_white/",x))
                //print(cat("bmpfonts:",x))
            }
        }
        moretextures = listdir(cat(@nscriptpath,"/blueengine/resources/map"))
        for x in moretextures{
            if x != ""{
                self.addtexture(cat(@nscriptpath,"/blueengine/resources/map",x))
                //print(cat("bmpfonts:",x))
            }
        }

        textures = [
            "resources/img2.png",
            "resources/blood.png",
            "resources/image.png",
            "resources/player.png",
            "resources/testimg.png",
            "resources/testimg2.png",
            "resources/grass1.png",
            "resources/grass_road_up.png",
            "resources/grass_road_side.png"
            ]
        for xt in textures{
            if xt != ""{
                self.addtexture(xt)
            }
        }

        //self.addtexture("/home/skorm/ramdisk_home/nc.jpg")

            loadtimerdiff = timerdiff(loadtimer)
            cwrite(cat("Loaded time:",loadtimerdiff),"g")
            ref = "blueengine_textures.resources_testimg_png"
            blueengine.addsquare("cursor").setposition("cursor",0.0,0.0,10.0).settexture("cursor",ref)

            //blueengine.bmpfont("fonttest").settexture("fonttest","blueengine_textures.resources_grass1_png").setposition("fonttest",20,10,-10).setscale("fonttest",0.1,0.1,10.0)
       
    }
}

print(iscode("blueengine__addtextures"(),"r"))