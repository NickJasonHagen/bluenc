use std::env::VarError;

use blue_engine::StringBufferTrait;
use blue_engine_utilities::animation;

use crate::*;
pub struct Bluenc{

}
impl Bluenc {
    pub fn camerasetposition(x: &f64,y:&f64,z:&f64,targetx:&f64,targety:&f64,targetz:&f64,vmap:&mut Varmap){
        let toset = "".to_owned() + &x.to_string() + "," + &y.to_string() + "," +  &z.to_string() + "," + &targetx.to_string() + "," + &targety.to_string() + "," +  &targetz.to_string();
        vmap.setvar("blueengine.camera_q".to_owned(), &toset);
        vmap.setvar("camera.x".to_owned(), &x.to_string());
        vmap.setvar("camera.y".to_owned(), &y.to_string());
        vmap.setvar("camera.z".to_owned(), &z.to_string());
        vmap.setvar("camera.targetx".to_owned(), &targetx.to_string());
        vmap.setvar("camera.targety".to_owned(), &targety.to_string());
        vmap.setvar("camera.targetz".to_owned(), &targetz.to_string());
    }
    pub fn image2d(objectname: &str,x: &f64,y:&f64,z:&f64,vmap: &mut Varmap) -> String{
        if objectname == "" {return "".to_string();}
        let loadq = vmap.getvar("blueengine.pngfont_q");
        vmap.setvar("blueengine.pngfont_q".to_string(),&pooladd(&loadq,&objectname));
        let prop = "".to_owned() + &objectname  + ".nodetype";
        vmap.setvar(prop.to_string(),"image2d");
        Bluenc::nodesetposition(&objectname, x, y, z, vmap);
        Bluenc::nodesetscale(&objectname,&5.0,&1.0,&5.0,vmap);
        Bluenc::nodesetrotation(&objectname,&0.0,&0.0,&0.0,vmap);

        return objectname.to_string();
    }
    pub fn nodespawnsquare(objectname: &str,x: &f64,y:&f64,z:&f64,vmap: &mut Varmap) -> String{
        if objectname == "" {return "".to_string();}
        let loadq = vmap.getvar("blueengine.square_q");
        vmap.setvar("blueengine.square_q".to_string(),&pooladd(&loadq,&objectname));
        let prop = "".to_owned() + &objectname  + ".nodetype";
        vmap.setvar(prop.to_string(),"square");
        Bluenc::nodesetposition(&objectname, x, y, z, vmap);
        Bluenc::nodesetscale(&objectname,&10.0,&10.0,&10.0,vmap);
        Bluenc::nodesetrotation(&objectname,&0.0,&0.0,&0.0,vmap);

        return objectname.to_string();
    }

    pub fn nodedelete(objectname: &str,vmap:&mut Varmap){

        if objectname == "" {return;}
        let getpooldata = vmap.getvar("blueengine.deletion_q");
        vmap.setvar("blueengine.deletion_q".to_string(),&pooladd(&getpooldata,&objectname));
    }

    pub fn nodesetposition(objectname: &str,x: &f64,y:&f64,z:&f64,vmap:&mut Varmap){

        if objectname == "" {return;}
        let prop = "".to_owned() + &objectname + ".x";
        vmap.setvar(prop,&x.to_string());
        let prop = "".to_owned() + &objectname + ".y";
        vmap.setvar(prop,&y.to_string());
        let prop = "".to_owned() + &objectname + ".z";
        vmap.setvar(prop,&z.to_string());
        let positionset = "".to_owned() + &objectname + "," + &x.to_string() + "," + &y.to_string() + "," + &z.to_string();
        let getpooldata = vmap.getvar("blueengine.position_q");
        vmap.setvar("blueengine.position_q".to_string(),&pooladd(&getpooldata,&positionset));
    }

