// load first sqaure
mynode = nodespawnsquare("mynode")
mytexture = textureload("resources/BlueLogoDiscord.png")
textureset(mynode,mytexture)

//load second square
mynode2 = nodespawnsquare("mynode2")
mytexture2 = textureload("resources/img2.png")
textureset(mynode2,mytexture2)

nodesetposition("mynode2",-2.5,0.0,0.0) // adjust the position of mynode2

timer = timerinit()
// spawns a coroutine for 5 seconds it will delete mynode2 and break itself

coroutine "deletenode"{
    if timerdiff(timer) > 4999 {
        nodedelete(mynode2)
        break self
    }
}
coroutine "gameloop"{

    // gameloop
}