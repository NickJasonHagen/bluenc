// load first sqaure
mynode = nodespawnsquare("mynode")
mytexture = textureload("resources/BlueLogoDiscord.png")

textureset(mynode,mytexture)

//load second square
mynode2 = nodespawnsquare("mynode2")
mytexture2 = textureload("resources/img2.png")
textureset(mynode2,mytexture2)

nodesetposition("mynode2",-2.5,0.0,0.0) // adjust the position of mynode2

rotation = 0.0
// at least one loop is required not to have the nscript shutdown. theres a comment so one exists by the identifier of gameloop
coroutine "gameloop"{
    rotation = rotation + 1.0
    if rotation > 360.0 {
        rotation = 0.0
    }
    nodesetrotation("mynode",0.0,0.0,rotation)
    nodesetrotation("mynode2",rotation,0.0,0.0)
    
}