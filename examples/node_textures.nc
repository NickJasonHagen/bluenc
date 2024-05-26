mynode = nodespawnsquare("mynode")
mytexture = textureload("resources/BlueLogoDiscord.png")
textureset(mynode,mytexture)

print("beginning!")
// at least one loop is required not to have the nscript shutdown. theres a comment so one exists by the identifier of gameloop
coroutine "gameloop"{
    // gameloop
}