use std::env::VarError;

use blue_engine::StringBufferTrait;
use blue_engine_utilities::animation;

use crate::*;
pub struct Bluenc{

}
impl Bluenc {
    pub fn camerasetposition(x: &f64,y:&f64,z:&f64,targetx:&f64,targety:&f64,targetz:&f64,vmap:&mut Varmap){
        let toset = "".to_owned() + &x.to_string() + "," + &y.to_string() + "," +  &z.to_string() + "," + &targetx.to_string() + "," + &targety.to_string() + "," +  &targetz.to_string();
        //vmap.setvar("blueengine.camera_q".to_owned(), &toset);
        vmap.pushstringarray("blueengine.camera_q", &toset);
        vmap.setvar("camera.x".to_owned(), &x.to_string());
        vmap.setvar("camera.y".to_owned(), &y.to_string());
        vmap.setvar("camera.z".to_owned(), &z.to_string());
        vmap.setvar("camera.targetx".to_owned(), &targetx.to_string());
        vmap.setvar("camera.targety".to_owned(), &targety.to_string());
        vmap.setvar("camera.targetz".to_owned(), &targetz.to_string());
    }
    pub fn image2d(objectname: &str,x: &f64,y:&f64,z:&f64,vmap: &mut Varmap) -> String{
        if objectname == "" {return "".to_string();}
        //let loadq = vmap.getvar("blueengine.image2d_q");
        //vmap.setvar("blueengine.image2d_q".to_string(),&arraypush(&loadq,&objectname));
        vmap.pushstringarray("blueengine.image2d_q", &objectname);
        let prop = "".to_owned() + &objectname  + ".nodetype";
        vmap.setvar(prop.to_string(),"image2d");
        Bluenc::nodesetposition(&objectname, x, y, z, vmap);
        Bluenc::nodesetscale(&objectname,&5.0,&1.0,&5.0,vmap);
        Bluenc::nodesetrotation(&objectname,&0.0,&0.0,&0.0,vmap);

        return objectname.to_string();
    }
    pub fn nodespawnsquare(objectname: &str,x: &f64,y:&f64,z:&f64,vmap: &mut Varmap) -> String{
        if objectname == "" {return "".to_string();}
        //let loadq = vmap.getvar("blueengine.square_q");
        //map.setvar("blueengine.square_q".to_string(),&arraypush(&loadq,&objectname));
        vmap.pushstringarray("blueengine.square_q", &objectname);
        let prop = "".to_owned() + &objectname  + ".nodetype";
        vmap.setvar(prop.to_string(),"square");
        Bluenc::nodesetposition(&objectname, x, y, z, vmap);
        Bluenc::nodesetscale(&objectname,&10.0,&10.0,&10.0,vmap);
        Bluenc::nodesetrotation(&objectname,&0.0,&0.0,&0.0,vmap);
        vmap.setprop(&objectname,"hrx","0");
        vmap.setprop(&objectname,"hry","0");
        vmap.setprop(&objectname,"hrz","0");
        return objectname.to_string();
    }

    pub fn nodedelete(objectname: &str,vmap:&mut Varmap){

        if objectname == "" {return;}
        //let getpooldata = vmap.getvar("blueengine.deletion_q");
        //vmap.setvar("blueengine.deletion_q".to_string(),&arraypush(&getpooldata,&objectname));
        vmap.pushstringarray("blueengine.deletion_q", objectname);

    }

    pub fn nodesetposition(objectname: &str,x: &f64,y:&f64,z:&f64,vmap:&mut Varmap){

        if objectname == "" {return;}
        let collision = vmap.getprop(&objectname, "collision");
        if collision == "true" {
            Bluenc::setcollisionpoint(&objectname,&x.to_string(), &y.to_string(), &z.to_string(), vmap);
        }
        vmap.setprop(&objectname,"x",&x.to_string());
        vmap.setprop(&objectname,"y",&y.to_string());
        vmap.setprop(&objectname,"z",&z.to_string());

        let positionset = "".to_owned() + &objectname + "," + &x.to_string() + "," + &y.to_string() + "," + &z.to_string();
        //let getpooldata = vmap.getvar("blueengine.position_q");
        vmap.pushstringarray("blueengine.position_q",&positionset);
    }

