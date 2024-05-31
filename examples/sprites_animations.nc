
mynode = spriteload("idle","resources/characters/cammy")
spritesetanimation(mynode,"anim_runleft")
nodesetposition(mynode,2.0,0.0,-2.0)
mynode = spriteload("mynode","resources/characters/cammy")
spritesetanimation(mynode,"anim_idleleft")

mynode2 = spriteload("mynode2","resources/characters/dude1")
spritesetanimation(mynode2,"anim_left")
nodesetposition(mynode2,2.0,0.0,0.0)

coroutine "gameloop"{
    if key.event == true {
        if key.a == "down"{
            spritesetanimation(mynode,"anim_idleleft")
            nodesetposition(mynode2,-2.0)
            spritedelete("idle")
            nodedelete("idle")
        }
        if key.d == "down"{
            spritesetanimation(mynode,"anim_idleright")
            nodesetposition(mynode2,2.0)
        }
        if key.e == "down"{
            spritesetanimation(mynode,"anim_attack1right")
        }
        if key.q == "down"{
            spritesetanimation(mynode,"anim_attack1left")
        }
    }
}