    pub fn nodesetrotation(objectname: &str,x: &f64,y:&f64,z:&f64,vmap:&mut Varmap){

        if objectname == "" {return;}
        let prop = "".to_owned() + &objectname + ".rx";
        vmap.setvar(prop,&x.to_string());
        let prop = "".to_owned() + &objectname + ".ry";
        vmap.setvar(prop,&y.to_string());
        let prop = "".to_owned() + &objectname + ".rz";
        vmap.setvar(prop,&z.to_string());

        let mut getpooldata = vmap.getvar("blueengine.rotation_q");
        let positionset = "".to_owned() + &objectname + "," + &x.to_string() + ",x";
        getpooldata = pooladd(&getpooldata,&positionset);
        let positionset = "".to_owned() + &objectname + "," + &y.to_string() + ",y";
        getpooldata = pooladd(&getpooldata,&positionset);
        let positionset = "".to_owned() + &objectname + "," + &z.to_string() + ",z";
        getpooldata = pooladd(&getpooldata,&positionset);
        vmap.setvar("blueengine.rotation_q".to_string(),&getpooldata);

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
        let getpooldata = vmap.getvar("blueengine.scale_q");
        vmap.setvar("blueengine.scale_q".to_string(),&pooladd(&getpooldata,&positionset));

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
        let mut getq = vmap.getvar("blueengine.textureload_q");
        getq = pooladd(&getq,&texturepath);
        vmap.setvar("blueengine.textureload_q".to_owned(),&getq);
        return "blueengine_textures.".to_owned()+&Nstring::stringtoeval(&texturepath);
    }

    pub fn textureset(object: &str,texture:&str,vmap:&mut Varmap){
        let mut getq = vmap.getvar("blueengine.textureset_q");
        let newqentree = "".to_owned() + &object + "," + &texture + ",";
        getq = pooladd(&getq,&newqentree);
        vmap.setvar("blueengine.textureset_q".to_owned(),&getq);

    }
    pub fn spriteload(objectname: &str,dir:&str,vmap:&mut Varmap) -> String{
        vmap.setvar("thissprite".to_owned(), &objectname);
        for xfile in split(&Nfile::dirtolist(dir, true),NC_ARRAY_DELIM){
            if Nstring::instring(&xfile,".png") {
                Bluenc::textureload(xfile, vmap);
            }
            let initscript = "".to_owned() + &dir + "/config.nc";
            nscript_execute_script(&initscript, "", "", "", "", "", "", "", "", "", vmap);

        }
        let thissprite = vmap.getvar("thissprite");
        for xanim in split(&arraysearch(&vmap.inobj(&thissprite), "anim_"),"|"){
            let getprop = "".to_owned() + &thissprite + "."+ &xanim;
            let getarray = vmap.getvar(&getprop);
            let mut fullpathstring = objectname.to_owned();
            let mut counter = 0;
            for xframe in split(&getarray,NC_ARRAY_DELIM){
                if counter !=0  && xframe != "" && counter > 0{
                    fullpathstring = fullpathstring.to_owned() + "|blueengine_textures." + &Nstring::stringtoeval(&dir) + "_" + &xframe + "_png" ;
                }
                if counter == 0 {
                    fullpathstring = fullpathstring + "|" + &xframe;
                }
                counter +=1;
            }

            //fullpathstring = Nstring::trimright(&fullpathstring,1);// cut off last pipe
            let newprop = "".to_owned() + &objectname + "." + &Nstring::replace(xanim,"anim_","texturearray_") ;
            vmap.setvar(newprop.to_owned(), &fullpathstring);
        #[cfg(debug_assertions)]
        println!("debug:new {} animarray:{}",&newprop,&fullpathstring);
            vmap.setvar(thissprite.clone() + ".animationtimer",&Ntimer::init().to_string());
            let mut animationhandlerarray = vmap.getvar("animationhandler.allsprites");
            animationhandlerarray = pooladd(&animationhandlerarray, &thissprite);
            vmap.setvar("animationhandler.allsprites".to_owned(), &animationhandlerarray);
        }

        vmap.setvar(dir.to_owned()+".name", &dir);

        // add a square to the animated node
        let sqrq = vmap.getvar("blueengine.square_q");
        vmap.setvar("blueengine.square_q".to_owned(),&pooladd(&sqrq,&objectname));

        return thissprite; // required per sprite set first thissprite in nscript
    }
    pub fn spritesetanimation(object:&str,anim:&str,vmap:&mut Varmap){
        let curanimprop = object.to_string()+".currentanim";
        if vmap.getvar(&curanimprop) != anim {
            vmap.setvar(curanimprop.to_string(), &anim);

            vmap.setvar(object.to_string()+".currentframe", "0");

            vmap.setvar(object.to_string()+".animationswitchtimer", &Ntimer::init().to_string());
            let key = "blueengine.anim_q";
            let getanimq = vmap.getvar(key);
            let animarraykey = "".to_owned() + &object + ".texturearray_" + &Nstring::replace(&anim,"anim_","");
            let animarray = vmap.getvar(&animarraykey);

            vmap.setvar("blueengine.anim_q".to_owned(), &pooladd(&getanimq,&animarray));
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
        guiactivemenus = pooladd(&guiactivemenus, &name);
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

}