    pub fn nodesetrotation(objectname: &str,x: &f64,y:&f64,z:&f64,vmap:&mut Varmap){

        if objectname == "" {return;}
        let prop = "".to_owned() + &objectname + ".rx";
        vmap.setvar(prop,&x.to_string());
        let prop = "".to_owned() + &objectname + ".ry";
        vmap.setvar(prop,&y.to_string());
        let prop = "".to_owned() + &objectname + ".rz";
        vmap.setvar(prop,&z.to_string());

        //let mut getpooldata = vmap.getvar("blueengine.rotation_q");
        let positionset = "".to_owned() + &objectname + "," + &x.to_string() + ",x";
        vmap.pushstringarray("blueengine.rotation_q", &positionset);
       // getpooldata = arraypush(&getpooldata,&positionset);

        let positionset = "".to_owned() + &objectname + "," + &y.to_string() + ",y";
        vmap.pushstringarray("blueengine.rotation_q", &positionset);
        //getpooldata = arraypush(&getpooldata,&positionset);
        let positionset = "".to_owned() + &objectname + "," + &z.to_string() + ",z";
        vmap.pushstringarray("blueengine.rotation_q", &positionset);
        //getpooldata = arraypush(&getpooldata,&positionset);
        //vmap.setvar("blueengine.rotation_q".to_string(),&getpooldata);

    }
    ///sets the scale of a blueengine(Nscript) node
    pub fn nodesetscale(objectname: &str,x: &f64,y:&f64,z:&f64,vmap:&mut Varmap){

        if objectname == "" {return;}
        let prop = "".to_owned() + &objectname + ".sx";
        vmap.setvar(prop,&x.to_string());
        let prop = "".to_owned() + &objectname + ".sy";
        vmap.setvar(prop,&y.to_string());
        let prop = "".to_owned() + &objectname + ".sz";
        vmap.setvar(prop,&z.to_string());
        let positionset = "".to_owned() + &objectname + "," + &x.to_string() + "," + &y.to_string() + "," + &z.to_string();
        //let getpooldata = vmap.getvar("blueengine.scale_q");
        //vmap.setvar("blueengine.scale_q".to_string(),&arraypush(&getpooldata,&positionset));
        vmap.pushstringarray("blueengine.scale_q", &positionset);

    }

    pub fn nodegetposition(objectname: &str,vmap:&mut Varmap) -> String{
        let prop ="".to_owned()+&objectname+".x";
        let x = vmap.getvar(&prop);
        let prop ="".to_owned()+&objectname+".y";
        let y = vmap.getvar(&prop);
        let prop ="".to_owned()+&objectname+".z";
        let z = vmap.getvar(&prop);
        let returnarraystring = "".to_owned() + &x + NC_ARRAY_DELIM + &y + NC_ARRAY_DELIM + &z;
        return returnarraystring;
    }

    pub fn nodegetrotation(objectname: &str,vmap:&mut Varmap) -> String{

        if objectname == "" {return "error".to_owned();}
        let prop ="".to_owned()+&objectname+".rx";
        let x = vmap.getvar(&prop);
        let prop ="".to_owned()+&objectname+".ry";
        let y = vmap.getvar(&prop);
        let prop ="".to_owned()+&objectname+".rz";
        let z = vmap.getvar(&prop);
        let returnarraystring = "".to_owned() + &x + NC_ARRAY_DELIM + &y + NC_ARRAY_DELIM + &z;
        return returnarraystring;
    }

    pub fn nodegetscale(objectname: &str,vmap:&mut Varmap) -> String{

        if objectname == "" {return "error".to_owned();}
        let prop ="".to_owned()+&objectname+".sx";
        let x = vmap.getvar(&prop);
        let prop ="".to_owned()+&objectname+".sy";
        let y = vmap.getvar(&prop);
        let prop ="".to_owned()+&objectname+".sz";
        let z = vmap.getvar(&prop);
        let returnarraystring = "".to_owned() + &x + NC_ARRAY_DELIM + &y + NC_ARRAY_DELIM + &z;
        return returnarraystring;
    }


