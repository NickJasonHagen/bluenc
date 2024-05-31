class blueengine{
    self.renderwidth = 2
    self.renderheight = 2
    self.vsync = false
    self.powermode = true
    self.title = "BlueEngine & Nscript example textnode()"
}

mynode = nodespawnsquare("mynode")
mytexture = textureload("resources/BlueLogoDiscord.png")
textureset(mynode,mytexture)
pngfontobj = pngfontload("resources/pngfont/nscriptwhite/")
selectednode = "text1" 
//2d image
beganx = -0.8
img = image2d("img")
textureset(img,mytexture)
nodesetposition(img,-0.75,0.75,-0.1)
nodesetscale(img,4.0,4.0,4.0)

// text nodes
textnode("fps","0",beganx,0.9,40.0,pngfontobj) 
textnode("cameratext","0",beganx,0.8,40.0,pngfontobj) 
textnode("text1","play game",beganx,0.1,40.0,pngfontobj) 
textnode("text2","settings",beganx,-0.0,40.0,pngfontobj) 
textnode("text3","exit",beganx,-0.1,40.0,pngfontobj)
textnode("text4","!@#$%^&*() €",beganx,-0.2,40.0,pngfontobj)


camera.x = 0.0
camera.y = 0.0
camera.z = 2.0
mkeytimer = timerinit()
fpsupdatetimer = timerinit()
camupdatetimer = timerinit()
coroutine "gameloop"{
diff = timerdiff(fpsupdatetimer)
    if diff > 999{
        textnodeupdate("fps",cat("fps:",blueengine.fps),beganx,0.9,40.0,pngfontobj) 
        fpsupdatetimer = timerinit()
    }

    if key.event == true{
        
        if timerdiff(mkeytimer) > 700{
            if key.y == "down"{
                textnodedelete("text4")
            }
            if key.r == "down"{
                textnodecolor(selectednode,"1,1,1,1")
                selectednode = "text1"
                textnodecolor(selectednode,"0.70357174,0,0,1")
                mkeytimer = timerinit()
                print("aaaaa")
            }
            if key.e == "down"{
                textnodecolor(selectednode,"1,1,1,1")
                selectednode = "text2"
                textnodecolor(selectednode,"0.70357174,0,0,1")
                mkeytimer = timerinit()
            }
            if key.q == "down"{
                textnodecolor(selectednode,"1,1,1,1")
                selectednode = "text3"
                textnodecolor(selectednode,"0.70357174,0,0,1")
                mkeytimer = timerinit()
            }
        }

        if key.a == "down"{
            camera.x = camera.x - 0.01
            
        }
        if key.d == "down"{
            camera.x = camera.x + 0.01
        }
        if key.w == "down"{
            camera.y = camera.y - 0.01
        }
        if key.s == "down"{
            camera.y = camera.y + 0.01
        }
        if key.up == "down"{
            camera.z = camera.z - 0.01
        }
        if key.down == "down"{
            camera.z = camera.z + 0.01
        }
        
        if timerdiff(camupdatetimer) > 60 {
            textnodeupdate("cameratext",cat("camx:",fromleft(camera.x,5)," camy:",fromleft(camera.y,5)),beganx,0.8,40.0,pngfontobj) 
            camupdatetimer = timerinit()
        }
       camerasetposition(camera.x,camera.y,camera.z,camera.x,camera.y,math("camera.z - 10"))
    }
}