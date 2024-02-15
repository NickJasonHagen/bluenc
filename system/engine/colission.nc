
class colissions{
    func construct(){

    }

    func getpoint(pointx,pointy,pointz){
        tx = splitselect(pointx,".",0)
        ty = splitselect(pointy,".",0)
        tz = splitselect(pointz,".",0)
        colissionpoint = replace(cat(tx,"_",ty,"_",tz),"-","m"))
        toret = self.*colissionpoint
        return toret
    }

    func set(spriteobj,pointx,pointy,pointz){
        if spriteobj == "" || pointx == "" return
        if *spriteobj.colissionpoint != ""{
            lastpoint = *spriteobj.colissionpoint
            colissions.*lastpoint = poolremove(colissions.*lastpoint,spriteobj)
        }
        tx = splitselect(pointx,".",0)
        ty = splitselect(pointy,".",0)
        tz = splitselect(pointz,".",0)
        colissionpoint = replace(cat(tx,"_",ty,"_",tz),"-","m")
        colissions.*colissionpoint = pooladd(colissions.*colissionpoint,spriteobj)
        *spriteobj.colissionpoint = colissionpoint
        return colissions.*colissionpoint
    }
}


func hit(){
    if player.lastmovedside == "left"{
        tmpset = -1
    }

    if player.lastmovedside == "right"{
        tmpset = 1
    }

    spritesonspot = colissions.getpoint(*currentplayer.x,*currentplayer.y,*currentplayer.z)
    xpos = *currentplayer.x + tmpset
    pointahead = colissions.getpoint(xpos,*currentplayer.y,*currentplayer.z)
    if pointahead != ".."{
        spritesonspot = pooladd(spritesonspot,pointahead)
    }

    ypos = *currentplayer.y + tmpset
    pointahead = colissions.getpoint(*currentplayer.x,ypos,*currentplayer.z)
    if pointahead != ".."{
        spritesonspot = pooladd(spritesonspot,pointahead)
    }
    pointahead = colissions.getpoint(xpos,ypos,*currentplayer.z)
    if pointahead != ".."{
        spritesonspot = pooladd(spritesonspot,pointahead)
    }
    ypos = *currentplayer.y - tmpset
    pointahead = colissions.getpoint(*currentplayer.x,ypos,*currentplayer.z)
    if pointahead != ".."{
        spritesonspot = pooladd(spritesonspot,pointahead)
    }
    pointahead = colissions.getpoint(xpos,ypos,*currentplayer.z)
    if pointahead != ".."{
        spritesonspot = pooladd(spritesonspot,pointahead)
    }
    for hit in spritesonspot{
        if hit != currentplayer{
            bloodnode = cat("bloodnode_",hit)
            print(cat("hitx=",*hit.x," hitz=",*hit.z))
            blueengine.addsquare(bloodnode).settexture(bloodnode,"blueengine_textures.resources_blood_png").setposition(bloodnode,*hit.x,*hit.y,1.2)
            print(cat("added new bloodloop:",self))
            *bloodnode.ran = random(-0.03,0.03,3)
            *bloodnode.ran2 = random(-0.03,0.03,3)
            coroutine bloodnode {
                t = self
                *t.z = *t.z - 0.1
                *t.x = *t.x - *t.ran
                *t.y = *t.y - *t.ran2
                blueengine.setposition(t,*t.x,*t.y,*t.z)
                if *t.z < -2.0 {
                    blueengine.delete(t)
                    break t
                }
            }
            *hit.setpos(*hit.x,*hit.y,-20.0)                
        }
    }
}