    pub fn textureload(texturepath: &str,vmap:&mut Varmap) -> String{
        // adds a texture to the textureloadquee located in the main app loop
        //let mut getq = vmap.getvar("blueengine.textureload_q");
        //getq = arraypush(&getq,&texturepath);
        //vmap.setvar("blueengine.textureload_q".to_owned(),&getq);
        vmap.pushstringarray("blueengine.textureload_q", &texturepath);
        return "blueengine_textures.".to_owned()+&Nstring::stringtoeval(&texturepath);
    }

    pub fn textureset(object: &str,texture:&str,vmap:&mut Varmap){
        //let mut getq = vmap.getvar("blueengine.textureset_q");
        let newqentree = "".to_owned() + &object + "," + &texture + ",";
        //getq = arraypush(&getq,&newqentree);
        //vmap.setvar("blueengine.textureset_q".to_owned(),&getq);
        vmap.pushstringarray("blueengine.textureset_q",&newqentree);

    }
    pub fn spritedelete(object:&str,vmap:&mut Varmap){
        let mut getspritebuffer = vmap.getstringarray("animationhandler.allsprites");
        getspritebuffer.retain(|value| *value != object);
        vmap.setstringarray("animationhandler.allsprites", getspritebuffer);
        Bluenc::nodedelete(object, vmap);
    }
    pub fn spriteload(objectname: &str,dir:&str,vmap:&mut Varmap) -> String{
        let spritedirloadedprop = "spritesdirloaded_".to_owned() + &Nstring::stringtoeval(&dir);
        vmap.setvar("thissprite".to_owned(), &spritedirloadedprop);
        // check for a main dirsprite obj, if its not there create it.
        let isloaded  = vmap.getvar(&spritedirloadedprop);
        if isloaded != "loaded"{
            for xfile in split(&Nfile::dirtolist(dir, true),NC_ARRAY_DELIM){
                if Nstring::instring(&xfile,".png") {
                    Bluenc::textureload(xfile, vmap);
                }
            }
            let initscript = "".to_owned() + &dir + "/config.nc";
            nscript_execute_script(&initscript, "", "", "", "", "", "", "", "", "", vmap);
            //let thissprite = objectname.to_string();//vmap.getvar("thissprite");
            vmap.setvar(spritedirloadedprop.clone(), "loaded");
            let animvec =splittostringvec(&arraysearch(&vmap.inobj(&spritedirloadedprop), "anim_"),"|");
            vmap.setstringarray(&spritedirloadedprop, animvec);
        }
        vmap.setobj(&spritedirloadedprop, &objectname);
            for xanim in vmap.getstringarray(&spritedirloadedprop){
                let getprop = "".to_owned() + &spritedirloadedprop + "."+ &xanim;
                let getarray = vmap.getvar(&getprop);
                let mut fullpathstring = spritedirloadedprop.to_owned();
                let mut counter = 0;
                for xframe in split(&getarray,NC_ARRAY_DELIM){
                    if counter == 0 {
                        fullpathstring = objectname.to_string() + "|" + &xframe;
                    }
                    if counter !=0  && xframe != "" && counter > 0{
                        fullpathstring = fullpathstring.to_owned() + "|blueengine_textures." + &Nstring::stringtoeval(&dir) + "_" + &xframe + "_png" ;
                    }

                    counter +=1;
                }

                //fullpathstring = Nstring::trimright(&fullpathstring,1);// cut off last pipe
                let newprop = "".to_owned() + &objectname + "." + &Nstring::replace(&xanim,"anim_","texturearray_") ;
                vmap.setvar(newprop.to_owned(), &fullpathstring);
                #[cfg(debug_assertions)]
                println!("debug:new {} animarray:{}",&newprop,&fullpathstring);
                vmap.setvar(spritedirloadedprop.clone() + ".animationtimer",&Ntimer::init().to_string());
                //let mut animationhandlerarray = vmap.getvar("animationhandler.allsprites");
                //animationhandlerarray = arraypush(&animationhandlerarray, &thissprite);
            }
        //clone maindirobj to new objectname

            //vmap.setvar("animationhandler.allsprites".to_owned(), &animationhandlerarray);



            vmap.pushstringarray("animationhandler.allsprites", &objectname);
        vmap.setvar(dir.to_owned()+".name", &dir);

        // add a square to the animated node
        //let sqrq = vmap.getvar("blueengine.square_q");
        //vmap.setvar("blueengine.square_q".to_owned(),&arraypush(&sqrq,&objectname));
        //vmap.pushstringarray("blueengine.square_q", &objectname);
        Bluenc::nodespawnsquare(&objectname, &0.0, &0.0, &0.0, vmap);

        return objectname.to_string(); // required per sprite set first thissprite in nscript
    }
    pub fn spritesetanimation(object:&str,anim:&str,vmap:&mut Varmap){
        let curanimprop = object.to_string()+".currentanim";
        if vmap.getvar(&curanimprop) != anim {
            vmap.setvar(curanimprop.to_string(), &anim);

            vmap.setvar(object.to_string()+".currentframe", "0");

            vmap.setvar(object.to_string()+".animationswitchtimer", &Ntimer::init().to_string());
            let key = "blueengine.anim_q";
            //let getanimq = vmap.getvar(key);
            let animarraykey = "".to_owned() + &object + ".texturearray_" + &Nstring::replace(&anim,"anim_","");
            let animarray = vmap.getvar(&animarraykey);

            vmap.pushstringarray("blueengine.anim_q", &animarray);
            #[cfg(debug_assertions)]
            println!("debg:animarray:{}",&animarray);
        }
    }
    pub fn spritegetanimations(object:&str,vmap:&mut Varmap) -> String{
        let res = arraysearch(&Nstring::replace(&vmap.inobj(&object),"|",NC_ARRAY_DELIM),"anim_");
        return res;
    }
    pub fn eguiclose(name:&str,vmap:&mut Varmap){
        let mut guiactivemenus = Nstring::replace(&vmap.getvar("guiactivemenus"),"|",NC_ARRAY_DELIM);
        guiactivemenus = poolremove(&guiactivemenus, &name);
        vmap.setvar("guiactivemenus".to_string(), &Nstring::replace(&guiactivemenus, NC_ARRAY_DELIM, "|"));
    }
    pub fn eguiopen(name:&str,vmap:&mut Varmap){
        let mut guiactivemenus = Nstring::replace(&vmap.getvar("guiactivemenus"),"|",NC_ARRAY_DELIM);
        guiactivemenus = arraypush(&guiactivemenus, &name);
        vmap.setvar("guiactivemenus".to_string(), &Nstring::replace(&guiactivemenus, NC_ARRAY_DELIM, "|"));
    }

