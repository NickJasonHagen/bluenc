//gui code blueengine wrapper
class null{

}
class gui{
    func construct(){
        if self == "gui" return "recurssionoverflow.prevention"
        //print(cat("menu ",self," added mode:",self.menuopenclose))
            
    }

    func new(newself,newtitle){
        self.title = newtitle
        return self
    }

    func addbutton(labelname,variable){
        prop = self.getid("button")
        self.*prop = labelname
        self.*varprop = variable

    }

    func addlabel(labelname,variable){
        prop = self.getid("label")
        self.*prop = labelname
        self.*varprop = variable
    }

    func addinput(labelname,variable){
        prop = self.getid("input")
        self.*prop = labelname
        self.*varprop = variable

    }

    func addslider(labelname,minimum,maximum,variable){
        prop = self.getid("slider")
        varprop = cat("var_",self.controlids)
        self.*prop = labelname
        self.*varprop = variable
        funcprop = cat("min_",self.controlids)
        self.*funcprop = minimum
        funcprop = cat("max_",self.controlids)
        self.*funcprop = maximum
    }

    func addcolorpicker(labelname,variable){
        self.getid("colorpicker")
    }

    func filebox(labelname,variable,function){
        prop = self.getid("file")
        varprop = cat("var_",self.controlids)
        self.*prop = labelname
        self.*varprop = variable
        funcprop = cat("func_",self.controlids)
        self.*funcprop = function
    }

    func show(){
        print(cat("showing:",self),"br")
        print(cat("ObjectDisplay:",object.display(self)),"r")

    }
    func getid(typec){
        self.controlids = math self.controlids + 1
        typeprop = cat("type_",self.controlids)
        self.*typeprop = typec
        self.*typeprop = typec
        varprop = cat("var_",self.controlids)
        ret = cat("control_",self.controlids)

        return ret
    }
    func switch(){
        if timerdiff(self.switchtimer) > 400{
            print(cat("switching:",menuname," ->",self.menuopenclose))
            match self.menuopenclose{
                "open" =>{
                    self.menuopenclose = "closed"
                    guiactivemenus = replace(guiactivemenus,cat(self,"|","")
                }
                "closed" =>{
                    self.menuopenclose = "open"
                    guiactivemenus = cat(guiactivemenus,self,"|")
                }
                _ =>{
                    print(cat("welwelwel some shit went wrong on the gui.switch() val->",self.menuopenclose)
                }
            }
            //print(guiactivemenus,"b")
            self.switchtimer = timerinit()
        }

        
    }
    self.menuopenclose = "open"
    self.controlids = 0
    self.switchtimer = "99999999999999999"
}

func guistart(){
    obj testmenu : gui
    obj testmenu2 : gui
    testmenu.construct()
    testmenu2.construct()
    guiactivemenus = cat(guiactivemenus,"testmenu")   

    testmenu.new("testmenu","mytestGui")
    .addinput("playerx","player.x")
    .addinput("playery","player.y")
    .addinput("playerz","player.z")
    .addbutton("testbutton","print(playertoset)")
    .addinput("setplayer","playertoset")
    .addbutton("set player","currentplayer = camera.switch(playertoset)")
    .addslider("speed",1,10,"player.movementspeed")
    

    playertoset = "dude1"
    testmenu2.new("testmenu2","mytestGui2")
    .addinput("setplayer","playertoset")
    .addinput("NscriptVersion","@nscriptversion")
    .addlabel("cameraz","camera.z").filebox("somefile","filepath","print")
    .addslider("speed",1,10,"player.movementspeed")
    

}
guistart()
