class blueengine{
    self.title = "BlueNC - Main Menu"
    self.renderwidth = 1920
    self.renderheight = 1080
}

class ui{
    func construct(){
        self.font = pngfontload("resources/pngfont/nscriptwhite/")
        print(cat("font loaded:",self.font))
    }
    func spawngamesenu(){
        beginx = 0.25
        beginy = 0.9
        size = 32
        countery = 0
        lineheight = beginy
        self.items = arraypush(items,listdir("games/"))
        for xitem in self.items {
            prop = cat("games",countery)
            self.*prop = textnode(prop,xitem,beginx,lineheight,size,self.font)
            lineheight -= 0.1
            countery ++ 
        }
        self.max = items[?] - 1
        self.selectednode = self.games1
        textnodecolor(self.selectednode,"0.70357174,0,0,1")
        self.selectedmenu = "games"
        self.selected = 0
    }
    func spawnexamplesmenu(){
        beginx = 0.1
        beginy = 0.9
        size = 32
        countery = 0
        lineheight = beginy
        self.items = listdir("examples/")
        for xitem in self.items {
            
            prop = cat("examples",countery)
            self.*prop = textnode(prop,xitem,beginx,lineheight,size,self.font)
            lineheight -= 0.1
            countery ++ 
        }
        self.max = items[?] - 1
        self.selectednode = self.examples0
        textnodecolor(self.selectednode,"0.70357174,0,0,1")
        self.selectedmenu = "examples"
        self.selected = 0
    }
    func spawnmainmenu(){
        beginx = -0.8
        beginy = 0.9
        size = 42
        countery = 0
        lineheight = beginy
        self.items = ["games","examples","settings","exit"]
        //items = arraypush(items,listdir("examples/"))
        for xitem in self.items {
            prop = cat("main",countery)
            self.*prop = textnode(prop,xitem,beginx,lineheight,size,self.font)
            lineheight -= 0.1
            countery ++ 
        }
        self.max = items[?] - 1
        self.selectednode = self.main1
        textnodecolor(self.selectednode,"0.70357174,0,0,1")
        self.selectedmenu = "main"
        self.selected = 0
    }
    func getselecteditem(){
        prop = cat(self.selectedmenu,self.selected)
        return print(self.*prop)
    }
    func menuselector(side){
        if timerdiff(self.keytimer) > 92{
            match side{
                "up" => {
                    textnodecolor(self.selectednode,"1,1,1,1")
                    self.selected --
                    if self.selected < 0 {
                        self.selected = 0
                    }
                    prop = cat(self.selectedmenu,self.selected)
                    self.selectednode = ui.*prop
                    textnodecolor(self.selectednode,"0.70357174,0,0,1")
                }
                "down" => {
                    textnodecolor(self.selectednode,"1,1,1,1")
                    self.selected ++
                    if self.selected = self.max{
                        self.selected = self.max
                    }
                    prop = cat(self.selectedmenu,self.selected)
                    self.selectednode = ui.*prop
                    textnodecolor(self.selectednode,"0.70357174,0,0,1")
                }
                _ => {
                    print(cat("seems something did a messed up call to ui.menuselector and gave it :",side),"red")
                }
            }
            self.keytimer = timerinit()
        }
    }
    self.selectedmenu = "main"
    self.max = 4
    self.selected = 1
    self.keytimer = timerinit()

}
mynode = nodespawnsquare("mynode")
mytexture = textureload("resources/BlueLogoDiscord_t.png")
textureset(mynode,mytexture)

camera.x = 0.0
camera.y = 0.0
camera.z = 2.0

//ui.construct()
ui.spawnmainmenu()

camerasetposition(camera.x,camera.y,camera.z,camera.x,camera.y,math("camera.z - 10"))

coroutine "game"{
    if key.event == true{
        if key.space == "down" || key.e == "down"{
            if ui.selectedmenu == "examples" {
                diff = timerdiff(ui.keytimer) 
                 if diff > 892 {
                    ui.keytimer = timerinit()
                    run(cat("./bluenc run ./examples/",ui.items[ui.selected]))
                    break "game"
                    break "cameraanimator"
                }
                
            }
            if ui.selectedmenu == "main" {
                
                match ui.items[ui.selected]{
                    "games" =>{
                        print(ui.items[ui.selected])
                        ui.spawngamesmenu()
                        print(ui.items[ui.selected])
                        print(ui.selectedmenu)
                    }
                    "examples" =>{

                        print(ui.items[ui.selected])
                        ui.spawnexamplesmenu()
                        print(ui.items[ui.selected])
                        ui.selectedmenu = "examples"
                        print(ui.selectedmenu)
                        ui.keytimer = timerinit()
                    }
                    "settings" =>{
                        ui.settings()
                    }
                    "exit" =>{
                        //break "cameraanimator"
                        //break "game"
                    }
                    _ => {
                        print(cat("unknown selections:",sel))
                    }
                }
                
            }

        }


        if key.s == "down" || key.down == "down" {
            ui.menuselector("down")
        }

        if key.w == "down" || key.up == "down"{
            ui.menuselector("up")
        }

    }
}
setz = 0.0
setx = 0.0
setz = 1.0
targetx = random(-0.5,0.5)
targety = random(-0.5,0.5)
targetz = random(-0.5,0.5)
coroutine "cameraanimator"{
    setz = setz + 0.001
    if setz > 1.2 {
        setx = setx + 0.0005
        sety = sety + 0.0002
        if setx > 0.3{
            setz = 1.0
            setx = random(0.001,0.005)
            sety = random(0.01,0.05)
            targetx = random(-0.5,0.5)
            targety = random(-0.5,0.5)
            targetz = random(-0.5,0.5)
        }
    }
    //position xyz cameratargetview xyz
    camerasetposition(setx,sety,setz,targetx,targety,targetz)
}