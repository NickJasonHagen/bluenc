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

testvar = 3
if testvar = 1 || testvar = 2 || testvar = 3{
    print("testvar test pass","green")
}
else{
    print("failed test","r")
}