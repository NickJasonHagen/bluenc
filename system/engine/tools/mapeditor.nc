class mapeditor{
    func init(){
        s = self
        *s.allmodels = listdir("./models/")
        *s.selectedmodeli = 0
        obj mapeditor : gui
        mapeditor.new("mapeditor","MapEditor UI")
        .addlabel("oncurrent=","fps")
        .addcombo("bn3 Models:","mapeditor.selectedmodeli","mapeditor.allmodels")
        .addbutton("Paint!","mapeditor.paint()")
        .addbutton("savemap","mapeditor.save()")
    }
    func paint(){
            tilename = map.tileid(map.calcx,map.calcy)
            newmodel = cat(@scriptdir,"models/",mapeditor.selectedmodeli)
            bn3delete(tilename)
            map.*tilename = print(newmodel,"b")
            bn3load(tilename,map.*tilename)
            bn3setposition(tilename,map.calcx,map.calcy,0.0)
    }
    func save(){
        objtofile(map,"./testmap.txt")
    }
}