//gui code blueengine wrapper
class null{

}

class gui{
    // consist of bridging towards the rust blue-engine
    func construct(){
        if self == "gui"{
            return
        }

    }
    func new(newself,newtitle){
        self.title = newtitle
        self.controlids = 0
        prop = self.getid("button")
        self.*prop = "[x]"
        tmp = cat("menus.closebyname(",self,")")
        self.*varprop = tmp  
        return self
    }
    func closebutton(){
        prop = self.getid("button")
        self.*prop = "[x]"
        tmp = cat("menus.closebyname(",self,")")
        self.*varprop = tmp

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
        //print(cat("newmenu id:",self.controlids," self:",self),"y")
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
    func construct(){
        guiactivemenus = ""
        self.switchtimer = timerinit()
    }
    func open(name){
        //guiactivemenus = name
        if instring(guiactivemenus,name) == 0{
            if instring(guiactivemenus,"|") == 0{
                guiactivemenus = name
            }
            else{
                guiactivemenus = cat(guiactivemenus,name,"|")
            }
            self.switchtimer = timerinit()
        }
        else{
            if timerdiff(self.switchtimer) > 300 {
                menus.closebyname(name)
            }
        }
    }
    func close(){
        if timerdiff(self.switchtimer) < 300 {
            return
        }
        else{
            if guiactivemenus == ""{
                menus.open("mainmenu")
                return
            } 
            self.sgui = split(guiactivemenus,"|")
            guiactivemenus = ""
            i = 0
            last = self.sgui[?] - 1
            for g in self.sgui{
                i ++
                if i != last{
                    if g != ""{
                        guiactivemenus = cat(guiactivemenus,g,"|")
                    }
                } 
            }

            if instring(guiactivemenus,"|") == 0{
                guiactivemenus = ""
            }
            self.switchtimer = timerinit()
        }
        
    }
    func closebyname(name){
        if instring(guiactivemenus,cat(name,"|")) == 1{
            guiactivemenus = replace(guiactivemenus,cat(name,"|"),"")
        }
        else{
            if instring(guiactivemenus,cat(name,"|")) == 1{
                guiactivemenus = replace(guiactivemenus,name,"")
            }           
        }
    }

}

func guistart(){
    //testmenu1 = "testmenu1"
    //testmenu2 = "testmenu2"
    //mainmenu = "mainmenu"
    obj testmenu1 : gui
    obj testmenu2 : gui
    //testmenu1.construct()
    //testmenu2.construct()
    guiactivemenus = ""//cat(guiactivemenus,"testmenu")   
    mypw = "123"
    tarray = inobj(blueengine_textures)// 
    tarray2 = ["123","456","789"]
    print(tarray,"bp")
    boxy = true
    bb = "123"
    playertoset = "dude1"
    Checkboxy = "oi"
    link = "http://nscript.duckdns.org"

    testmenu1.new("testmenu1","mytestGui")
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
    


    testmenu2.new("testmenu2","mytestGui2")
    .addinput("setplayer","playertoset")
    .addinput("NscriptVersion","@nscriptversion")
    .addlabel("cameraz","camera.z").filebox("somefile","filepath","print")
    .addslider("speed",1,10,"player.movementspeed")
    .addpassword("pw",mypw)

    //menus.construct()
    viewingclas = "char_22"
    obj mainmenu : gui
    mainmenu.new("mainmenu","Main Menu BlueNC")
    .addlabel("fps","fps")
    .addbutton("Testmenu1","menus.open(testmenu1)")
    .addbutton("Testmenu2","menus.open(testmenu2)")
    .addbutton("clasmenu","menus.open(clasmenu)")
    
    obj clasmenu : gui
    clasmenu.new("clasmenu","Clas eviewer")
    .addinput("Classdata:","viewingclas")
    .addbutton("load","reloadclasviewer()")
}

func reloadclasviewer(){
    catchvar = viewingclas
    print(cat("reloading!",viewingclas," in:",inobj(viewingclas)),"g")
    rtmptimer = timerinit()
    menus.closebyname(catchvar)
    coroutine "reloaderviewer"{
        if timerdiff(rtmptimer) > 50{
            delobj(clasmenu)
            obj clasmenu : gui
            clasmenu.new("clasmenu","Class eviewer")
            .addinput("Clasdata","viewingclas")
            .addbutton("load","reloadclasviewer()")
            debugmode(1)
            for oi in inobj(catchvar){
                tosetvar = cat catchvar "." oi
                clasmenu.addinput(oi,tosetvar)
                print(cat("adding input:",oi),"y")
            }
            debugmode(0)
            //menus.open(clasmenu)     
            break "reloaderviewer"
        }
    }

}