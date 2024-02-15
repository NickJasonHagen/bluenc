
class camera{
    func switch(towardschar){
        debugmode(1)
        print(cat("moving to char:",towardschar),"bp")
        player.x = *towardschar.x
        player.y = *towardschar.y
        player.z = *towardschar.z
        print(cat("pos:",*towardschar.x,"-",*towardschar.y,"-",*towardschar.z),"p")
        player.movedtimer = timerinit()
        blueengine.setposition("cursor",player.x ,player.y,player.z)
        camera.x = player.x
        camera.y = player.y - 3
        blueengine.setcamerapos(camera.x,camera.y,camera.z,player.x,math("player.y + 30"),math("player.z - 100"))
        //print(cat("PlayerDebugPos:x",player.x,",y",player.y,",z",player.z))
        moved = "false"
        debugmode(0)
        return towardschar    
    }
    camera "camera"
    camera.x = 0.0
    camera.y = -10.0
    camera.z = 10.0
    camera.targety = 2
    camera.targetx = 0
    camera.targetz = -10
    //print(object.display(self))
}