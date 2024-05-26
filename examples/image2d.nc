mynode = nodespawnsquare("mynode")
mytexture = textureload("resources/BlueLogoDiscord.png")

textureset(mynode,mytexture)

coroutine "postloader" {
    size = 24
    defidingspace = 1000
    if size > 17 {
        defidingspace = 850
    }
    if size > 28 {
        defidingspace = 650
    } 
    size_scale_w = size / 20
    size_scale_h = size / 16
    size_distance = size / defidingspace
    print(size_distance)
    text = cat "elham is a boss " size_distance
    nodechr = 0
    addx = size_distance
    setx = -1.0
    sety = 0.97
    
    for xchar in print(split(text)){
        if xchar != "" {
            nodechr ++
            setx = setx + addx
            mynode2d = image2d(cat("textnode_",nodechr),0.1,0.6,0.1)
            nodesetposition(mynode2d,setx,sety,0.0)
            nodesetscale(mynode2d,size_scale_w,size_scale_h,1.0)
            match xchar{
                " " => {
                    xchar = "space"
                }
                 "." => {
                    xchar = "dot"
                }
            }
            mytexture2 = textureload(cat("resources/pngfont/nscriptwhite/",xchar,".png"))
            print(mytexture2)
            textureset(mynode2d,mytexture2)
            print(xchar,"blue")
        }
    }

    


}
coroutine "messedup"{
    break "postloader"
    break self
}
camera.x = 0.0
camera.y = 0.0
camera.z = 2.0
coroutine "gameloop"{
        if key.event == true{
            //keystates are either "up" or "down"
            if key.a == "down"{
                camera.x = camera.x - 0.01
                nodesetposition(mynode2d,-0.5,0.0,0.0)
            }
           if key.d == "down"{
                camera.x = camera.x + 0.01
                nodesetposition(mynode2d,0.5,0.0,0.0)
            }
            if key.w == "down"{
                nodesetposition(mynode2d,0.5,0.3,0.0)
                camera.y = camera.y - 0.01
            }
           if key.s == "down"{
                camera.y = camera.y + 0.01
                nodesetposition(mynode2d,0.5,-0.3,0.0)
            }
            if key.up == "down"{
                camera.z = camera.z - 0.01
            }
           if key.down == "down"{
                camera.z = camera.z + 0.01
            }
        }

    camerasetposition(camera.x,camera.y,camera.z,camera.x,camera.y,math("camera.z - 10")))
}