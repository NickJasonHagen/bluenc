class devtools{
    func runcode(){
        if timerdiff(keytimer) < 99 {
            return
        }
        else {
            code = terminalinput("code?","exit")
            keytimer = timerinit()
            return code(code)   
        }

    }
    keytimer = timerinit()
}

class object{
    func display(oname){
        
        displaystring = cat("Object ",oname,@lf)
        for oxx in inobj(oname){
            entr = cat(oname,"[",oxx,"=",*oname.*oxx,"]",@lf)
            displaystring = cat displaystring entr
        }
        return displaystring
    }
    
}