    pub fn eguiwindow(name:&str,title:&str,vmap:&mut Varmap)->String{
        vmap.setvar(name.to_string()+".title", &title);

        vmap.setvar(name.to_string()+".controlids", "0");
        let getid = Bluenc::eguigetid(&name, "button", vmap);
        vmap.setvar(name.to_string()+".control_"+ &getid, "[x]");
        let mut fntorun = "eguiclosewindow(\"".to_owned() + &name + "\")";
        fntorun = "^".to_owned() + &string_to_hex(&fntorun);// format it to a nscript static string!
        println!("closing:{}",&fntorun);
        vmap.setvar(name.to_string()+".var_"+ &getid,&fntorun );
        return name.to_string();
    }
    pub fn eguibutton(windowname:&str,buttonname:&str,func:&str,vmap:&mut Varmap ){
        let getid = Bluenc::eguigetid(&windowname, "button", vmap);
        vmap.setvar(windowname.to_string()+".control_"+ &getid, &buttonname);
        vmap.setvar(windowname.to_string()+".var_"+ &getid,&func );
    }
    pub fn eguisavefilebutton(windowname:&str,buttonname:&str,varstr:&str,func:&str,vmap:&mut Varmap ){
        let getid = Bluenc::eguigetid(&windowname, "filesave", vmap);
        vmap.setvar(windowname.to_string()+".control_"+ &getid, &buttonname);
        vmap.setvar(windowname.to_string()+".var_"+ &getid,&varstr );
        vmap.setvar(windowname.to_string()+".func_"+ &getid,&func );
    }
    pub fn eguiopenfilebutton(windowname:&str,buttonname:&str,varstr:&str,func:&str,vmap:&mut Varmap ){
        let getid = Bluenc::eguigetid(&windowname, "fileopen", vmap);
        vmap.setvar(windowname.to_string()+".control_"+ &getid, &buttonname);
        vmap.setvar(windowname.to_string()+".var_"+ &getid,&varstr );
        vmap.setvar(windowname.to_string()+".func_"+ &getid,&func );
    }

