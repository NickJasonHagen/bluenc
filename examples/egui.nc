
// so the with the nscript egui system you can create menu objects and layer them by executing functions.
// you can delete the object using objdelete()

// function which will be triggered on button click event
func showcasefunction(){
    mynode.rz = mynode.rz + 1.0
    nodesetrotation(mynode,mynode.rx,mynode.ry,mynode.rz)
    nodesetcolor(mynode,colorvar)
}
//create a new window ui with the identifier mainmenu ( nscript object)
mygui = eguiwindow("mainmenu","Showcase MainMenu!")

// add a button to the window with rotatesquare as its name which will trigger showcasefunction() whenever its clicked
eguibutton(mygui,"rotatesquare","showcasefunction()")

// add a label to the window with a concatination of a static string and a variable as a var (live updated)
eguilabel(mygui,"sqaure Rz:","mynode.rz")

// add a inputfield the typed data be life updated to a given variable(asstring)
eguiinput(mygui,"rx:","mynode.rx")

// demonstration of a checkbox, so labelname , boolvariable(asstring)
mybool = false // passed as 2th argument will be updated on click
eguicheckbox(mygui,"checkbox","mybool")
eguilabel(mygui,"mybool:","mybool")

// and a hyperlink which can open a site using the default browser
eguihyperlink(mygui,"nscript","http://www.nscript.duckdns.org")

//colorpicker label , variable(asstring) updates on OK
colorvar = "" // will contain the color argb code ( updates on interaction)
eguicolorpicker(mygui,"pick a color","colorvar")
eguiinput(mygui,"colorvar:","colorvar")

//todo : slider filebox combo radio

// slider , window, label,variable(asstr) , min , max
eguislider(mygui,"ry:","mynode.ry","0","360")

// openfile dialog box , window, labelname , variable(asstr)
selectedfile = ""
eguilabel(mygui,"selectedfile:","selectedfile")
eguiopenfilebutton(mygui,"Select a file box","selectedfile","print")

// savenfile dialog box , window, labelname , variable(asstr)
eguisavefilebutton(mygui,"Save a file box","selectedfile","print")
// make the window visible
eguiopenwindow(mygui)
//radio buttons
array = ["on","off","standby"]
selectedfromarray = ""
eguiradio(mygui,"radio options","selectedfromarray","array")

//combo
arraycombo = ["on","off","standby"]
selectedfromarraycombo = "off"
eguicombo(mygui,"radio options","selectedfromarraycombo","arraycombo")

// spawn a test node
mynode = nodespawnsquare("mynode")
mytexture = textureload("resources/BlueLogoDiscord.png")
textureset(mynode,mytexture)

coroutine "gameloop"{
    if key.event == true{
        if key.esc == "down"{
            eguiopenwindow(mygui)
        }
    }
}