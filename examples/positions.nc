// load first sqaure
mynode = nodespawnsquare("mynode")
mytexture = textureload("resources/BlueLogoDiscord.png")
textureset(mynode,mytexture)

//load second square
mynode2 = nodespawnsquare("mynode2")
mytexture2 = textureload("resources/img2.png")
textureset(mynode,mytexture2)

nodesetposition("mynode2",-2.5,0.0,0.0) // adjust the position of mynode2

// at least one loop is required not to have the nscript shutdown. theres a comment so one exists by the identifier of gameloop
coroutine "gameloop"{
    // gameloop
}