    pub fn eguilabel(windowname:&str,buttonname:&str,varstr:&str,vmap:&mut Varmap ){
        let getid = Bluenc::eguigetid(&windowname, "label", vmap);
        vmap.setvar(windowname.to_string()+".control_"+ &getid, &buttonname);
        vmap.setvar(windowname.to_string()+".var_"+ &getid,&varstr );
    }
    pub fn eguiinput(windowname:&str,buttonname:&str,varstr:&str,vmap:&mut Varmap ){
        let getid = Bluenc::eguigetid(&windowname, "input", vmap);
        vmap.setvar(windowname.to_string()+".control_"+ &getid, &buttonname);
        vmap.setvar(windowname.to_string()+".var_"+ &getid,&varstr );
    }
    pub fn eguipassword(windowname:&str,buttonname:&str,varstr:&str,vmap:&mut Varmap ){
        let getid = Bluenc::eguigetid(&windowname, "password", vmap);
        vmap.setvar(windowname.to_string()+".control_"+ &getid, &buttonname);
        vmap.setvar(windowname.to_string()+".var_"+ &getid,&varstr );
    }
    pub fn eguihyperlink(windowname:&str,buttonname:&str,varstr:&str,vmap:&mut Varmap ){
        let getid = Bluenc::eguigetid(&windowname, "link", vmap);
        vmap.setvar(windowname.to_string()+".control_"+ &getid, &buttonname);
        vmap.setvar(windowname.to_string()+".var_"+ &getid,&varstr );
    }
    pub fn eguicheckbox(windowname:&str,buttonname:&str,varstr:&str,vmap:&mut Varmap ){
        let getid = Bluenc::eguigetid(&windowname, "checkbox", vmap);
        vmap.setvar(windowname.to_string()+".control_"+ &getid, &buttonname);
        vmap.setvar(windowname.to_string()+".var_"+ &getid,&varstr );
    }
    pub fn eguicolorpicker(windowname:&str,buttonname:&str,varstr:&str,vmap:&mut Varmap ){
        let getid = Bluenc::eguigetid(&windowname, "color", vmap);
        vmap.setvar(windowname.to_string()+".control_"+ &getid, &buttonname);
        vmap.setvar(windowname.to_string()+".var_"+ &getid,&varstr );
    }
    pub fn eguislider(windowname:&str,labelname:&str,varstr:&str,min:&str,max:&str,vmap:&mut Varmap){
        let getid = Bluenc::eguigetid(&windowname, "slider", vmap);
        vmap.setvar(windowname.to_string()+".control_"+ &getid, &labelname);
        vmap.setvar(windowname.to_string()+".var_"+ &getid,&varstr );
        vmap.setvar(windowname.to_string()+".min_"+ &getid,&min );
        vmap.setvar(windowname.to_string()+".max_"+ &getid,&max );
    }
    pub fn eguicombo(windowname:&str,labelname:&str,varstr:&str,arraystr:&str,vmap:&mut Varmap){
        let getid = Bluenc::eguigetid(&windowname, "combo", vmap);
        vmap.setvar(windowname.to_string()+".control_"+ &getid, &labelname);
        vmap.setvar(windowname.to_string()+".var_"+ &getid,&varstr );
        vmap.setvar(windowname.to_string()+".func_"+ &getid,&arraystr );
        vmap.setvar(windowname.to_string()+".selected_"+ &getid,"0" );
    }
    pub fn eguiradio(windowname:&str,labelname:&str,varstr:&str,arraystr:&str,vmap:&mut Varmap){
        let getid = Bluenc::eguigetid(&windowname, "radio", vmap);
        vmap.setvar(windowname.to_string()+".control_"+ &getid, &labelname);
        vmap.setvar(windowname.to_string()+".var_"+ &getid,&varstr );
        vmap.setvar(windowname.to_string()+".func_"+ &getid,&arraystr );
        let startat  = split(&vmap.getvar(&arraystr),NC_ARRAY_DELIM).len()-1;
        println!("radio start@{}",&startat);
        vmap.setvar(windowname.to_string()+".selected_"+ &getid,&startat.to_string() );
    }

