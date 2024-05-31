

#![allow(unused)]
use blue_engine::{
    primitive_shapes::{cube, square, triangle, uv_sphere},
    uniform_type::Matrix,
    utils::default_resources::DEFAULT_MATRIX_4,
    Engine, Instance, ObjectSettings, PolygonMode, PowerPreference, RotateAxis, ShaderSettings,
    TextureData, Vertex, WindowDescriptor,
};
//use blue_engine_egui::egui as gui;
use blue_engine_utilities::egui;
use blue_engine_utilities::egui::egui as gui;
use std::{char, env, net::SocketAddrV4};
mod includes{
    pub mod bluenc;
    pub mod maps;
    pub mod bn3models;
    pub mod corefunctions;
    pub mod textnodes;
}
pub use includes::bluenc::*;
pub use includes::maps::*;
use std::time::{Instant, Duration};
pub use includes::bn3models::*;

pub use includes::textnodes::*;

pub use includes::corefunctions::*;
//use blue_engine_utilities::FlyCamera;
//use nscript_v2::*;
//extern crate nscript_v2;
use std::{collections::{HashMap}, f32};
//use std::{env, array, string};
extern crate nscript;

use nscript::*;

fn vertex(x: f32,y: f32,z: f32) -> Vertex {
Vertex {
        position: [x,y, z],
        uv: [1.0, 0.0],
        normal: [0f32, 0f32, 0f32],
    }
}

struct Ncanimation{
    pub animation_map: HashMap<String,Vec<String>>,
    pub animation_i: HashMap<String,usize>,
    pub animation_timers:HashMap<String,i64>,

    pub animation_currentframe:HashMap<String,String>,
}
impl Ncanimation{
    pub fn new() -> Ncanimation{
        Ncanimation{
            animation_map: HashMap::new(),
            animation_i: HashMap::new(),
            animation_timers: HashMap::new(),
            animation_currentframe: HashMap::new(),

        }
    }
    pub fn set(&mut self,squareid: &str,animationarray: &Vec<String>){
        if animationarray.len() < 2 {
            //println!("error on the animation system, a given array doesnt meet the requirements, for ID:{}",squareid);
            return;
        }
        let start_index = 2;
        let end_index = animationarray.len(); // Use the length of the array for the end index
        // Create a slice from start_index to the end of the array
        let trimmed_slice:Vec<String> = animationarray[start_index..end_index].to_owned();
        //rintln!("Settingarrayy   = {:?}",trimmed_slice);
        self.animation_map.insert(squareid.to_string(),trimmed_slice );
        self.animation_i.insert(squareid.to_string(),0);

        let timer = match animationarray[1].parse::<i64>(){
            Ok(f) => {
                f
            }
            Err(e) => {
                20// default 100ms, however this should not be ok.
            }
        };
        self.animation_timers.insert(squareid.to_string(),timer);
        self.animation_timers.insert(squareid.to_string() + "_diff",timer);

    }
    pub fn get(&mut self,squareid: &str) -> String{
        match self.animation_map.get(squareid){
            Some(res) => {
                println!("Found: {:?}",res);
            }
            None =>{
                //print!("Animation texture error for: {}",squareid);
            }
        }
        "".to_string()
    }
    pub fn frame(&mut self,squareid:&str) -> String{
        if squareid == ""{
            return "".to_owned();
        }
        let animarray: Vec<String> = match self.animation_map.get(squareid){
            Some(res) => {
               res.to_owned()
            }
            None =>{
                //print!("Animation texture error for: {}",squareid);
                Vec::new()
            }
        };
        let mut anim_i: usize = match self.animation_i.get(squareid){
            Some(res) => {
               res.to_owned()
            }
            None =>{
                0
            }
        };
        let  frametime: i64 = match self.animation_timers.get(squareid){
            Some(res) => {
               res.to_owned()
            }
            None =>{
                0
            }
        };

        let timerdiffref = squareid.to_owned() + "_diff";
        let  timerdiff: i64 = match self.animation_timers.get(&timerdiffref){
            Some(res) => {
               res.to_owned()
            }
            None =>{
                0
             }
        };
        if Ntimer::diff(timerdiff) > frametime{
            if animarray.len() == 0 {
                return "".to_owned();
            }
            if anim_i >= animarray.len(){
                anim_i = 0
            }

            let thisframe = animarray[anim_i].to_string();
            anim_i = anim_i + 1;
            self.animation_i.insert(squareid.to_string(), anim_i);
            self.animation_timers.insert(timerdiffref, Ntimer::init());
            self.animation_currentframe.insert(squareid.to_string(),thisframe.clone());
             return thisframe;
        }
        return "".to_string();
        let  runningframe = match self.animation_currentframe.get(squareid){
            Some(res) => {
               res.to_owned()
            }
            None =>{
                "".to_string()
            }
        };
        return runningframe;

    }
}

