class controls{
    func ingame(){
           if key.event == "true"{

            if key.esc == "down"{
                menus.close()
            }
            if key.r == "down" && key.t == "down"{
                menus.open("mainmenu")
                //print("mm")
            }

            if instring(guiactivemenus,"clasmenu") == false{
                player.attacking = false
                if key.j == "down"{
                    hit()
                    player.attacking = true
                    player.movedtimer = timerinit()
                    *currentplayer.setanim(cat("anim_attack1",player.lastmovedside))
                }
                if key.k == "down"{
                    hit()
                    //print(spritesonspot,"p")
                    player.attacking = true
                    player.movedtimer = timerinit()
                    *currentplayer.setanim(cat("anim_attack2",player.lastmovedside))
                }
                if key.l == "down"{
                    hit()
                    player.attacking = true
                    player.movedtimer = timerinit()
                    *currentplayer.setanim(cat("anim_attack3",player.lastmovedside))
                }
                if key.left = "down"{
                    camera.x = math camera.x - math("player.movementspeed / 10")
                    cmoved = "true"
                }
                if key.right = "down"{
                    camera.x = math camera.x + math("player.movementspeed / 10")
                    cmoved = "true"
                }
                if key.up = "down"{
                    camera.z = math camera.z - math("player.movementspeed / 10") - math("player.movementspeed / 10")
                    cmoved = "true"
                }
                if key.down = "down"{
                    camera.z = math camera.z + math("player.movementspeed / 10") + math("player.movementspeed / 10")
                    cmoved = "true"
                }
                if key.n = "down"{
                    camera.y = math camera.y - math("player.movementspeed / 10")
                    cmoved = "true"
                }
                if key.m = "down"{
                    camera.y = math camera.y + math("player.movementspeed / 10")
                    cmoved = "true"
                }
                //if player.attacking = false {
                if cmoved == "true" {
                    blueengine.setcamerapos(camera.x,camera.y,camera.z,camera.x,math("camera.y + 30"),math("camera.z - 100"))
                    cmoved = "false"
                    temp = cat camera.x "," camera.y "," camera.z
                }
                if key.a == "down"{
                    *currentplayer.setanim("anim_left")
                    player.lastmovedside = "left"
                    player.x = math player.x - math("player.movementspeed / 10")
                    moved = "true"
                }
                if key.d == "down"{
                    *currentplayer.setanim("anim_right")
                    player.lastmovedside = "right"
                    player.x = math player.x + math("player.movementspeed / 10")
                    moved = "true"
                }
                if key.w == "down"{
                    *currentplayer.setanim("anim_up")
                    player.lastmovedside = "up"
                    player.y = math player.y + math("player.movementspeed / 10")
                    moved = "true"
                }
                if key.s == "down"{
                    *currentplayer.setanim("anim_down")
                    player.lastmovedside = "down"
                    player.y = math player.y - math("player.movementspeed / 10")
                    moved = "true"
                }
                if key.q == "down"{
                    player.z = math player.z - math("player.movementspeed / 10")
                    moved = "true"
                }
                if key.e == "down"{
                    player.z = math player.z + math("player.movementspeed / 10")
                    moved = "true"
                }
                if key.o == "down"{
                    //player.sx = math player.sx - movementspeed
                    //blueengine.setscale("main",player.sx,player.sy,player.sy)
                }
                if key.x == "down" && key.left = "down"{
                    devtools.runcode()
                }
                if key.p == "down"{
                    mapeditor.paint()//bn3setposition("square_1",-4.0,-2.0,0.0)
                }
                if key.o == "down"{
                    bn3setposition("square_1",-4.0,2.0,0.0)

                }
                if key.i == "down"{
                    bn3delete("square_1")

                }
                if moved == "true" {
                    *currentplayer.setpos(player.x ,player.y,0.4)
                    player.movedtimer = timerinit()
                    blueengine.setposition("cursor",player.x ,player.y,player.z)
                    camera.x = player.x
                    camera.y = math player.y - 3
                    map.calculatemappos()
                    blueengine.setcamerapos(camera.x,camera.y,camera.z,player.x,math("player.y + 30"),math("player.z - 100"))
                    moved = "false"
                }
            
            }
        }
    }
}