    fn eguigetid(name:&str,uitype: &str,vmap: &mut Varmap)->String{
        let toget = name.to_string()+".controlids";
        let id = vmap.getvar(&toget);
        let newid = nscript_i32(&id) + 1;
        vmap.setvar(toget,&newid.to_string());
        vmap.setvar(name.to_string()+".type_"+ &newid.to_string(),&uitype);
        return newid.to_string();
    }
    /// a textnode for bluenc, renders a textstring of seperated png files onto the top layer for
    /// overlay
    pub fn textnode(identifiername:&str,defaulttext: &str,x: &str,y: &str,size:&str,font:&str,vmap: &mut Varmap) -> String{
        //let getq = vmap.getvar("blueengine.textnode_q");
        vmap.setvar(identifiername.to_string() + ".text", &defaulttext.to_lowercase());
        vmap.setvar(identifiername.to_string() + ".x", &x);
        vmap.setvar(identifiername.to_string() + ".y", &y);
        vmap.setvar(identifiername.to_string() + ".size", &size);
        vmap.setvar(identifiername.to_string() + ".font", &font);
        vmap.pushstringarray("blueengine.textnode_q",&identifiername);
        identifiername.to_string()
    }
    pub fn textnodeupdate(identifiername:&str,defaulttext: &str,x: &str,y: &str,size:&str,font:&str,vmap: &mut Varmap) -> String{
        //let getq = vmap.getvar("blueengine.textnodeupdate_q");
        vmap.setvar(identifiername.to_string() + ".text", &defaulttext.to_lowercase());
        vmap.setvar(identifiername.to_string() + ".newx", &x);
        vmap.setvar(identifiername.to_string() + ".newy", &y);
        vmap.setvar(identifiername.to_string() + ".newsize", &size);
        vmap.setvar(identifiername.to_string() + ".newfont", &font);
        vmap.pushstringarray("blueengine.textnodeupdate_q",&identifiername);
        identifiername.to_string()
    }
    /// loads a pngfont for bluenc, used for textnode
    pub fn loadpngfont(dirname:&str,vmap:&mut Varmap) ->String{

        let fontobjectname = Nstring::stringtoeval(&dirname.to_string());
        for xchar in split(&Nfile::dirtolist(&dirname,false),NC_ARRAY_DELIM){
            let prop = fontobjectname.to_owned() + "." + &xchar;
            let toload = dirname.to_string() + &xchar;
            let loadtexture = Bluenc::textureload(&toload, vmap);
            vmap.setvar(prop.to_string(),&loadtexture);

        }
        fontobjectname

    }
    /// sets on object to collisionpoint as a array of unique object identiefiers
    pub fn setcollisionpoint(objectname:&str,x:&str,y:&str,z:&str,vmap:&mut Varmap){
        let id = Bluenc::collisionpointtag(&x,&y,&z);
        let getoldpointid = vmap.getprop(&objectname,"collisionpoint");
        if getoldpointid != "" {
            vmap.retainstringarray(&getoldpointid, &objectname);
        }
        if id != getoldpointid {
            //println!("new object {} on point {}", &objectname,&id);
            vmap.pushuniquestringarray(&id,&objectname);
        }

        // set the points id as a prop to the object whe it moves it can be reset faster
        vmap.setprop(&objectname,"collisionpoint",&id);
    }

    /// gets a nscript array back with all objects
    pub fn getcollisionpoint(x:&str,y:&str,z:&str,vmap:&mut Varmap) -> String{
        let id = Bluenc::collisionpointtag(&x,&y,&z);
        vmap.getstringarray(&id).join(NC_ARRAY_DELIM)
    }

    /// returns a normal array for faster internal use ( projectilehandler)
    pub fn getcollisionpointraw(x:&str,y:&str,z:&str,vmap:&mut Varmap) -> Vec<String>{
        let id = Bluenc::collisionpointtag(&x,&y,&z);
        vmap.getstringarray(&id)
    }

