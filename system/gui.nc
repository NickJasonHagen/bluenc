//gui code blueengine wrapper
class null{

}
class gui{
    // consist of bridging towards the rust blue-engine
    func construct(){
        if self == "gui" return "recurssionoverflow.prevention"
        //print(cat("menu ",self," added mode:",self.menuopenclose))
            
    }

    func new(newself,newtitle){
        self.title = newtitle
        return self
    }

    func addbutton(labelname,variablenscriptcodeasstring){
        prop = self.getid("button")
        self.*prop = labelname
        self.*varprop = variablenscriptcodeasstring

    }

    func addlabel(labelname,variableasstring){
        prop = self.getid("label")
        self.*prop = labelname
        self.*varprop = variableasstring
    }
    func addhyperlink(labelname,variable){
        prop = self.getid("link")
        self.*prop = labelname
        self.*varprop = variable
    }
    func addradio(labelname,variableasstring,givenarray){
        prop = self.getid("radio")
        self.*prop = labelname
        self.*varprop = variableasstring
        funcprop = cat("func_",self.controlids)
        self.*funcprop = givenarray
        funcprop = cat("selected_",self.controlids)
        self.*funcprop = givenarray[?] - 1
    }
    func addcombo(labelname,variableasstring,givenarray){
        prop = self.getid("combo")
        self.*prop = labelname
        self.*varprop = variableasstring
        funcprop = cat("func_",self.controlids)
        self.*funcprop = givenarray
        funcprop = cat("selected_",self.controlids)
        self.*funcprop = 0
    }
    func addinput(labelname,variableasstring){
        prop = self.getid("input")
        self.*prop = labelname
        self.*varprop = variableasstring
    }
    func addcheckbox(labelname,variable){
        prop = self.getid("checkbox")
        self.*prop = labelname
        self.*varprop = variable
    }
    func addpassword(labelname,variable){
        prop = self.getid("password")
        self.*prop = labelname
        self.*varprop = variable
    }
    func addcolor(labelname,variableasstring){
        prop = self.getid("input")
        self.*prop = labelname
        self.*varprop = variableasstring
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
        prop = self.getid("color")
        varprop = cat("var_",self.controlids)
        self.*prop = labelname
        self.*varprop = variable
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
    self.switchtimer = timerinit()
}

class menus{
    func open(name){
        guiactivemenus = name
        if instring(guiactivemenus,name) == 0{
            if instring(guiactivemenus,"|") == 0{
                guiactivemenus = name
            }
            else{
                guiactivemenus = print(cat(guiactivemenus,name,"|"))
            }
            
        }
    }
    func close(){
         guiactivemenus = ""
         return
        self.sgui = split(guiactivemenus,"|")
        guiactivemenus = ""
        i = 0
        last = self.sgui[?] - 1
        for g to last{
            i ++
            if i != last{labellll
                if self.sgui[g] != ""{
                    guiactivemenus = cat(guiactivemenus,self.sgui[g],"|")
                }
            } 
        }
        if last > 1{
            guiactivemenus = print(trimright(guiactivemenus,1))
        }
        
    }
    guiactivemenus = ""
}

func guistart(){
    obj testmenu : gui
    obj testmenu2 : gui
    testmenu.construct()
    testmenu2.construct()
    guiactivemenus = ""//cat(guiactivemenus,"testmenu")   
mypw = "123"
tarray = inobj(blueengine_textures)// 
tarray2 = ["123","456","789"]
print(tarray,"bp")
boxy = false
bb = "123"

Checkboxy = "oi"
link = "http://nscript.duckdns.org"
    testmenu.new("testmenu","mytestGui")
    .addlabel("fps=","fps")
    .addhyperlink("Nscript",link)
    .addinput("playerx","player.x")
    .addinput("playery","player.y")
    .addinput("playerz","player.z")
    .addbutton("testbutton","print(playertoset)")
    .addinput("setplayer","playertoset")
    .addbutton("set player","currentplayer = camera.switch(playertoset)")
    .addslider("speed",1,10,"player.movementspeed")
    .addlabel("colission","colissions.get(player.x,player.y,player.z)")
    .addcolorpicker("PickColor","selectedcolor")
    .addbutton("setcolor","blueengine.setcolor(currentplayer,selectedcolor)")
    .addcombo("aa22","aa",tarray)
    .addcheckbox("Checkboxy","boxy")
    .addlabel("boxstatus:","boxy")
    .addradio("myradio","bb",tarray2)
    .addlabel("->","bb")
    

    playertoset = "dude1"
    testmenu2.new("testmenu2","mytestGui2")
    .addinput("setplayer","playertoset")
    .addinput("NscriptVersion","@nscriptversion")
    .addlabel("cameraz","camera.z").filebox("somefile","filepath","print")
    .addslider("speed",1,10,"player.movementspeed")
    .addpassword("pw",mypw)
    

}

