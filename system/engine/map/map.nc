class map{
    func testmap(){
        setx = 0
        sety = 0
        setz = 0
        loadtimer = timerinit()
        tox = 40000
        toy = tox / 100 //yeah its already half units
        cwrite(cat("loading ",tox," sqaures started"),"g")
        for x to tox{
            thisobj = cat("square_",x)
            //blueengine.addsquare(thisobj)

            match setx{
                4 => {
                    ref = cat @scriptdir "/models/grass1.3dbnc"//"blueengine_textures.resources_grass_road_up_png"
                }
                8 => {
                    ref = cat @scriptdir "/models/tree.3dbnc"//"blueengine_textures.resources_grass1_png"
                }
                _ => {
                    ranarray = [
                        "grass1",
                        "grass1",
                        "grass",
                        "grass",
                        "grass",
                        "tree",
                        "tree"
                        ]
                        max = ranarray[?] - 1
                    rand = random(0,max,0)
                    //print(cat("pick:",ranarray[rand]))
                    ref = cat @scriptdir "models/" ranarray[rand] ".3dbnc"//"blueengine_textures.resources_grass1_png"//blueengine_textures.resources_grass1_png
                }

            }
            if 1 == 1 {
                if sety == -14 or sety == -8 {
                    ref =  cat @scriptdir "models/tree.3dbnc"
                }
                if sety == -12 {
                ref =  cat @scriptdir "models/grass_road_horizontal.3dbnc" 
                }
                if setx == 14 or setx == 8 {
                    ref =  cat @scriptdir "models/tree.3dbnc"
                }
                if setx == 16 {
                ref =  cat @scriptdir "models/grass_road_vertical.3dbnc" 
                }
            }

            //blueengine.settexture(thisobj,ref)
            //blueengine.setposition(thisobj,math("setx - 25"),math("sety + 20"))
            //modeleditor.load(thisobj,ref,setx,sety,0.0)
            //bn3load(thisobj,ref)
            //make em floats
            newx = cat setx ".0"
            newy = cat sety ".0"
            //mappos = cat("map_maptile_",replace(setx,"-","m"),"_",replace(sety,"-","m"))
            mappos = self.tileid(setx,sety) 
            self.*mappos = ref
            //print(cat("setting mappos:",mappos," with ref:",ref))
            //bn3setposition(thisobj,newx,newy,0.0)
            setx = math setx + 2
            if setx > toy {
                print(cat("loading:",x,"/",tox," ->",self.*mappos))
                setx = 0
                sety = math sety - 2
            }
        }
        skydometexture = print(cat("blueengine_textures.",stringtoeval(cat(@nscriptpath,"/blueengine/resources/map/","night.jpg"))))
        //blueengine.addsquare("skydome").settexture("skydome",skydometexture).setposition("skydome",30,60,30).setrotation("skydome",75,"x").setscale("skydome",120,120,0)
        
        self.calculatemappos().setviewrange(16,10).setmapview(self.calcx,self.calcy).loadmappart()
    }
    func tileid(posx,posy){
        return cat("map_maptile_",replace(posx,"-","m"),"_",replace(posy,"-","m"))
    }

    func loadmappart(){

        for rowy to self.renderheight{
            for rowx to self.renderheight{
                doublex = rowx * 2
                doubley = 0 - rowy * 2
                newx = self.view_x + doublex
                newy = self.view_y + doubley
                tilename = self.tileid(newx,newy)
                ref = print(self.*tilename)
                print(cat("loadingTile:",ref," tilename:",tilename),"g")
                bn3load(tilename,ref)
                bn3setposition(tilename,newx,newy,0.0)
            }
        }
    }
    
    func setmapview(posx,posy){
        self.view_x = posx
        self.view_y = posy
        self.beginrowx = self.view_x - self.viewwidth
        self.endrowx = self.view_x + self.viewwidth
        self.beginrowy = self.view_y + self.viewheight - self.cameraoffsetz
        self.endrowy = self.view_y - self.viewheight - self.cameraoffsetz       

    }
    func setviewrange(rangex,rangey){
        self.viewwidth = rangex
        self.viewheight = rangey
        self.renderwidth = self.viewwidth * 2
        self.renderheight = self.viewheight * 2
    }
    func calculatemappos(){
        self.calcx = nearest_even(player.x)
        self.calcy = nearest_even(player.y)
        self.changed = false
            if self.calcx > math("*self.view_x + 1") {
                self.mapmoveright()
                self.changed = true
            }
            if self.calcx < math("*self.view_x - 1"){
                self.mapmoveleft()
                self.changed = true
            }
            if self.calcy < self.view_y {
                self.mapmovedown()
                self.changed = true
            }
            if self.calcy > self.view_y {
                self.mapmoveup()
                self.changed = true
            }
            if self.changed == true {
                self.setmapview(self.calcx,self.calcy)
                self.clearcorners()
            }
        
    }
    func mapmoveright(){
        print(cat("wiping:",self.beginrowy))
        for row to self.renderheight{
            rowc = self.beginrowy - row - 1
            tilename = self.tileid(self.beginrowx,rowc)
            //print(cat("deleting:",tilename))
            bn3delete(tilename)
            tilename = self.tileid(self.endrowx,rowc)
            bn3delete(tilename)
            bn3load(tilename,self.*tilename)
            bn3setposition(tilename,self.endrowx,rowc,0.0)
            //self.setmapview(mat("self.view_x + 2"),self.view_y)
            //print(cat("spawning:",tilename))
        }
    }
    
    func mapmoveleft(){
        print(cat("wiping:",self.endrowy))

        for row to self.renderheight{
            rowc = self.beginrowy - row - 1
            tilename = self.tileid(self.endrowx,rowc)
            bn3delete(tilename)
            tilename = self.tileid(self.beginrowx,rowc)
            bn3delete(tilename)
            bn3load(tilename,self.*tilename)
            bn3setposition(tilename,self.beginrowx,rowc,0.0)
            //self.setmapview(mat("self.view_x + 2"),self.view_y)
            //print(cat("spawning:",tilename))
        }
    }
    func mapmoveup(){
        print(cat("wiping:",self.endrowy))
        for row to self.renderwidth{
            rowc = 0 - self.viewwidth + row + self.view_x - 1
            tilename = self.tileid(rowc,self.endrowy)
            //print(cat("deleting:",tilename))
            bn3delete(tilename)
            tilename = self.tileid(rowc,self.beginrowy)
            bn3delete(tilename)
            bn3load(tilename,self.*tilename)
            bn3setposition(tilename,rowc,self.beginrowy,0.0)
            //self.setmapview(mat("self.view_x + 2"),self.view_y)
            //print(cat("spawning:",tilename))
        }
    }
    func mapmovedown(){
        print(cat("wiping:",self.beginrowx))
        for row to self.renderwidth{
            rowc = 0 - self.viewwidth + row + self.view_x - 1
            tilename = self.tileid(rowc,self.beginrowy)
            //print(cat("deleting:",tilename))
            bn3delete(tilename)
            tilename = self.tileid(rowc,self.endrowy)
            bn3delete(tilename)
            bn3load(tilename,self.*tilename)
            bn3setposition(tilename,rowc,self.endrowy,0.0)
            //self.setmapview(mat("self.view_x + 2"),self.view_y)
            //print(cat("spawning:",tilename))
        }
    }
    func clearcorners(){
        tx = self.beginrowx - 2
        ty = self.beginrowy - 2
        tilename = self.tileid(tx,ty)
        bn3delete(tilename)
        tx = self.endrowx + 2
        ty = self.beginrowy - 2
        tilename = self.tileid(tx,ty)
        bn3delete(tilename)
        tx = self.beginrowx - 2
        ty = self.endrowy + 2
        tilename = self.tileid(tx,ty)
        bn3delete(tilename)
        tx = self.endrowx + 2
        ty = self.endrowy + 2
        tilename = self.tileid(tx,ty)
        bn3delete(tilename)
    }
    self.cameraoffsetz = 0
    self.view_x = 0
    self.view_y = 0
}