fn main() {

  cwrite(" started!!","g");
    let mut vmap = Varmap::new(); // global
    let args: Vec<String> = env::args().collect();
    //println!("Program name: {}", args[0]);

    // Print other arguments passed to the program
    let mut newargs: Vec<String> = Vec::new();
    let arglen = args.len().clone();
    //println!("Argumentslengt:{}",&arglen);
    let mut i = 0;
    for givenarg in args.clone() {

            //println!("env:{}",&givenarg);
            let v = "".to_owned() + &givenarg.to_owned();
            let key ="cmdarg".to_owned() + &i.to_string();
vmap.setvar(key, &v);

            newargs.push(v);
        i +=1;


    }
    for _ in 0..10{
        //println!("empty args");
        newargs.push("".to_owned());

    }
    let mut initscript = NC_SCRIPT_DIR.to_owned() +"system/init.nc";
    let mut runinit = false;
    if args.len() > 2 {
        initscript = args[2].to_owned();
        if args[1] == "run" {
            runinit = true;
        }
    }


    // exent nscript core functions with blueengine functions!
    vmap.setextentionfunctions(nscript_bluenc_functionbindings);

    if runinit{
        nscript_execute_script(&initscript,&newargs[1],&newargs[2],&newargs[3],&newargs[4],&newargs[5],&newargs[6],&newargs[7],&newargs[8],&newargs[9],&mut vmap);

    }else{
    // allows a envoiremental variable ( default: home/nscript)
        let coreinitscript ="".to_owned() +  NC_SCRIPT_DIR + "system/BENC.nc";
        nscript_execute_script(&coreinitscript,&newargs[1],&newargs[2],&newargs[3],&newargs[4],&newargs[5],&newargs[6],&newargs[7],&newargs[8],&newargs[9],&mut vmap);


    }


    //let mut engine = Engine::new().expect("win");
    //let mut engine = Engine::new_config(WindowDescriptor { width: 1920, height: 1080, title: "Nscript&BlueEngine Testing",..Default::default()}).expect("win");
    let vsync = vmap.getvar("blueengine.vsync");
    let powermode = vmap.getvar("blueengine.powermode");
    let mut renderwidth = nscript_i32(&vmap.getvar("blueengine.renderwidth")) ;
    let mut renderheight = nscript_i32(&vmap.getvar("blueengine.renderheight"));
    let rendertitle = "static title needs to be fixed";
    let cfgtitle = vmap.getvar("blueengine.title");
    let mut cfgpowermode = blue_engine::PowerPreference::LowPower;
    if powermode == "true" {
       cfgpowermode = blue_engine::PowerPreference::HighPerformance;
    }
    let mut cfgvsync = blue_engine::wgpu::PresentMode::Fifo;
    if vsync == "false" {
        cfgvsync = blue_engine::wgpu::PresentMode::Immediate;
    }
    if renderwidth < 160 {
        renderwidth = 160
    }
    if renderheight < 120 {
        renderheight = 120
    }
    let mut engine = Engine::new_config(blue_engine::WindowDescriptor {
        width: renderwidth as u32, height: renderheight as u32, title: rendertitle,
        power_preference: cfgpowermode,
        present_mode: cfgvsync,
        features: blue_engine::header::imports::wgpu::Features::empty(),
        ..Default::default()
    })
    .expect("win");
engine.window.set_title(&cfgtitle);
    //engine.window.title("BlueEngine-Nscript");
    cwrite("engine started!!","g");
    //let mut engine = ENGINE.lock().unwrap();
    //let test_instance = Instance::default();
    //println!("{:?}", test_instance.to_raw());
let mut texture_map: HashMap<String, blue_engine::wgpu::BindGroup> = HashMap::new();
    // let gui_context = blue_engine_egui::EGUI::new(&engine.event_loop, &mut engine.renderer);
    //
    // // We add the gui as plugin, which runs once before everything else to fetch events, and once during render times for rendering and other stuff
    // engine.plugins.push(Box::new(gui_context));
let mut animationsystem = Ncanimation::new();
    let mut string:Vec<String> = Vec::new();
    string.push("123345".to_owned());
    //animationsystem.set("test",string);
    //animationsystem.get("test");
    //sleep(5000);
    let speed = -0.05;
    let objectsettignslayer1 = ObjectSettings {
        camera_effect: false,
        shader_settings: ShaderSettings::default(),
    };

if 1 == 2 {// lets test if its only inside the loop..
    // quee system to load textures , ( first nodes can be used to copy to others)
    let queetest = nscript_checkvar("blueengine.textureload_q", &mut vmap);
     //println!("DEBUG!!!!!!!!!!!!!{}",&queetest);
    //cwrite(&nscript_checkvar("blueengine.textureload_q", &mut vmap),"p");
    for i in vmap.getstringarray("blueengine.textureload_q"){//NC_ARRAY_DELIM
        let ckey = "blueengine_textures.".to_owned() + &Nstring::stringtoeval(&i);
        if nscript_checkvar(&ckey, &mut vmap) == "" && i != ""{
            #[cfg(debug_assertions)]
            cwrite(&ckey,"green");
            vmap.setvar("tmp".to_owned(),&i);
            if i != "" {
                //perform blue engine texture loading
                let t = engine
                    .renderer
                    .build_texture(
                        ckey.clone(),
                        TextureData::Path(i.to_string()),
                        blue_engine::TextureMode::Clamp,
                    )
                    .unwrap();
                // create a object with the texture to be cloned
                square(
                    ckey.clone()
                    ,
                    ObjectSettings::default(),
                    &mut engine.renderer,
                    &mut engine.objects,
                );
                // set the main texture !
                engine.objects.get_mut(&ckey).unwrap().set_texture(t);

                // move the object out of sight ( yeah i know blue it will be invisible :P )
                engine
                    .objects
                    .get_mut(&ckey)
                    .unwrap()
                    .set_position(-0.2f32, 0.1f32, -990.001f32);
                // let system know its used!
                vmap.setvar(ckey.clone(), &ckey);
                //let m = "texture added:".to_owned() + &ckey;
                //cwrite(&m,"purple")
            }


        }
        else{
            let m = "texture already exists:".to_owned() + &ckey;
            cwrite(&m,"r")
        }
            vmap.setstringarray("blueengine.textureload_q",Vec::new() );
    }

    for i in split(&vmap.getvar("blueengine.image2d_q"),NC_ARRAY_DELIM){//NC_ARRAY_DELIM
        //let ckey = "blueengine_textures.".to_owned() + &Nstring::stringtoeval(&i);
        if  i != "" {
            engine.objects.new_object(
                i,
                vec![
                    vertex(1.0, 1.0, 0.0),
                    Vertex {
                    position: [1.0, -1.0, 0.0],
                    uv: [1.0, 1.0],
                    normal: [0f32, 0f32, 0f32],
                    },
                    Vertex {
                    position: [-1.0, -1.0, 0.0],
                    uv: [0.0, 1.0],
                    normal: [0f32, 0f32, 0f32],
                    },
                    Vertex {
                    position: [-1.0, 1.0, 0.0],
                    uv: [0.0, 0.0],
                    normal: [0f32, 0f32, 0f32],
                    },
                ],
                vec![2, 1, 0, 2, 0, 3],
                ObjectSettings {
                    camera_effect: false,
                    ..Default::default()
                }
                ,
                &mut engine.renderer
            );
            engine.objects.update_object(i, |object| {
                object.set_render_order(0).unwrap();

                //object.set_scale(0.5, 0.5, 1f32);
                //object.set_position(0.0,0.0,-10.0);

    });           //layer1.flag_as_changed();
            //println!("bmpfont {}",i);
        }
    }
vmap.setvar("blueengine.image2d_q".to_owned(),"" );

    //:w
    //spanwning quee
    for i in split(&vmap.getvar("blueengine.square_q"),NC_ARRAY_DELIM){
        #[cfg(debug_assertions)]
        cwrite(&i,"green");
        if i != "" {
        square(
                i,
               ObjectSettings::default(),
                &mut engine.renderer,
            &mut engine.objects,
        );
    //         engine.objects.update_object(i, |object| {
    //             object.set_render_order(1).unwrap();
    //
    //             //object.set_scale(0.5, 0.5, 1f32);
    //             object.set_position(0.0,0.0,-10.0);
    //
    // });
        }
    }
vmap.setvar("blueengine.square_q".to_owned(),"" );

    // set positions quee
    for i in vmap.getstringarray("blueengine.position_q"){
        //cwrite(&i,"green");
        if i != "" {
            let data = split(&i,",");
            if data.len() > 3 {
                //cwrite(&i,"yellow");
                engine
                    .objects
                    .get_mut(data[0])
                    .unwrap()
                    .set_position(data[1].parse().unwrap_or(0.0), data[2].parse().unwrap_or(0.0), data[3].parse().unwrap_or(0.0));

            }
            else{
                cwrite("A split error on the position quee accuired","red");
                cwrite(&i,"red");

            }
        }
    }
vmap.setstringarray("blueengine.position_q",Vec::new() );

            // Bridge: Nscript setTexture q.
            let qbuffer = vmap.getstringarray("blueengine.textureset_q");
            for i in qbuffer{
                if i != ""{ // if queed items in pool

                    let data = split(&i,",");
                    if data.len() > 1  && data[0] != ""{
                //cwrite(&i,"m");
                        engine.objects
                            .get_mut(data[0])
                            .expect("Error during copying texture of the main square")
                            .reference_texture(data[1]);
                        //cwrite(data[1],"b")
                    }
                    else{
                        cwrite("Split error on the settexxture q","red");

                cwrite(&i,"red");
                    }
                    // remove the entree from the pool ( in nscript blueengine.position_quee )
                    //vmap.setvar("blueengine.textureset_q".to_owned(),&poolremove(&qbuffer,&i) );
                }
            }
vmap.setstringarray("blueengine.textureset_q",Vec::new() );

    for i in vmap.getstringarray("blueengine.update_q"){
        //cwrite(&i,"green");
        if i != "" {
            let prop = i.to_owned() + ".x";
            let newx = nscript_checkvar(&prop,&mut vmap);
            let prop = i.to_owned() + ".y";
            let newy = nscript_checkvar(&prop,&mut vmap);
            let prop = i.to_owned() + ".z";
            let newz = nscript_checkvar(&prop,&mut vmap);

                engine
                    .objects
                    .get_mut(&i)
                    .unwrap()
                    .set_position(newx.parse().unwrap_or(0.0), newy.parse().unwrap_or(0.0), newz.parse().unwrap_or(0.0));

        }
    }

vmap.setstringarray("blueengine.update_q",Vec::new() );

    for i in split(&vmap.getvar("blueengine.visibility_q"), NC_ARRAY_DELIM) {
        //cwrite(&i,"green");
        if i != "" {
            let isdata = split(i, ",");
            if isdata.len() > 1 {
                let mut isvisbool = true;
                if isdata[1] == "false" {
                    isvisbool = false;
                }
                let isobj = engine.objects.get_mut(isdata[0]).unwrap();
                isobj.is_visible = isvisbool;
            }
        }
    }

    vmap.setvar("blueengine.visibility_q".to_owned(), "");
    }
    //cwrite(&nscript_checkvar("blueengine.squarequee", &mut vmap),"red");

    // key mapping, used inside game_loop for bridging to nscript
    let keyvec = [
        blue_engine::KeyCode::Escape,
        blue_engine::KeyCode::ArrowUp,
        blue_engine::KeyCode::ArrowDown,
        blue_engine::KeyCode::ArrowLeft,
        blue_engine::KeyCode::ArrowRight,
        blue_engine::KeyCode::KeyA,
        blue_engine::KeyCode::KeyB,
        blue_engine::KeyCode::KeyC,
        blue_engine::KeyCode::KeyD,
        blue_engine::KeyCode::KeyE,
        blue_engine::KeyCode::KeyF,
        blue_engine::KeyCode::KeyG,
        blue_engine::KeyCode::KeyH,
        blue_engine::KeyCode::KeyI,
        blue_engine::KeyCode::KeyJ,
        blue_engine::KeyCode::KeyK,
        blue_engine::KeyCode::KeyL,
        blue_engine::KeyCode::KeyM,
        blue_engine::KeyCode::KeyN,
        blue_engine::KeyCode::KeyO,
        blue_engine::KeyCode::KeyP,
        blue_engine::KeyCode::KeyQ,
        blue_engine::KeyCode::KeyR,
        blue_engine::KeyCode::KeyS,
        blue_engine::KeyCode::KeyT,
        blue_engine::KeyCode::KeyU,
        blue_engine::KeyCode::KeyV,
        blue_engine::KeyCode::KeyW,
        blue_engine::KeyCode::KeyX,
        blue_engine::KeyCode::KeyY,
        blue_engine::KeyCode::KeyZ,
    ];
    let keyname = [
        // keymapping naming ( must contain the same size and order as the keymapping!!)
        "key.esc",
        "key.up",
        "key.down",
        "key.left",
        "key.right",
        "key.a",
        "key.b",
        "key.c",
        "key.d",
        "key.e",
        "key.f",
        "key.g",
        "key.h",
        "key.i",
        "key.j",
        "key.k",
        "key.l",
        "key.m",
        "key.n",
        "key.o",
        "key.p",
        "key.q",
        "key.r",
        "key.s",
        "key.t",
        "key.u",
        "key.v",
        "key.w",
        "key.x",
        "key.y",
        "key.z",
    ];
    let mut animationtimer = Ntimer::init();
    let mut uiselfname = String::new();

    let mut uiselfpath = Some("".to_string()); // String::new();
    let mut uiselfage = 1;
    // Start the egui context
    let gui_context = egui::EGUI::new(&mut engine.renderer, &engine.window);

    // We add the gui as plugin, which runs once before everything else to fetch events, and once during render times for rendering and other stuff
    engine.signals.add_signal("egui", Box::new(gui_context));
    let mut color = [1f32, 1f32, 1f32, 1f32];
    let mut fpscounter = 0;
    let codetimer = Instant::now();
    let mut fps = 0;
    let mut fpstimer = Ntimer::init();
    engine
        .update_loop(move |renderer, window, objects, input, camera, signals| {
            let egui_signal =  signals.get_signal::<egui::EGUI>("egui").unwrap();
            //let egui_plugin = egui_signal

                // downcast it to obtain the plugin
                // .downcast_mut::<egui::EGUI>()
                // .expect("Plugin not found");

            // ui function will provide the context
            fpscounter +=1;
            if Ntimer::diff(fpstimer) > 999 {
                fpstimer = Ntimer::init();
                fps = fpscounter.clone();
                vmap.setvar("blueengine.fps".to_string(),&fps.to_string());
                fpscounter = 0;
            }
            let eachmenu  = vmap.getvar("guiactivemenus");


            egui_signal.unwrap().ui(
                |ctx| {
                    for eachmenu in split(&vmap.getvar("guiactivemenus"),"|"){
                        if eachmenu != ""{
                            //et eachmenu = nscript_checkvar("gui.activemenu",&mut vmap);
                            let titleref = "".to_owned() + &eachmenu.trim() + ".title";
                            let title ="nc ".to_owned()+ &nscript_checkvar(&titleref,&mut vmap);
                            //println!("this menu{} title:{}",eachmenu,title);
                            gui::Window::new(&title).show(ctx, |ui| {
                                let controllarr = arraysearch(&Nstring::replace(&vmap.inobj(&eachmenu),"|",NC_ARRAY_DELIM),"type_");
                                 //let fpsmsg = "fps:".to_owned() + &vmap.getvar("fps") ;
                                 //ui.label(&fpsmsg);

                                //println!("inobj:{}",controllarr);
                                for eachcontrol in split(&controllarr,NC_ARRAY_DELIM){
                                    if eachcontrol != ""{
                                        let eachcontroltype = "".to_owned() + &eachmenu + "." + eachcontrol;
                                        let thistype = vmap.getvar(&eachcontroltype);
                                        match thistype.as_str(){
                                            "link" => {
                                                let mut inputvarlabelname = vmap.getvar(&Nstring::replace(&eachcontroltype, "type_", "control_"));//evaluate the variablename
                                                //let mut originalvarname = nscript_checkvar(&originalvarname,&mut vmap);// get actualvalue!
                                                let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                let mut originalvarname = vmap.getvar(&inputvarname);//evaluate the variablename
                                                //let mut inputvar = nscript_checkvar(&originalvarname,&mut vmap);// get actualvalue!

                                                ui.hyperlink_to(inputvarlabelname, originalvarname);
                                            }
                                            "checkbox" =>{
                                                let mut inputvarlabelname = vmap.getvar(&Nstring::replace(&eachcontroltype, "type_", "control_"));//evaluate the variablename
                                                //let mut originalvarname = nscript_checkvar(&originalvarname,&mut vmap);// get actualvalue!
                                                let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                let mut originalvarname = vmap.getvar(&inputvarname);//evaluate the variablename
                                                let mut inputvar = vmap.getvar(&originalvarname);// get actualvalue!
                                                let mut  mybool = match inputvar.as_str(){
                                                    "true" => true,
                                                    _ => false
                                                };
                                                let mut changedvar = mybool.clone();

                                                //ui.label(&inputvarlabelname);
                                                //ui.checkbox(&mut mybool, &inputvarlabelname);
                                                ui.add(gui::Checkbox::new(&mut mybool, &inputvarlabelname));
                                                if changedvar != mybool{
                                                    vmap.setvar(originalvarname,&mybool.to_string());

                                                }
                                            }
                                            "radio" => {
                                                let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                let mut originalvarname = vmap.getvar(&inputvarname);//evaluate the variablename
                                                let mut inputvar = vmap.getvar(&originalvarname);// get actualvalue!
                                                let mut selected  = inputvar.clone();

                                                let mut inputname = vmap.getvar(&Nstring::replace(&eachcontroltype, "type_", "control_"));

                                                let arrayitems = vmap.getvar(&Nstring::replace(&eachcontroltype, "type_", "func_"));
                                                let arrayitems = vmap.getvar(&arrayitems);
                                                let selidvarname = Nstring::replace(&eachcontroltype, "type_", "selected_");
                                                let selidval = vmap.getvar(&selidvarname);
                                                let selid = vmap.getvar(&selidval);
                                                let mut selected = nscript_usize(&selid);
                                                let alternatives = split(&arrayitems,NC_ARRAY_DELIM);
                                                //let mut selected = 0;
                                                let mut sel2 = selected.clone();
                                                let mut i = 0;
                                                ui.label(&inputname);
                                                for x in alternatives.clone(){

                                                    if i != selected {

                                                        if ui.radio(false,x).clicked(){
                                                            sel2 = i;
                                                        };
                                                    }else{
                                                        // if ui.radio(true,alternatives[selected]).clicked(){
                                                        //     sel2 = i;
                                                        // };
                                                        ;

                                                    }
                                                    i +=1;
                                                }

                                                if selected != sel2{
                                                    vmap.setvar(originalvarname,&alternatives[sel2]);

                                                    vmap.setvar(selidvarname,&sel2.to_string());
                                                    //println!("nput:{}",inputvar);
                                                }
                                            }
                                            "combo" => {
                                                let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                let mut originalvarname = vmap.getvar(&inputvarname);//evaluate the variablename
                                                let mut inputvar = vmap.getvar(&originalvarname);// get actualvalue!
                                                let mut selected  = inputvar.clone();

                                                let mut inputname = vmap.getvar(&Nstring::replace(&eachcontroltype, "type_", "control_"));

                                                let arrayitems = vmap.getvar(&Nstring::replace(&eachcontroltype, "type_", "func_"));

                                                let arrayitems = vmap.getvar(&arrayitems);
                                                let selidvarname = Nstring::replace(&eachcontroltype, "type_", "selected_");
                                                let selid = vmap.getvar(&selidvarname);
                                                let mut selected = match selid.parse::<usize>(){
                                                    Ok(res) =>{
                                                        res
                                                    }
                                                    Err(e) =>{
                                                        0
                                                    }
                                                };
                                                let alternatives = split(&arrayitems,NC_ARRAY_DELIM);
                                                //let mut selected = 0;
                                                if selected >= alternatives.len(){
                                                    selected = alternatives.len() - 1;
                                                }
                                                gui::ComboBox::from_label(&inputvar).show_index(
                                                    ui,
                                                    &mut selected,
                                                    alternatives.len(),
                                                    |selected| alternatives[selected]
                                                );
                                                if alternatives[selected] != inputvar{
                                                    vmap.setvar(originalvarname,&alternatives[selected]);

                                                    vmap.setvar(selidvarname,&selected.to_string());
                                                    //println!("nput:{}",inputvar);
                                                }
                                            }
                                            "password" =>{
                                                let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                let mut originalvarname = vmap.getvar(&inputvarname);//evaluate the variablename
                                                let mut inputvar = vmap.getvar(&originalvarname);// get actualvalue!
                                                let mut changedvar = inputvar.clone();

                                                let mut inputname = vmap.getvar(&Nstring::replace(&eachcontroltype, "type_", "control_"));


                                                ui.horizontal(|ui| {
                                                    let name_label = ui.label(&inputname);
                                                    //let field = ui::TextEdit::singleline(&mut changedvar);
                                                    //let field = ui.text_edit_singleline(&mut changedvar);

                                                    //ui.label(name_label);
                                                    let textfield = ui.add_sized(
                                                        [ui.available_width(), 24.],
                                                        gui::TextEdit::singleline(&mut changedvar).password(true),
                                                    );//.labelled_by(name_label.id);
                                                    //(true);

                                                    if changedvar != inputvar{
                                                        vmap.setvar(originalvarname,&changedvar);
                                                        //println!("nput:{}",inputvar);
                                                    }

                                                    //println!("input name:{}",inputname);
                                                });

                                            }
                                            "input" =>{
                                                let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                let mut originalvarname = vmap.getvar(&inputvarname);//evaluate the variablename
                                                let mut inputvar = vmap.getvar(&originalvarname);// get actualvalue!
                                                let mut changedvar = inputvar.clone();

                                                let mut inputname = vmap.getvar(&Nstring::replace(&eachcontroltype, "type_", "control_"));


                                                ui.horizontal(|ui| {
                                                    let name_label = ui.label(&inputname);
                                                    ui.text_edit_singleline(&mut changedvar)
                                                        .labelled_by(name_label.id);

                                                    if changedvar != inputvar{
                                                        vmap.setvar(originalvarname,&changedvar);
                                                        //println!("nput:{}",inputvar);
                                                    }

                                                    //println!("input name:{}",inputname);
                                                });
                                            }
                                            "label" =>{
                                                let inputvarname = Nstring::replace(&eachcontroltype, "type_", "control_");
                                                let mut originalvarname = vmap.getvar(&inputvarname);//evaluate the variablename
                                                let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                let mut inputvar = vmap.getvar(&inputvarname);//evaluate the variablename
                                                let mut inputvar = vmap.getvar(&inputvar);
                                                ui.label("".to_owned()+&originalvarname+ &inputvar);

                                            }
                                            "button" =>{


                                                let mut inputname = vmap.getvar(&Nstring::replace(&eachcontroltype, "type_", "control_"));

                                                if ui.button(&inputname).clicked() {
                                                    let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                    let mut inputvar = nscript_checkvar(&inputvarname,&mut vmap);//evaluate the variablename
                                                    let sheet = nscript_checkvar(&inputvar,&mut vmap);
                                                    let mut inputvar = nscript_parsesheet(&sheet,&mut vmap);// get actualvalue!

                                                }
                                            }
                                            "fileopen" =>{


                                                let mut inputname = vmap.getvar(&Nstring::replace(&eachcontroltype, "type_", "control_"));

                                                if ui.button(&inputname).clicked() {
                                                    if let Some(path) = rfd::FileDialog::new().pick_file() {

                                                        let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                        let mut inputvarnameraw = vmap.getvar(&inputvarname);//evaluate the variablename
                                                        let mut inputvar = vmap.getvar(&inputvarnameraw);// get actualvalue!
                                                        vmap.setvar(inputvarnameraw.to_string(),&format!("{:?}",path));
                                                        let inputvarname = Nstring::replace(&eachcontroltype, "type_", "func_");
                                                        let mut fnvar = vmap.getvar(&inputvarname);//evaluate the variablename
                                                        //let mut fnvar = nscript_checkvar(&fnvar,&mut vmap);// get actualvalue!
                                                        let nscriptfunctiontorun = "".to_owned() + &fnvar + "(" + &format!("{:?}",path) +")";
                                                        println!("file - >{} \n tovar: {}",nscriptfunctiontorun,&inputvarnameraw);
                                                        nscript_checkvar(&nscriptfunctiontorun,&mut vmap);

                                                    }

                                                }
                                            }
                                            "filesave" =>{


                                                let mut inputname = vmap.getvar(&Nstring::replace(&eachcontroltype, "type_", "control_"));

                                                if ui.button(&inputname).clicked() {
                                                    if let Some(path) = rfd::FileDialog::new().save_file() {

                                                        let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                        let mut inputvarnameraw = vmap.getvar(&inputvarname);//evaluate the variablename
                                                        let mut inputvar = vmap.getvar(&inputvarnameraw);// get actualvalue!
                                                        vmap.setvar(inputvarnameraw,&format!("{:?}",path));
                                                        let inputvarname = Nstring::replace(&eachcontroltype, "type_", "func_");
                                                        let mut fnvar = vmap.getvar(&inputvarname);//evaluate the variablename
                                                        //fnvar = vmap.getvar(&fnvar);
                                                        //let mut fnvar = nscript_checkvar(&fnvar,&mut vmap);// get actualvalue!
                                                        let nscriptfunctiontorun = "".to_owned() + &fnvar + "(" + &format!("{:?}",path) +")";
                                                       println!("file - >{}",nscriptfunctiontorun);
                                                        nscript_checkvar(&nscriptfunctiontorun,&mut vmap);

                                                    }

                                                }
                                            }
                                            "slider" =>{
                                                let labelname = vmap.getvar(&eachcontroltype);
                                                let inputvarname =  Nstring::replace(&eachcontroltype, "type_", "var_");
                                                let inputmin = match vmap.getvar(&Nstring::replace(&eachcontroltype, "type_", "min_")).parse::<u32>(){
                                                    Ok(res) => {
                                                        res
                                                    }
                                                    Err(_) =>{
                                                        0
                                                    }
                                                };

                                                let inputmax = match vmap.getvar(&Nstring::replace(&eachcontroltype, "type_", "max_")).parse::<u32>(){
                                                    Ok(res) => {
                                                        res
                                                    }
                                                    Err(_) =>{
                                                        0
                                                    }
                                                };
                                                let mut inputvarnameori = vmap.getvar(&inputvarname);//evaluate the variablename
                                                let mut inputvar = match vmap.getvar(&inputvarnameori).parse::<u32>(){
                                                    Ok(res) => {
                                                        res
                                                    }
                                                    Err(_) =>{
                                                        0
                                                    }
                                                };// get actualvalue!
                                                let mut changedvar = inputvar.clone();
                                                ui.add(gui::Slider::new(&mut changedvar, inputmin..=inputmax).text(labelname));
                                                if changedvar != inputvar{
                                                    //println!("nput:setting{} with:{}",&inputvarname,&inputvar);

                                                    vmap.setvar(inputvarnameori,&changedvar.to_string());
                                                }
                                            }
                                            "color" => {

                                                let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                let mut originalvarname = vmap.getvar(&inputvarname);//evaluate the variablename
                                                let mut inputvar = vmap.getvar(&originalvarname);// get actualvalue!
                                                let mut changedvar = inputvar.clone();

                                                let mut inputname = vmap.getvar(&Nstring::replace(&eachcontroltype, "type_", "control_"));


                                                // ui.horizontal(|ui| {
                                                ui.label(&inputname);
                                                ui.color_edit_button_rgba_unmultiplied(&mut color);
                                                changedvar = "".to_owned() + &color[0].to_string() + "," + &color[1].to_string() + "," + &color[2].to_string() + "," + &color[3].to_string() ;
                                                if changedvar != inputvar{
                                                    //println!("nput:setting{} with:{}",&inputvarname,&inputvar);

                                                    vmap.setvar(originalvarname,&changedvar.to_string());
                                                }

                                            }
                                            _ => {
                                                println!("uhmm egui type went messed up, unknowntype :{}",thistype);
                                            }
                                        }
                                    }

                                }
                            });
                        }
                    }
                },
                &window,
            );
                //}




            // egui_plugin.ui(
            //     |ctx| {
            //         gui::Window::new("title").show(ctx, |ui| {
                        // ui.horizontal(|ui| {
                        //     ui.label("Pick a color");
                        //     ui.color_edit_button_rgba_unmultiplied(&mut color);
                        // });
                        // ui.horizontal(|ui| {
                        //     let name_label = ui.label("Your name: ");
                        //     ui.text_edit_singleline(&mut uiselfname)
                        //         .labelled_by(name_label.id);
                        // });
                        // ui.add(gui::Slider::new(&mut uiselfage, 0..=120).text("age"));
                        // if ui.button("Click each year").clicked() {
                        //     uiselfage += 1;
                        // }
                        // ui.label(format!("Hello '{}', age {}", uiselfname,uiselfage));
                        //
                        // ui.label("Drag-and-drop files onto the window!");
                        //
                        // if ui.button("Open file…").clicked() {
                        //     if let Some(path) = rfd::FileDialog::new().pick_file() {
                        //         uiselfpath = Some(path.display().to_string());
                        //     }
                        // }
                        //
                        // if let Some(picked_path) = &uiselfpath {
                        //     ui.horizontal(|ui| {
                        //         ui.label("Picked file:");
                        //         ui.monospace(picked_path);
                        //     });
                        // }
                    //});
                    // vmap.getvar("");
                    // objects
                    //     .get_mut("dude1")
                    //     .unwrap()
                    //     .set_uniform_color(color[0], color[1], color[2], color[3])
                    //     .unwrap();
            //     },
            //     &window,
            // );
            // run all nscript loops , (quees be filled here and bridged to the render engine)
            let elapsed_time = codetimer.elapsed().as_millis();
            if elapsed_time > 16 {
            nscript_loops(&mut vmap);
            vmap.setvar("key.event".to_owned(),"false");
                let codetimer = Instant::now();


                //
                let qbuffer = vmap.getstringarray("blueengine.deletion_q");
                if qbuffer.len() != 0{
                    for i in qbuffer{
                        if i != ""{ // if queed items in pool

                            objects.remove(&i);

                            // remove the entree from the pool
                            //vmap.setvar("blueengine.camera_q".to_owned(),&poolremove(&qbuffer,&i) );

                        }
                    }
                }
                vmap.setstringarray("blueengine.deletion_q",Vec::new());//clearpool

                let pngfontq = vmap.getstringarray("blueengine.image2d_q");
                if pngfontq.len() != 0 {
                    for i in pngfontq{//NC_ARRAY_DELIM
                        //let ckey = "blueengine_textures.".to_owned() + &Nstring::stringtoeval(&i);
                        if  i != ""{

                            objects.new_object(
                                i.to_string(),
                                vec![
                                    Vertex {
                                        position: [1.0, 1.0, 0.0],
                                        uv: [1.0, 0.0],
                                        normal: [0f32, 0f32, 0f32],
                                    },
                                    Vertex {
                                        position: [1.0, -1.0, 0.0],
                                        uv: [1.0, 1.0],
                                        normal: [0f32, 0f32, 0f32],
                                    },
                                    Vertex {
                                        position: [-1.0, -1.0, 0.0],
                                        uv: [0.0, 1.0],
                                        normal: [0f32, 0f32, 0f32],
                                    },
                                    Vertex {
                                        position: [-1.0, 1.0, 0.0],
                                        uv: [0.0, 0.0],
                                        normal: [0f32, 0f32, 0f32],
                                    },
                                ],
                                vec![2, 1, 0, 2, 0, 3],
                                objectsettignslayer1,
                                renderer
                            );

                            let layer1 = objects
                                .get_mut(&i)
                                .expect("failed to gete object");

                            layer1.set_render_order(1).unwrap();
                            layer1.camera_effect = false;
                            //layer1.flag_as_changed();
                            //println!("sqaure {}",i);
                        }
                    }
                }
vmap.setstringarray("blueengine.image2d_q",Vec::new() );

                let textureloadq = vmap.getstringarray("blueengine.textureload_q");
                if textureloadq.len() != 0{
                    for i in textureloadq{//NC_ARRAY_DELIM
                        let ckey = "blueengine_textures.".to_owned() + &Nstring::stringtoeval(&i);
                        if nscript_checkvar(&ckey, &mut vmap) == "" && i != ""{
                            #[cfg(debug_assertions)]
                            cwrite(&ckey,"green");
                            vmap.setvar("tmp".to_owned(),&i);
                            if i != "" {
                                //perform blue engine texture loading
                                let t =
                                renderer
                                    .build_texture(
                                        ckey.clone(),
                                        TextureData::Path(i.to_owned()),
                                        blue_engine::TextureMode::Clamp,
                                    )
                                    .unwrap();
                                // create a object with the texture to be cloned
                                square(
                                    ckey.clone()
                                    ,
                                    ObjectSettings::default(),
                                    renderer,
                                    objects,
                                );
                                // set the main texture !
                                objects.get_mut(&ckey).unwrap().set_texture(t);

                                // move the object out of sight ( yeah i know blue it will be invisible :P )

                                objects
                                    .get_mut(&ckey)
                                    .unwrap()
                                    .set_position(-0.2f32, 0.1f32, -990.001f32);
                                // let system know its used!
                                vmap.setvar(ckey.clone(), &ckey);
                                //let m = "texture added:".to_owned() + &ckey;
                                //cwrite(&m,"purple")
                            }


                        }
                        // else{
                        //     let m = "texture already exists:".to_owned() + &ckey;
                        //     cwrite(&m,"r")
                        // }
                    }
                    vmap.setstringarray("blueengine.textureload_q",Vec::new() );
                }

                let qbuffer = vmap.getvar("blueengine.bmpfonttextureset_q");
                if qbuffer != ""{
                    for i in split(&qbuffer,NC_ARRAY_DELIM){
                        if i != ""{ // if queed items in pool
                            //println!("text:{}",i);
                            let data = split(&i,",");
                            if data.len() > 0  && data[0] != ""{
                                objects
                                    .get_mut(data[0])
                                    .expect("Error during copying texture of the main square")
                                    .reference_texture(data[1]);
                                //cwrite(data[1],"b")
                            }
                            else{
                                cwrite("Split error on the settexture q","red")
                            }
                            // remove the entree from the pool ( in nscript blueengine.position_quee )
                            //vmap.setvar("blueengine.textureset_q".to_owned(),&poolremove(&qbuffer,&i) );
                        }
                    }
                }
                vmap.setvar("blueengine.bmpfonttextureset_q".to_owned(),"" );


                let qbuffer = vmap.getstringarray("blueengine.square_q");
                if qbuffer.len() != 0 {
                    for i in qbuffer{//NC_ARRAY_DELIM
                        //let ckey = "blueengine_textures.".to_owned() + &Nstring::stringtoeval(&i);
                        if  i != ""{
                            objects.new_object(
                                i.to_string(),

                                vec![
                                    Vertex {
                                        position: [1.0, 1.0, 0.0],
                                        uv: [1.0, 0.0],
                                        normal: [0f32, 0f32, 0f32],
                                    },
                                    Vertex {
                                        position: [1.0, -1.0, 0.0],
                                        uv: [1.0, 1.0],
                                        normal: [0f32, 0f32, 0f32],
                                    },
                                    Vertex {
                                        position: [-1.0, -1.0, 0.0],
                                        uv: [0.0, 1.0],
                                        normal: [0f32, 0f32, 0f32],
                                    },
                                    Vertex {
                                        position: [-1.0, 1.0, 0.0],
                                        uv: [0.0, 0.0],
                                        normal: [0f32, 0f32, 0f32],
                                    },
                                ],
                                vec![2, 1, 0, 2, 0, 3],
                                ObjectSettings::default(),
                                renderer
                            );
                            let thisobj = objects
                                .get_mut(&i)
                                .expect("Error getting ref");
                            //let splitdata = split(i,",");

                            //thisobj.set_position(0.0,0.0,-5.0);
                            //println!("sqaure {}",i);
                        }
                    }

                    vmap.setstringarray("blueengine.square_q",Vec::new() );
                }


                // Bridge: Nscript setTexture q.
                let qbuffer = vmap.getstringarray("blueengine.textureset_q");
                if qbuffer.len() != 0 {
                    for i in qbuffer{
                        if i != ""{ // if queed items in pool
                            //println!("text:{}",i);
                            let data = split(&i,",");
                            if data.len() > 0  && data[0] != ""{
                                objects
                                    .get_mut(data[0])
                                    .expect("Error during copying texture of the main square")
                                    .reference_texture(data[1]);
                                //cwrite(data[1],"b")
                            }
                            else{
                                cwrite("Split error on the settexture q","red")
                            }
                            // remove the entree from the pool ( in nscript blueengine.position_quee )
                            //vmap.setvar("blueengine.textureset_q".to_owned(),&poolremove(&qbuffer,&i) );
                        }
                    }
                    vmap.setstringarray("blueengine.textureset_q",Vec::new() );
                }


                // Bridge: Nscript Scale Quee handler.
                // in nscript class blueengine.roation_q is used by func blueengine.setscale()
                let qbuffer = vmap.getstringarray("blueengine.scale_q");
                if qbuffer.len() != 0 {
                    for i in qbuffer{
                        if i != ""{ // if queed items in pool

                            let data = split(&i,",");
                            if data.len() > 2 && data[0] != ""{// check obj,x,y,z
                                objects
                                    .get_mut(data[0])
                                    .unwrap()
                                    .resize(data[1].parse().unwrap_or(0.0), data[2].parse().unwrap_or(0.0), data[3].parse().unwrap_or(0.0),window.inner_size());
                                objects.update_object(data[0], |object| {


                                    object.set_scale(data[1].parse().unwrap_or(0.0), data[2].parse().unwrap_or(0.0), data[3].parse().unwrap_or(0.0));


                                });;
                            }
                            else{
                                cwrite("Split error on the scale q","red")
                            }
                            // remove the entree from the pool ( in nscript blueengine.position_quee )
                            //vmap.setvar("blueengine.scale_q".to_owned(),&poolremove(&qbuffer,&i) );

                        }
                    }
                    vmap.setstringarray("blueengine.scale_q",Vec::new() );
                }

                let qbuffer = vmap.getstringarray("blueengine.color_q");
                if qbuffer.len() != 0 {
                    for i in qbuffer{
                        if i != ""{ // if queed items in pool
                            let data = split(&i,",");
                            if data.len() > 4 {
                                let color1 = match data[1].parse::<f32>(){
                                    Ok(r) => r,
                                    Err(e) => 0.0,
                                };
                                let color2 = match data[2].parse::<f32>(){
                                    Ok(r) => r,
                                    Err(e) => 0.0,
                                };
                                let color3 = match data[3].parse::<f32>(){
                                    Ok(r) => r,
                                    Err(e) => 0.0,
                                };
                                let color4 = match data[4].parse::<f32>(){
                                    Ok(r) => r,
                                    Err(e) => 0.0,
                                };

                                objects
                                    .get_mut(data[0])
                                    .unwrap()
                                    .set_uniform_color(color1, color2, color3, color4)
                                    .unwrap();
                            }
                        }
                    }
                    vmap.setstringarray("blueengine.color_q",Vec::new() );
                }
                // Bridge: Nscript scale Quee handler.
                // in nscript class blueengine.position_q is used by func blueengine.setposition()

                let qbuffer = vmap.getstringarray("blueengine.position_q");
                if qbuffer.len() != 0 {
                    for i in qbuffer{
                        if i != ""{ // if queed items in pool

                            let data = split(&i,",");
                            if data.len() > 3 && data[0] != ""{// check obj,x,y,z
                                // let thisobj = objects
                                //     .get_mut(data[0])
                                //     .unwrap()
                                //     .set_position(data[1].parse().unwrap_or(0.0), data[2].parse().unwrap_or(0.0), data[3].parse().unwrap_or(0.0));
                                objects.update_object(data[0], |object| {

                                    object.set_position(data[1].parse().unwrap_or(0.0), data[2].parse().unwrap_or(0.0), data[3].parse().unwrap_or(0.0))

                                });
                            }
                            else{
                                cwrite("Split error on the position quee","red")
                            }
                            // remove the entree from the pool ( in nscript blueengine.position_quee )
                            //vmap.setvar("blueengine.position_q".to_owned(),&poolremove(&qbuffer,&i) );

                        }
                    }
                    vmap.setstringarray("blueengine.position_q",Vec::new());
                }



                // Bridge: Nscript rotation Quee handler.
                // in nscript class blueengine.roation_q is used by func blueengine.setrotatation()
                let qbuffer = vmap.getstringarray("blueengine.rotation_q");
                if qbuffer.len() != 0 {
                    for i in qbuffer{
                        if i != ""{ // if queed items in pool
                            //println!("debug:{}",i);
                            let data = split(&i,",");
                            if data.len() > 2 {// check obj,x,y,z
                                let tcurrent = data[0].to_owned() + ".r" + data[2];
                                let tmpcurrotated = match vmap.getvar(&tcurrent).parse::<f32>(){
                                    Ok(res) =>{
                                        res
                                    }
                                    Err(err) => {
                                        //println!("[Bluenc] Error on the rotation Q (tmpcurrent) cannot convert string to f32 {}",i);
                                        0.0

                                    }

                                };
                                let thas = data[0].to_owned() + ".hr" + data[2];
                                let tmphasrotated = match vmap.getvar(&thas).parse::<f32>(){
                                    Ok(res) =>{
                                        res
                                    }
                                    Err(err) => {
                                        //println!("[Bluenc] Error on the rotation Q (hasrotateD) cannot convert string to f32 {}",i);
                                        0.0

                                    }

                                };

                                let datatoset = match data[1].parse::<f32>(){
                                    Ok(res) =>{
                                        res
                                    }
                                    Err(err) => {
                                        println!("[Bluenc] Error on the rotation Q cannot convert string to f32 {}",i);
                                        0.0

                                    }
                                };
                                let torotate = 0.0 - tmphasrotated + datatoset;
                                vmap.setvar(thas,&datatoset.to_string());
                                vmap.setvar(tcurrent,data[1]);
                                let axis = match data[2]{
                                    "x" => {
                                        RotateAxis::X
                                    }
                                    "y" => {
                                        RotateAxis::Y
                                    }
                                    "z" => {
                                        RotateAxis::Z
                                    }
                                    _ => {
                                        println!("error on the split rotation_q  data={}",i);
                                        RotateAxis::X
                                    }
                                };
                                objects
                                    .get_mut(data[0])
                                    .unwrap()
                                    .set_rotatation(torotate, axis);

                            }
                            else{
                                cwrite("Split error on the rotation quee","red");
                                cwrite(&i,"r");
                            }
                            // remove the entree from the pool ( in nscript blueengine.position_quee )
                            //vmap.setvar("blueengine.rotation_q".to_owned(),&poolremove(&qbuffer,&i) );//clearpool

                        }
                    }
                    vmap.setstringarray("blueengine.rotation_q",Vec::new());
                }

            let qbuffer = vmap.getstringarray("blueengine.camera_q");
                if qbuffer.len() != 0 {
                    for i in qbuffer{
                        if i != ""{ // if queed items in pool
                            let data = split(&i,",");
                            if data.len() > 2 {// check obj,x,y,z
                                camera
                                    .set_position(data[0].parse().unwrap_or(0.0), data[1].parse().unwrap_or(0.0), data[2].parse().unwrap_or(0.0))
                                    .expect("Couldn't update the camera eye");
                                if data.len() > 5 {
                                    camera
                                        .set_target(data[3].parse().unwrap_or(0.0), data[4].parse().unwrap_or(0.0), data[5].parse().unwrap_or(0.0))
                                        .expect("Couldn't update the camera eye");
                                }
                                else{
                                    camera.set_target(data[0].parse().unwrap_or(0.0), data[1].parse().unwrap_or(0.0), data[2].parse().unwrap_or(0.0))
                                        .expect("Couldn't update the camera eye");

                                }

                            }
                            else{
                                cwrite("Split error on the rotation quee ","red")
                            }
                            // remove the entree from the pool
                            //vmap.setvar("blueengine.camera_q".to_owned(),&poolremove(&qbuffer,&i) );

                        }
                    }
                    vmap.setstringarray("blueengine.camera_q",Vec::new());//clearpool
                }


            // Keyboard-input bridge , registers key.* classproperties to down or up
            let mut idx = 0;
            for xkey in keyname{
                if input.key_held(keyvec[idx]) {
                    vmap.setvar(keyname[idx].to_owned(),"down");
                    vmap.setvar("key.event".to_owned(),"true");
                    //cwrite(&keyname[idx],"y")
                }
                else{
                    vmap.setvar(keyname[idx].to_owned(),"up");
                }
                idx +=1;
            }

                let qbuffer = vmap.getstringarray("blueengine.anim_q");
                //println!("animq:{}",qbuffer);
                if qbuffer.len() != 0 {
                    for sprite in qbuffer {
                        if sprite != "" {
                            //println!("full: {}",sprite);
                            let spritesplit:Vec<String> = sprite.split("|").map(String::from).collect();
                            //println!("array: {:?}",spritesplit);
                            if spritesplit.len() > 1{
                                animationsystem.set(&spritesplit[0],&spritesplit);
                            }
                        }

                    }
                    vmap.setstringarray("blueengine.anim_q",Vec::new());//clearpool
                }

                if Ntimer::diff(animationtimer) > 20 {
                    let getq = vmap.getstringarray("animationhandler.allsprites");
                    if getq.len() != 0 {
                        for sprite in getq{
                            let thisanim = animationsystem.frame(&sprite);
                            if thisanim != ""{
                                objects
                                    .get_mut(&sprite)
                                    .expect("Error during copying texture of the main square")
                                    .reference_texture(thisanim);

                            }
                        }
                    }
                    animationtimer = Ntimer::init();
                }
                let qbuffer = vmap.getstringarray("blueengine.update_q");
                if qbuffer.len() != 0 {
                    for i in qbuffer{
                        //cwrite(&i,"green");
                        if i != "" {

                            let prop = i.to_owned() + ".x";
                            let newx = nscript_checkvar(&prop,&mut vmap);
                            let prop = i.to_owned() + ".y";
                            let newy = nscript_checkvar(&prop,&mut vmap);
                            let prop = i.to_owned() + ".z";
                            let newz = nscript_checkvar(&prop,&mut vmap);


                            objects
                                .get_mut(&i)
                                .unwrap()
                                .set_position(newx.parse().unwrap_or(0.0), newy.parse().unwrap_or(0.0), newz.parse().unwrap_or(0.0));
                            //println!("u {} x {} y {} z {}",i,newx,newy,newz);

                        }
                    }
                    vmap.setstringarray("blueengine.update_q",Vec::new() )
                }

                let qbuffer = vmap.getstringarray("blueengine.textnode_q");
                if qbuffer.len() != 0{

                    for i in qbuffer{
                        if i != ""{ // if queed items in pool
                            let mut counter = 1;

                            let prop = i.to_string() + ".text";
                            let text = vmap.getvar(&prop);
                            let prop = i.to_string() + ".x";
                            let text_x = nscript_f32(&vmap.getvar(&prop));
                            let prop = i.to_string() + ".y";
                            let text_y = nscript_f32(&vmap.getvar(&prop));
                            let prop = i.to_string() + ".size";
                            let text_size = nscript_f32(&vmap.getvar(&prop));
                            let scalewidth = text_size / 20.0;
                            let scaleheight = text_size / 16.0;
                            let mut distancebetweenchars: f32;
                            distancebetweenchars = text_size / (1400.0 - (text_size * 21.0));

                            let prop = i.to_string() + ".font";
                            let usedfont = vmap.getvar(&prop);
                            let mut setposx = text_x.clone();
                            for xchar in split(&text,""){
                                if xchar != "" {
                                    let charnode = i.to_string() + "_textchar_" + &counter.to_string();
                                    counter +=1;
                                    objects.new_object(
                                        charnode.clone(),
                                        vec![
                                            Vertex {
                                                position: [1.0, 1.0, 0.0],
                                                uv: [1.0, 0.0],
                                                normal: [0f32, 0f32, 0f32],
                                            },
                                            Vertex {
                                                position: [1.0, -1.0, 0.0],
                                                uv: [1.0, 1.0],
                                                normal: [0f32, 0f32, 0f32],
                                            },
                                            Vertex {
                                                position: [-1.0, -1.0, 0.0],
                                                uv: [0.0, 1.0],
                                                normal: [0f32, 0f32, 0f32],
                                            },
                                            Vertex {
                                                position: [-1.0, 1.0, 0.0],
                                                uv: [0.0, 0.0],
                                                normal: [0f32, 0f32, 0f32],
                                            },
                                        ],
                                        vec![2, 1, 0, 2, 0, 3],
                                        objectsettignslayer1,
                                        renderer
                                    );

                                    let  mut layer1 = objects
                                        .get_mut(&charnode)
                                        .expect("failed to gete object");

                                    layer1.set_render_order(1).unwrap();
                                    layer1.camera_effect = false;
                                    let textureprop = usedfont.to_string() + "." + &Bluenc::pngcharname(&xchar);

                                    let texture = vmap.getvar(&textureprop);
                                    objects
                                        .get_mut(&charnode)
                                        .expect("Error during copying texture of the main square")
                                        .reference_texture(texture.clone());
                                    vmap.setvar(charnode.to_string()+".texture",&texture);


                                    objects
                                        .get_mut(&charnode)
                                        .unwrap()
                                        .resize(scalewidth, scaleheight, 0.0,window.inner_size());
                                    objects.update_object(&charnode[..], |object| {


                                        object.set_scale(scalewidth,scaleheight,0.0);


                                    });;
                                    objects
                                        .get_mut(&charnode)
                                        .unwrap()
                                        .set_position(setposx, text_y, nscript_f32("0.0"));

                                    match xchar {// custom adjusting for characters
                                        "1" | "i" | " " | "." | "," => {
                                            setposx = setposx + (distancebetweenchars*0.6);
                                        }
                                        "f" | "t" =>{
                                            setposx = setposx + (distancebetweenchars*0.8);
                                        }

                                        "y" | "z" | "x" | "d" | "u" | "r" | "s" | "o" | "c" | "a" =>{
                                            setposx = setposx + (distancebetweenchars*1.05);
                                        }
                                        "w" | "\\" | "/" | "#" | "m" => {

                                            setposx = setposx + (distancebetweenchars*1.25);
                                        }
                                        _ =>{
                                            setposx = setposx + distancebetweenchars;
                                        }
                                    }
                                    #[cfg(debug_assertions)]
                                    println!("textnode {} char {} posx{}",&i,&charnode,&setposx);
                               }
                            }



                            // remove the entree from the pool
                            //vmap.setvar("blueengine.camera_q".to_owned(),&poolremove(&qbuffer,&i) );

                            vmap.setvar(i.to_string()+".lenght",&counter.to_string());
                        }
                    }
                    vmap.setstringarray("blueengine.textnode_q",Vec::new());//
                }
                let qbuffer = vmap.getstringarray("blueengine.textcolor_q");
                if qbuffer.len() != 0{
                    for i in qbuffer{
                        if i != ""{ // if queed items in pool
                            let key = i.to_string() + ".lenght";
                            let getlenght = nscript_usize(&vmap.getvar(&key));

                            let key = i.to_string() + ".color";
                            let getcolor = vmap.getvar(&key);
                            #[cfg(debug_assertions)]
                            println!("colorq lenght={} color={}",&getlenght,&getcolor);

                            for xchar in 1..getlenght{
                                // create the nscript variable
                                let charnode = i.to_string() + "_textchar_" + &xchar.to_string();
                                let data = split(&getcolor,",");
                                if data.len() > 3 {
                                    let color1 = match data[0].parse::<f32>(){
                                        Ok(r) => r,
                                        Err(e) => 0.0,
                                    };
                                    let color2 = match data[1].parse::<f32>(){
                                        Ok(r) => r,
                                        Err(e) => 0.0,
                                    };
                                    let color3 = match data[2].parse::<f32>(){
                                        Ok(r) => r,
                                        Err(e) => 0.0,
                                    };
                                    let color4 = match data[3].parse::<f32>(){
                                        Ok(r) => r,
                                        Err(e) => 0.0,
                                    };

                                    objects
                                        .get_mut(&charnode)
                                        .unwrap()
                                        .set_uniform_color(color1, color2, color3, color4)
                                        .unwrap();
                                }
                                // remove the entree from the pool
                                //vmap.setvar("blueengine.camera_q".to_owned(),&poolremove(&qbuffer,&i) );

                            }
                        }
                    }
                    vmap.setstringarray("blueengine.textcolor_q",Vec::new());
                }

                // text update quee, this changes the updated characters with new settings.
                let qbuffer = vmap.getstringarray("blueengine.textnodeupdate_q");
                if qbuffer.len() != 0{

                    for i in qbuffer{
                        if i != ""{ // if queed items in pool
                            let mut counter = 1;

                            let prop = i.to_string() + ".text";
                            let text = vmap.getvar(&prop);
                            let prop = i.to_string() + ".x";
                            let text_x = nscript_f32(&vmap.getvar(&prop));
                            let prop = i.to_string() + ".y";
                            let text_y = nscript_f32(&vmap.getvar(&prop));
                            let prop = i.to_string() + ".size";
                            let text_size = nscript_f32(&vmap.getvar(&prop));
                            let mut scalewidth = text_size / 20.0;
                            let mut scaleheight = text_size / 16.0;
                            let mut distancebetweenchars: f32;
                            distancebetweenchars = text_size / (1400.0 - (text_size * 21.0));

                            let prop = i.to_string() + ".font";
                            let usedfont = vmap.getvar(&prop);
                            let prop = i.to_string() + ".newfont";
                            let newfont = vmap.getvar(&prop);
                            let prop = i.to_string() + ".newx";
                            let newx = nscript_f32(&vmap.getvar(&prop));
                            let prop = i.to_string() + ".newy";
                            let newy = nscript_f32(&vmap.getvar(&prop));
                            let prop = i.to_string() + ".size";
                            let newsize = nscript_f32(&vmap.getvar(&prop));
                            let mut setposx = newx.clone();
                            let prop = i.to_string() + ".lenght";
                            let lenght = nscript_usize(&vmap.getvar(&prop));
                            let splitchars = split(&text,"");
                            let oldsize = splitchars.len();
                            let mut scalewidth = newsize / 20.0;
                            let mut scaleheight = newsize   / 16.0;
                            let mut distancebetweenchars: f32;
                            let mut rescalefromhere = false;
                            distancebetweenchars = newsize / (1400.0 - (newsize* 21.0));
                            for xchar in splitchars{
                                if xchar != "" {
                                    let charnode = i.to_string() + "_textchar_" + &counter.to_string();
                                    counter +=1;

                                    // check if the chrsize increased then add new nodes
                                    if lenght < counter{
                                        objects.new_object(
                                            charnode.clone(),
                                            vec![
                                                Vertex {
                                                    position: [1.0, 1.0, 0.0],
                                                    uv: [1.0, 0.0],
                                                    normal: [0f32, 0f32, 0f32],
                                                },
                                                Vertex {
                                                    position: [1.0, -1.0, 0.0],
                                                    uv: [1.0, 1.0],
                                                    normal: [0f32, 0f32, 0f32],
                                                },
                                                Vertex {
                                                    position: [-1.0, -1.0, 0.0],
                                                    uv: [0.0, 1.0],
                                                    normal: [0f32, 0f32, 0f32],
                                                },
                                                Vertex {
                                                    position: [-1.0, 1.0, 0.0],
                                                    uv: [0.0, 0.0],
                                                    normal: [0f32, 0f32, 0f32],
                                                },
                                            ],
                                            vec![2, 1, 0, 2, 0, 3],
                                            objectsettignslayer1,
                                            renderer
                                        );
                                        vmap.setvar(charnode.to_string()+".texture", "reset.");
                                        #[cfg(debug_assertions)]
                                        println!("new node added for  {} text {}",&i,counter);
                                    }
                                    let  mut layer1 = objects
                                        .get_mut(&charnode)
                                        .expect("failed to gete object");

                                    layer1.set_render_order(1).unwrap();
                                    layer1.camera_effect = false;

                                    // check for texture changes
                                    let textureprop = usedfont.to_string() + "." + &Bluenc::pngcharname(&xchar);
                                    let texture = vmap.getvar(&textureprop);
                                    let chrtextureprop = charnode.to_string()+".texture";
                                    let chrtexture = vmap.getvar(&chrtextureprop);
                                    if chrtexture != texture  || newfont != usedfont{
                                        rescalefromhere = true;
                                        objects
                                            .get_mut(&charnode)
                                            .expect("Error during copying texture of the main square")
                                            .reference_texture(texture.clone());
                                        vmap.setvar(charnode.to_string()+".texture",&texture);
                                        vmap.setvar(charnode.to_string()+".font",&newfont);
                                        #[cfg(debug_assertions)]
                                        println!("textnode {} char {} texture {}",&i,&charnode,&texture);

                                    }

                                    // check for changes on scale
                                    let key = i.to_string() + ".newsize";
                                    let newsize = nscript_f32(&vmap.getvar(&key));
                                    if newsize != text_size || rescalefromhere {


                                        objects
                                            .get_mut(&charnode)
                                            .unwrap()
                                            .resize(scalewidth, scaleheight, 0.0,window.inner_size());
                                        objects.update_object(&charnode[..], |object| {
                                        object.set_scale(scalewidth,scaleheight,0.0);
                                    });;
                                        vmap.setvar(i.to_string()+ ".size", &text_size.to_string());
                                    #[cfg(debug_assertions)]
                                        println!("size adjusted");
                                    //}

                                    // check if the postion has changed
                                    //if newx != text_x || newy != text_y{

                                        //setposx = newx.clone();
                                        vmap.setvar(i.to_string()+".x", &newx.to_string());
                                        vmap.setvar(i.to_string()+".y", &newy.to_string());
                                        objects
                                            .get_mut(&charnode)
                                            .unwrap()
                                            .set_position(setposx, newy, nscript_f32("0.0"));
                                        #[cfg(debug_assertions)]
                                        println!("textnode {} x {} nx {} y {} ny {}",&i,&text_x.to_string(),&newx.to_string(),&text_y.to_string(),&newy.to_string());
                                    }
                                    match xchar {// custom adjusting for characters
                                        "1" | "i" | " " | "." | "," => {
                                            setposx = setposx + (distancebetweenchars*0.6);
                                        }
                                        "f" | "t" =>{
                                            setposx = setposx + (distancebetweenchars*0.8);
                                        }

                                        "y" | "z" | "x" | "d" | "u" | "r" | "s" | "o" | "c" | "a" =>{
                                            setposx = setposx + (distancebetweenchars*1.05);
                                        }
                                        "w" | "\\" | "/" | "#" | "m" => {

                                            setposx = setposx + (distancebetweenchars*1.25);
                                        }
                                        _ =>{
                                            setposx = setposx + distancebetweenchars;
                                        }

                                    }


                                    //}

                                }
                            }


                            // wipe the remaining
                            let  mut counter2 = counter.clone();
                            if counter <= oldsize-1 {
                                for xspace in counter..oldsize{

                                    let charnode = i.to_string() + "_textchar_" + &counter2.to_string();
                                    objects.remove(&charnode);
                                    counter2 +=1;

                                }
                            }
                            vmap.setvar(i.to_string()+".lenght",&counter.to_string());
                        }
                    }
                    vmap.setstringarray("blueengine.textnodeupdate_q",Vec::new());
                }

                let qbuffer = vmap.getstringarray("blueengine.textdelete_q");
                if qbuffer.len() != 0{
                    for i in qbuffer{
                        if i != ""{ // if queed items in pool
                            let key = i.to_string() + ".lenght";
                            let getlenght = nscript_usize(&vmap.getvar(&key));
                            for xchar in 1..getlenght{
                                // create the nscript variable
                                let charnode = i.to_string() + "_textchar_" + &xchar.to_string();
                                    objects.remove(&charnode);

                                vmap.setvar(i.to_string() + ".lenght", "0");
                            }
                            vmap.setvar(key.to_string(), "0");
                        }
                    }
                    vmap.setstringarray("blueengine.textdelete_q",Vec::new());//
                }
            }
            //reset nscript property
        })
        .expect("Error during update loop");
}

