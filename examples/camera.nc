mynode = nodespawnsquare("mynode")
mytexture = textureload("resources/BlueLogoDiscord.png")
textureset(mynode,mytexture)

setz = 0.0
setx = 0.0
setz = 1.0
targetx = 1.5
targety = 1.0
targetz = 1.5
coroutine "gameloop"{
    setz = setz + 0.05
    if setz > 1.5 {
        setx = setx + 0.0025
        sety = sety + 0.005
        if setx > 0.4{
            setz = 1.0
            setx = random(0.001,0.005)
            sety = random(0.01,0.05)
            targetx = random(-1.5,1.5)
            targety = random(-1.5,1.5)
            targetz = random(-1.5,1.5)
        }
    }
    //position xyz cameratargetview xyz
    camerasetposition(setx,sety,setz,targetx,targety,targetz)
}