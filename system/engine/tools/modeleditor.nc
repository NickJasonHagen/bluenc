class modeleditor{
    func init(){
        self.texturesinlist = self.alltextures
        modeleditorselectedtexture = ""
        self.alltextures = arraysearch(inobj(blueengine_textures),"resources_map_")
        print(alltextures)
        obj modeleditor : gui
        modeleditor.new("modeleditor","modeleditor")
        .addlabel("fps=","fps")
        .addcombo("Parts:","modeleditorselectedpart","modeleditor.buildingpool")
        .addlabel("selected=","modeleditorselectedpart")
        .addlabel("_______________________________")
        .addinput("posx","*modeleditorselectedpart.x").addinput("posy","*modeleditorselectedpart.y").addinput("posz","*modeleditorselectedpart.z")
        .addinput("rotx","*modeleditorselectedpart.rx").addinput("roty","*modeleditorselectedpart.ry").addinput("rotz","*modeleditorselectedpart.rz")
        .addinput("scalex","*modeleditorselectedpart.sx").addinput("scaley","*modeleditorselectedpart.sy").addinput("scalez","*modeleditorselectedpart.sz")
        .addbutton("update transform","modeleditor.settransform()")
        .addlabel("_______________________________")
        .addcombo("Squares:","modeleditor.selectedtexture","modeleditor.texturesinlist")
        .addlabel("selected=","modeleditor.selectedtexture")
        .addinput("Filter","modeleditor.filter")
        .addbutton("update filter","modeleditor.setfilter()")
        .addbutton("changeTexture","modeleditor.settexture(modeleditor.selectedtexture)")
        .addlabel("_______________________________")
        .addbutton("spawnnew","modeleditor.spawnsquare(modeleditor.selectedtexture)")
        .addbutton("copy current","modeleditor.copysquare()")
        .addbutton("delete current","modeleditor.deletesquare()")
        .savefilebox("save","modeleditor.modelfilename","modeleditor.save")
        self.squares = 0
    }
    func setfilter(){
        if self.filter != ""{
            self.texturesinlist = arraysearch(modeleditor.alltextures,self.filter)
        }
        else{
            self.texturesinlist = self.alltextures
        }     
    }
    func deletesquare(){
        t = modeleditorselectedpart
        self.buildingpool = poolremove(self.buildingpool,t)
        blueengine.delete(t)
    }
    func settexture(){
        t = modeleditorselectedpart
        *t.texture = cat("blueengine_textures.",modeleditor.selectedtexture)
        blueengine.settexture(t,*t.texture)
    }
    func spawnsquare(texture){
        self.squares ++
        self.newname = print(cat("modeleditor_objects_",self.squares),"g")
        self.texture = print(cat("blueengine_textures.",texture),"g")
        this = self
        buildingpartobj = self.newname
        *buildingpartobj.texture = self.texture
        self.buildingpool = pooladd(self.buildingpool,*this.newname)
        blueengine.addsquare(*this.newname).settexture(*this.newname,*this.texture).setposition(*this.newname,0,0,0)
    }
    func copysquare(){
        self.squares ++
        t = modeleditorselectedpart
        self.newname = print(cat("modeleditor_objects_",self.squares),"g")
        obj self.newname : *t
        *t.hrx = 0
        *t.hry = 0
        *t.hrz = 0
        self.texture = *t.texture//print(cat("blueengine_textures.",texture),"g")
        this = self
        buildingpartobj = self.newname
        *buildingpartobj.texture = self.texture
        self.buildingpool = pooladd(self.buildingpool,"__",*this.newname)
        blueengine.addsquare(*this.newname).settexture(*this.newname,*this.texture)
        .setposition(*this.newname,*t.x,*t.y,*t.z)
        .setrotation(*this.newname,*t.rx,"x")
        .setrotation(*this.newname,*t.ry,"y")
        .setrotation(*this.newname,*t.rz,"z")
        .setscale(*this.newname,*t.sx,*t.sy,*t.sz)
    }
    func settransform(){
        t = modeleditorselectedpart
        blueengine.setposition(t,*t.x,*t.y,*t.z)
        .setrotation(t,*t.rx,"x")
        .setrotation(t,*t.ry,"y")
        .setrotation(t,*t.rz,"z")
        .setscale(t,*t.sx,*t.sy,*t.sz)
    }
    func save(tosave){
        outputfile = ""
        for each in self.buildingpool{
            outputfile = cat outputfile *each.texture "," *each.x "," *each.y "," *each.z "," *each.rx "," *each.ry "," *each.rz "," *each.sx  "," *each.sy "," *each.sz "," *each.col @lf
        }
        print(cat("path:",tosave,"g"))
        print(fwrite(tosave,outputfile),"g)"
    }
    func load(asobject,file,posx,posy,posz){
        self.i = 0
        self.modeldata = fread(file)
         //store the meshbatch position
        *asobject.x = posx
        *asobject.y = posy
        *asobject.z = posz
        for part in split(self.modeldata,@lf){
            if part != ""{
                self.i ++
                thispartobject = cat(asobject,"__",self.i)
                self.partdata = split(part,",")
                self.newx = self.partdata[1] + posx
                self.newy = self.partdata[2] + posy
                self.newz = self.partdata[3] + posz
                s = self
                blueengine.addsquare(thispartobject)
                .settexture(thispartobject,*s.partdata[0])
                .setposition(thispartobject,*s.newx,*s.newy,*s.newz)
                .setrotation(thispartobject,*s.partdata[4],"x")
                .setrotation(thispartobject,*s.partdata[5],"y")
                .setrotation(thispartobject,*s.partdata[6],"z")
                .setscale(thispartobject,*s.partdata[7],*s.partdata[8],*s.partdata[9])
                *asobject.parts = pooladd(*asobject.parts,thispartobject)
            }
        }
    }

}

class newmodel{

}