    /// reads out the objec.collisionpoint property and removes itself from the group
    pub fn removecollisionpoint(objectname:&str,vmap:&mut Varmap){
        let id = vmap.getprop(&objectname,"collisionpoint");
        vmap.retainstringarray(&id,&objectname);
    }

    /// interally used to get the storagetag
    fn collisionpointtag(x:&str,y:&str,z:&str) -> String{
        let collisionpointprop = "collisionpoint_".to_owned() + &split(&x,".")[0] + "_"+ &split(y,".")[0] + "_" + &split(&z,".")[0];
        return collisionpointprop
    }
    /// this returns a array with xyz vector to be used for projectiles and rays
    pub fn raycaster(rayid:&str,texture:&str,start: (f32, f32, f32), target: (f32, f32, f32), step: f32,vmap:&mut Varmap)->String {
        let (x0, y0, z0) = start;
        let (x1, y1, z1) = target;

        let dx = x1 - x0;
        let dy = y1 - y0;
        let dz = z1 - z0;

        let distance = ((dx * dx) + (dy * dy) + (dz * dz)).sqrt();
        let steps = (distance / step).ceil() as usize;

        let mut points = Vec::with_capacity(steps + 1);

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let x = x0 + t * dx;
            let y = y0 + t * dy;
            let z = z0 + t * dz;
            points.push((x, y, z));
        }

        vmap.f32vectormap.insert(rayid.to_string(),points);
        let queeentree = "nodeprojectile,".to_string() + &rayid + "," + &texture + "," + &start.0.to_string() + "," + &start.1.to_string() + "," + &start.2.to_string()+",";
        vmap.pushstringarray("blueengine.event_q", &queeentree);
        return rayid.to_string();
    }
    /// set a color to the textnodes, will render to all characters
    pub fn textnodecolor(identifiername:&str,colorcode:&str,vmap:&mut Varmap){
        //let getq = vmap.getvar("blueengine.textcolor_q");
        vmap.setvar(identifiername.to_string() +".color",&colorcode);
        vmap.pushstringarray("blueengine.textcolor_q",&identifiername);
    }
    pub fn nodesetcolor(identifiername:&str,colorcode:&str,vmap:&mut Varmap){
        //let getq = vmap.getvar("blueengine.textcolor_q");
        vmap.setvar(identifiername.to_string() +".color",&colorcode);
        let topush = identifiername.to_string() + "," + &colorcode;
        vmap.pushstringarray("blueengine.color_q",&topush);
    }
     /// this function will return the filename of the pngfont, a questionmark be returned for
    /// unknown characters to not crash the engine!
    pub fn pngcharname(char:&str) ->String{
        let chrtexturename = match char{
            "1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9"|"0"| "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" |"r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" =>{
                return char.to_string();
            }
            "." => {"dot".to_string()}
            " " => {"space".to_string()}
            "," => {"comma".to_string()}
            "(" => {"parenthesisopen".to_string()}
            ")" => {"parenthesisclose".to_string()}
            "{" => {"curlybrackeyopen".to_string()}
            "}" => {"curlybrackeyclose".to_string()}
            "]" => {"squarebrackeyclose".to_string()}
            "[" => {"squarebrackeyopen".to_string()}
            ">" => {"anglebrackeyclose".to_string()}
            "<" => {"anglebrackeyopen".to_string()}
            "*" => {"asterix".to_string()}
            "&" => {"ampersand".to_string()}
            ":" => {"colon".to_string()}
            "!" => {"exclamationmark".to_string()}
            "$" => {"dollar".to_string()}
            "?" => {"questionmark".to_string()}
            "@" => {"at".to_string()}
            ";" => {"semicolon".to_string()}
            "|" => {"pipe".to_string()}
            "-" => {"dash".to_string()}
            "+" => {"plus".to_string()}
            "#" => {"hashtag".to_string()}
            "%" => {"percentage".to_string()}
            "^" => {"caret".to_string()}
            "_" => {"underscore".to_string()}
            "/" => {"slash".to_string()}
            "\\" => {"backslash".to_string()}


            _ => "questionmark".to_string()
        };
        return chrtexturename;
    }
}
