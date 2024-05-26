mynode = nodespawnsquare("mynode")
mytexture = textureload("resources/BlueLogoDiscord.png")
textureset(mynode,mytexture)

camera.x = 0.0
camera.y = 0.0
camera.z = 2.0
coroutine "gameloop"{
    // built in value key.event be set if any keys are given , sets false afterwords runs each frame
        if key.event == true{
            //keystates are either "up" or "down"
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
        }

    camerasetposition(camera.x,camera.y,camera.z,0.0,0.0,0.0)
}