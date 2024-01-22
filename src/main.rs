

#![allow(unused)]
use blue_engine::{
    primitive_shapes::{cube, square, triangle, uv_sphere},
    uniform_type::Matrix,
    utils::default_resources::DEFAULT_MATRIX_4,
    Engine, Instance, ObjectSettings, PolygonMode, PowerPreference, RotateAxis, ShaderSettings,
    TextureData, Vertex, WindowDescriptor,
};
use blue_engine_egui::egui as gui;
mod includes{
    pub mod bluenc;
}
pub use includes::bluenc::*;
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
                100// default 100ms, however this should not be ok.
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
    //let mut engine = Engine::new().expect("win");
    let mut engine = Engine::new_config(WindowDescriptor { width: 1920, height: 1080, title: "Nscript&BlueEngine Testing",..Default::default()}).expect("win");


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


    let serverscriptfilename = NC_SCRIPT_DIR.to_owned() +"system/BENC.nc";
    nscript_execute_script(&serverscriptfilename,"","","","","","","","","",&mut vmap);


    let objectsettignslayer1 = ObjectSettings {
        camera_effect: false,
        shader_settings: ShaderSettings::default(),
    };


    // quee system to load textures , ( first nodes can be used to copy to others)
    let queetest = nscript_checkvar("blueengine.textureload_q", &mut vmap);
     //println!("DEBUG!!!!!!!!!!!!!{}",&queetest);
    //cwrite(&nscript_checkvar("blueengine.textureload_q", &mut vmap),"p");
    for i in split(&nscript_checkvar("blueengine.textureload_q", &mut vmap),NC_ARRAY_DELIM){//NC_ARRAY_DELIM
        let ckey = "blueengine_textures.".to_owned() + &Nstring::stringtoeval(&i);
        if nscript_checkvar(&ckey, &mut vmap) == "" && i != ""{
            //cwrite(&ckey,"green");
            vmap.setvar("tmp".to_owned(),i);
            if i != "" {
                //perform blue engine texture loading
                let t = engine
                    .renderer
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
    }
vmap.setvar("blueengine.textureload_q".to_owned(),"" );
    for i in split(&nscript_checkvar("blueengine.bmpfont_q", &mut vmap),NC_ARRAY_DELIM){//NC_ARRAY_DELIM
        //let ckey = "blueengine_textures.".to_owned() + &Nstring::stringtoeval(&i);
        if  i != ""{
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

                object.set_scale(0.5, 0.5, 1f32);
                object.set_translation(0.0,0.0,-10.0);

    });           //layer1.flag_as_changed();
            //println!("bmpfont {}",i);
        }
    }
vmap.setvar("blueengine.bmpfont_q".to_owned(),"" );

    //:w
    //spanwning quee
    for i in split(&nscript_checkvar("blueengine.square_q", &mut vmap),NC_ARRAY_DELIM){
        //cwrite(&i,"green");
        if i != "" {
        square(
                i,
               ObjectSettings::default(),
                &mut engine.renderer,
            &mut engine.objects,
        );
            engine.objects.update_object(i, |object| {
                object.set_render_order(1).unwrap();

                //object.set_scale(0.5, 0.5, 1f32);
                object.set_translation(0.0,0.0,-10.0);

    });
        }
    }
vmap.setvar("blueengine.square_q".to_owned(),"" );
    // set positions quee
    for i in split(&nscript_checkvar("blueengine.position_q", &mut vmap),NC_ARRAY_DELIM){
        //cwrite(&i,"green");
        if i != "" {
            let data = split(&i,",");
            if data.len() > 2 {
                //cwrite(&i,"yellow");
                engine
                    .objects
                    .get_mut(data[0])
                    .unwrap()
                    .set_position(data[1].parse().unwrap_or(0.0), data[2].parse().unwrap_or(0.0), data[3].parse().unwrap_or(0.0));

            }
            else{
                cwrite("A split error on the position quee accuired","red");
                cwrite(i,"red");

            }
        }
    }
vmap.setvar("blueengine.position_q".to_owned(),"" );

            // Bridge: Nscript setTexture q.
            let qbuffer = nscript_checkvar("blueengine.textureset_q", &mut vmap);
            for i in split(&qbuffer,NC_ARRAY_DELIM){
                if i != ""{ // if queed items in pool

                    let data = split(&i,",");
                    if data.len() > 0  && data[0] != ""{
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
vmap.setvar("blueengine.textureset_q".to_owned(),"" );


    cwrite(&nscript_checkvar("blueengine.squarequee", &mut vmap),"red");

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
        blue_engine::KeyCode::KeyZ,];
    let keyname = [ // keymapping naming ( must contain the same size and order as the keymapping!!)

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
let mut animationtimer= Ntimer::init();
    let mut uiselfname = String::new();

    let mut uiselfpath  = Some("".to_string());// String::new();
    let mut uiselfage = 1;
        // Start the egui context
    let gui_context =
        blue_engine_egui::EGUI::new(&engine.event_loop, &mut engine.renderer, &engine.window);

    // We add the gui as plugin, which runs once before everything else to fetch events, and once during render times for rendering and other stuff
    engine.plugins.push(Box::new(gui_context));
 let mut color = [1f32, 1f32, 1f32, 1f32];

    engine
        .update_loop(move |renderer, window, objects, input, camera, plugins| {
      let cube_vertices = vec![
        // Front face
        Vertex { position: [1.0, 1.0, 1.0], uv: [1.0, 0.0], normal: [0.0, 0.0, 1.0] },
        Vertex { position: [1.0, -1.0, 1.0], uv: [1.0, 1.0], normal: [0.0, 0.0, 1.0] },
        Vertex { position: [-1.0, -1.0, 1.0], uv: [0.0, 1.0], normal: [0.0, 0.0, 1.0] },
        Vertex { position: [-1.0, 1.0, 1.0], uv: [0.0, 0.0], normal: [0.0, 0.0, 1.0] },

        // Back face
        Vertex { position: [1.0, 1.0, -1.0], uv: [1.0, 0.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [1.0, -1.0, -1.0], uv: [1.0, 1.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [-1.0, -1.0, -1.0], uv: [0.0, 1.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [-1.0, 1.0, -1.0], uv: [0.0, 0.0], normal: [0.0, 0.0, -1.0] },

        // Top face
        Vertex { position: [1.0, 1.0, -1.0], uv: [1.0, 0.0], normal: [0.0, 1.0, 0.0] },
        Vertex { position: [1.0, 1.0, 1.0], uv: [1.0, 1.0], normal: [0.0, 1.0, 0.0] },
        Vertex { position: [-1.0, 1.0, 1.0], uv: [0.0, 1.0], normal: [0.0, 1.0, 0.0] },
        Vertex { position: [-1.0, 1.0, -1.0], uv: [0.0, 0.0], normal: [0.0, 1.0, 0.0] },

        // Bottom face
        Vertex { position: [1.0, -1.0, -1.0], uv: [1.0, 0.0], normal: [0.0, -1.0, 0.0] },
        Vertex { position: [1.0, -1.0, 1.0], uv: [1.0, 1.0], normal: [0.0, -1.0, 0.0] },
        Vertex { position: [-1.0, -1.0, 1.0], uv: [0.0, 1.0], normal: [0.0, -1.0, 0.0] },
        Vertex { position: [-1.0, -1.0, -1.0], uv: [0.0, 0.0], normal: [0.0, -1.0, 0.0] },

        // Left face
        Vertex { position: [-1.0, 1.0, 1.0], uv: [1.0, 0.0], normal: [-1.0, 0.0, 0.0] },
        Vertex { position: [-1.0, -1.0, 1.0], uv: [1.0, 1.0], normal: [-1.0, 0.0, 0.0] },
        Vertex { position: [-1.0, -1.0, -1.0], uv: [0.0, 1.0], normal: [-1.0, 0.0, 0.0] },
        Vertex { position: [-1.0, 1.0, -1.0], uv: [0.0, 0.0], normal: [-1.0, 0.0, 0.0] },

        // Right face
        Vertex { position: [1.0, 1.0, 1.0], uv: [1.0, 0.0], normal: [1.0, 0.0, 0.0] },
        Vertex { position: [1.0, -1.0, 1.0], uv: [1.0, 1.0], normal: [1.0, 0.0, 0.0] },
        Vertex { position: [1.0, -1.0, -1.0], uv: [0.0, 1.0], normal: [1.0, 0.0, 0.0] },
        Vertex { position: [1.0, 1.0, -1.0], uv: [0.0, 0.0], normal: [1.0, 0.0, 0.0] },
    ];
        // Define indices for the cube
    let cube_indices = vec![
        // Front face
        0, 1, 2, 0, 2, 3,
        // Back face
        4, 5, 6, 4, 6, 7,
        // Top face
        8, 9, 10, 8, 10, 11,
        // Bottom face
        12, 13, 14, 12, 14, 15,
        // Left face
        16, 17, 18, 16, 18, 19,
        // Right face
        20, 21, 22, 20, 22, 23,
    ];
            let egui_plugin = plugins[0]
                // downcast it to obtain the plugin
                .downcast_mut::<blue_engine_egui::EGUI>()
                .expect("Plugin not found");

            // ui function will provide the context
            let eachmenu  = nscript_checkvar("guiactivemenus",&mut vmap);


            egui_plugin.ui(
                |ctx| {
                    for eachmenu in split(&nscript_checkvar("guiactivemenus",&mut vmap),"|"){
                        if eachmenu != ""{
                            //et eachmenu = nscript_checkvar("gui.activemenu",&mut vmap);
                            let titleref = "".to_owned() + &eachmenu.trim() + ".title";
                            let title ="nc ".to_owned()+ &nscript_checkvar(&titleref,&mut vmap);
                            //println!("this menu{} title:{}",eachmenu,title);
                            gui::Window::new(&title).show(ctx, |ui| {
                                let controllarr = arraysearch(&Nstring::replace(&vmap.inobj(&eachmenu),"|",NC_ARRAY_DELIM),"type_");
                                // let fpsmsg = "fps:".to_owned() + &vmap.getvar("fps") ;
                                // ui.label(&fpsmsg);

                                //println!("inobj:{}",controllarr);
                                for eachcontrol in split(&controllarr,NC_ARRAY_DELIM){
                                    if eachcontrol != ""{
                                        let eachcontroltype = "".to_owned() + &eachmenu + "." + eachcontrol;
                                        let thistype = nscript_checkvar(&eachcontroltype,&mut vmap);
                                        match thistype.as_str(){
                                            "link" => {
                                                let mut inputvarlabelname = nscript_checkvar(&Nstring::replace(&eachcontroltype, "type_", "control_"),&mut vmap);//evaluate the variablename
                                                //let mut originalvarname = nscript_checkvar(&originalvarname,&mut vmap);// get actualvalue!
                                                let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                let mut originalvarname = nscript_checkvar(&inputvarname,&mut vmap);//evaluate the variablename
                                                //let mut inputvar = nscript_checkvar(&originalvarname,&mut vmap);// get actualvalue!

                                                ui.hyperlink_to(inputvarlabelname, originalvarname);
                                            }
                                            "checkbox" =>{
                                                let mut inputvarlabelname = nscript_checkvar(&Nstring::replace(&eachcontroltype, "type_", "control_"),&mut vmap);//evaluate the variablename
                                                //let mut originalvarname = nscript_checkvar(&originalvarname,&mut vmap);// get actualvalue!
                                                let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                let mut originalvarname = nscript_checkvar(&inputvarname,&mut vmap);//evaluate the variablename
                                                let mut inputvar = nscript_checkvar(&originalvarname,&mut vmap);// get actualvalue!
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
                                                let mut originalvarname = nscript_checkvar(&inputvarname,&mut vmap);//evaluate the variablename
                                                let mut inputvar = nscript_checkvar(&originalvarname,&mut vmap);// get actualvalue!
                                                let mut selected  = inputvar.clone();

                                                let mut inputname = nscript_checkvar(&Nstring::replace(&eachcontroltype, "type_", "control_"),&mut vmap);

                                                let arrayitems = nscript_checkvar(&Nstring::replace(&eachcontroltype, "type_", "func_"),&mut vmap);
                                                let selidvarname = Nstring::replace(&eachcontroltype, "type_", "selected_");
                                                let selidval = nscript_checkvar(&selidvarname,&mut vmap);
                                                let selid = nscript_checkvar(&selidval,&mut vmap);
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
                                                let mut sel2 = selected.clone();
                                                let mut i = 0;
                                                ui.label(&inputname);
                                                for x in alternatives.clone(){

                                                    if i != selected {

                                                        if ui.radio(false,x).clicked(){
                                                            sel2 = i;
                                                        };
                                                    }else{
                                                        if ui.radio(true,alternatives[selected]).clicked(){
                                                            sel2 = i;
                                                        };
                                                        ;

                                                    }
                                                    i +=1;
                                                }

                                                if alternatives[selected] != alternatives[sel2]{
                                                    vmap.setvar(originalvarname,&alternatives[sel2]);

                                                    vmap.setvar(selidvarname,&sel2.to_string());
                                                    //println!("nput:{}",inputvar);
                                                }
                                            }
                                            "combo" => {
                                                let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                let mut originalvarname = nscript_checkvar(&inputvarname,&mut vmap);//evaluate the variablename
                                                let mut inputvar = nscript_checkvar(&originalvarname,&mut vmap);// get actualvalue!
                                                let mut selected  = inputvar.clone();

                                                let mut inputname = nscript_checkvar(&Nstring::replace(&eachcontroltype, "type_", "control_"),&mut vmap);

                                                let arrayitems = nscript_checkvar(&Nstring::replace(&eachcontroltype, "type_", "func_"),&mut vmap);
                                                let selidvarname = Nstring::replace(&eachcontroltype, "type_", "selected_");
                                                let selid = nscript_checkvar(&selidvarname,&mut vmap);
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
                                                let mut originalvarname = nscript_checkvar(&inputvarname,&mut vmap);//evaluate the variablename
                                                let mut inputvar = nscript_checkvar(&originalvarname,&mut vmap);// get actualvalue!
                                                let mut changedvar = inputvar.clone();

                                                let mut inputname = nscript_checkvar(&Nstring::replace(&eachcontroltype, "type_", "control_"),&mut vmap);


                                                password(&mut changedvar);
                                                //println!("input name:{}",inputname);

                                            }
                                            "input" =>{
                                                let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                let mut originalvarname = nscript_checkvar(&inputvarname,&mut vmap);//evaluate the variablename
                                                let mut inputvar = nscript_checkvar(&originalvarname,&mut vmap);// get actualvalue!
                                                let mut changedvar = inputvar.clone();

                                                let mut inputname = nscript_checkvar(&Nstring::replace(&eachcontroltype, "type_", "control_"),&mut vmap);


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
                                                let mut originalvarname = nscript_checkvar(&inputvarname,&mut vmap);//evaluate the variablename
                                                let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                let mut inputvar = nscript_checkvar(&inputvarname,&mut vmap);//evaluate the variablename
                                                let mut inputvar = nscript_checkvar(&inputvar,&mut vmap);
                                                ui.label("".to_owned()+&originalvarname+ &inputvar);

                                            }
                                            "button" =>{


                                                let mut inputname = nscript_checkvar(&Nstring::replace(&eachcontroltype, "type_", "control_"),&mut vmap);

                                                if ui.button(&inputname).clicked() {
                                                    let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                    let mut inputvar = nscript_checkvar(&inputvarname,&mut vmap);//evaluate the variablename
                                                    let mut inputvar = nscript_parseline(&nscript_formatsheet(&inputvar),&mut vmap);// get actualvalue!

                                                }
                                            }
                                            "file" =>{


                                                let mut inputname = nscript_checkvar(&Nstring::replace(&eachcontroltype, "type_", "control_"),&mut vmap);

                                                if ui.button(&inputname).clicked() {
                                                    if let Some(path) = rfd::FileDialog::new().pick_file() {

                                                        let inputvarname = Nstring::replace(&eachcontroltype, "type_", "var_");
                                                        let mut inputvar = nscript_checkvar(&inputvarname,&mut vmap);//evaluate the variablename
                                                        let mut inputvar = nscript_checkvar(&inputvar,&mut vmap);// get actualvalue!
                                                        vmap.setvar(inputvarname,&format!("{:?}",path));
                                                        let inputvarname = Nstring::replace(&eachcontroltype, "type_", "func_");
                                                        let mut fnvar = nscript_checkvar(&inputvarname,&mut vmap);//evaluate the variablename
                                                        //let mut fnvar = nscript_checkvar(&fnvar,&mut vmap);// get actualvalue!
                                                        let nscriptfunctiontorun = "".to_owned() + &fnvar + "(" + &format!("{:?}",path) +")";
                                                        //println!("file - >{}",nscriptfunctiontorun);
                                                        nscript_func(&nscriptfunctiontorun,&mut vmap);

                                                    }

                                                }
                                            }
                                            "slider" =>{
                                                let labelname = nscript_checkvar(&eachcontroltype,&mut vmap);
                                                let inputvarname =  Nstring::replace(&eachcontroltype, "type_", "var_");
                                                let inputmin = match nscript_checkvar(&Nstring::replace(&eachcontroltype, "type_", "min_"),&mut vmap).parse::<u32>(){
                                                    Ok(res) => {
                                                        res
                                                    }
                                                    Err(_) =>{
                                                        0
                                                    }
                                                };

                                                let inputmax = match nscript_checkvar(&Nstring::replace(&eachcontroltype, "type_", "max_"),&mut vmap).parse::<u32>(){
                                                    Ok(res) => {
                                                        res
                                                    }
                                                    Err(_) =>{
                                                        0
                                                    }
                                                };
                                                let mut inputvarnameori = nscript_checkvar(&inputvarname,&mut vmap);//evaluate the variablename
                                                let mut inputvar = match nscript_checkvar(&inputvarnameori,&mut vmap).parse::<u32>(){
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
                                                let mut originalvarname = nscript_checkvar(&inputvarname,&mut vmap);//evaluate the variablename
                                                let mut inputvar = nscript_checkvar(&originalvarname,&mut vmap);// get actualvalue!
                                                let mut changedvar = inputvar.clone();

                                                let mut inputname = nscript_checkvar(&Nstring::replace(&eachcontroltype, "type_", "control_"),&mut vmap);


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
            nscript_loops(&mut vmap);
            vmap.setvar("key.event".to_owned(),"false");
            for i in split(&nscript_checkvar("blueengine.bmpfont_q", &mut vmap),NC_ARRAY_DELIM){//NC_ARRAY_DELIM
                //let ckey = "blueengine_textures.".to_owned() + &Nstring::stringtoeval(&i);
                if  i != ""{

                    objects.new_object(
                        i,
                        vec![
                            // Front face
        Vertex { position: [1.0, 1.0, 1.0], uv: [1.0, 0.0], normal: [0.0, 0.0, 1.0] },
        Vertex { position: [1.0, -1.0, 1.0], uv: [1.0, 1.0], normal: [0.0, 0.0, 1.0] },
        Vertex { position: [-1.0, -1.0, 1.0], uv: [0.0, 1.0], normal: [0.0, 0.0, 1.0] },
        Vertex { position: [-1.0, 1.0, 1.0], uv: [0.0, 0.0], normal: [0.0, 0.0, 1.0] },

        // Back face
        Vertex { position: [1.0, 1.0, -1.0], uv: [1.0, 0.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [1.0, -1.0, -1.0], uv: [1.0, 1.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [-1.0, -1.0, -1.0], uv: [0.0, 1.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [-1.0, 1.0, -1.0], uv: [0.0, 0.0], normal: [0.0, 0.0, -1.0] },

        // Top face
        Vertex { position: [1.0, 1.0, -1.0], uv: [1.0, 0.0], normal: [0.0, 1.0, 0.0] },
        Vertex { position: [1.0, 1.0, 1.0], uv: [1.0, 1.0], normal: [0.0, 1.0, 0.0] },
        Vertex { position: [-1.0, 1.0, 1.0], uv: [0.0, 1.0], normal: [0.0, 1.0, 0.0] },
        Vertex { position: [-1.0, 1.0, -1.0], uv: [0.0, 0.0], normal: [0.0, 1.0, 0.0] },

        // Bottom face
        Vertex { position: [1.0, -1.0, -1.0], uv: [1.0, 0.0], normal: [0.0, -1.0, 0.0] },
        Vertex { position: [1.0, -1.0, 1.0], uv: [1.0, 1.0], normal: [0.0, -1.0, 0.0] },
        Vertex { position: [-1.0, -1.0, 1.0], uv: [0.0, 1.0], normal: [0.0, -1.0, 0.0] },
        Vertex { position: [-1.0, -1.0, -1.0], uv: [0.0, 0.0], normal: [0.0, -1.0, 0.0] },

        // Left face
        Vertex { position: [-1.0, 1.0, 1.0], uv: [1.0, 0.0], normal: [-1.0, 0.0, 0.0] },
        Vertex { position: [-1.0, -1.0, 1.0], uv: [1.0, 1.0], normal: [-1.0, 0.0, 0.0] },
        Vertex { position: [-1.0, -1.0, -1.0], uv: [0.0, 1.0], normal: [-1.0, 0.0, 0.0] },
        Vertex { position: [-1.0, 1.0, -1.0], uv: [0.0, 0.0], normal: [-1.0, 0.0, 0.0] },

        // Right face
        Vertex { position: [1.0, 1.0, 1.0], uv: [1.0, 0.0], normal: [1.0, 0.0, 0.0] },
        Vertex { position: [1.0, -1.0, 1.0], uv: [1.0, 1.0], normal: [1.0, 0.0, 0.0] },
        Vertex { position: [1.0, -1.0, -1.0], uv: [0.0, 1.0], normal: [1.0, 0.0, 0.0] },
        Vertex { position: [1.0, 1.0, -1.0], uv: [0.0, 0.0], normal: [1.0, 0.0, 0.0] },
    ],

                        vec![
        // Front face
        0, 1, 2, 0, 2, 3,
        // Back face
        4, 5, 6, 4, 6, 7,
        // Top face
        8, 9, 10, 8, 10, 11,
        // Bottom face
        12, 13, 14, 12, 14, 15,
        // Left face
        16, 17, 18, 16, 18, 19,
        // Right face
        20, 21, 22, 20, 22, 23,
    ],
                        ObjectSettings::default(),
                        renderer
                    );
                    let layer1 = objects
                        .get_mut(i)
                        .expect("failed to gete object");

                    layer1.set_render_order(1).unwrap();
                    //layer1.camera_effect = false;
                    layer1.flag_as_changed();
                    //println!("sqaure {}",i);
                }
            }
vmap.setvar("blueengine.bmpfont_q".to_owned(),"" );
            let qbuffer = nscript_checkvar("blueengine.bmpfonttextureset_q", &mut vmap);
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
vmap.setvar("blueengine.bmpfonttextureset_q".to_owned(),"" );

            for i in split(&nscript_checkvar("blueengine.square_q", &mut vmap),NC_ARRAY_DELIM){//NC_ARRAY_DELIM
                //let ckey = "blueengine_textures.".to_owned() + &Nstring::stringtoeval(&i);
                if  i != ""{
                    objects.new_object(
                        i,
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
                            .get_mut(i)
                            .expect("Error getting ref");
                    //thisobj.set_position(0.0,0.0,-5.0);
                    //println!("sqaure {}",i);
                }
            }
vmap.setvar("blueengine.square_q".to_owned(),"" );


            // Bridge: Nscript setTexture q.
            let qbuffer = nscript_checkvar("blueengine.textureset_q", &mut vmap);
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
            vmap.setvar("blueengine.textureset_q".to_owned(),"" );


            // Bridge: Nscript Scale Quee handler.
            // in nscript class blueengine.roation_q is used by func blueengine.setscale()
            let qbuffer = nscript_checkvar("blueengine.scale_q", &mut vmap);
            for i in split(&qbuffer,NC_ARRAY_DELIM){
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
            vmap.setvar("blueengine.scale_q".to_owned(),"" );

            let qbuffer = nscript_checkvar("blueengine.color_q", &mut vmap);
            for i in split(&qbuffer,NC_ARRAY_DELIM){
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
            vmap.setvar("blueengine.color_q".to_owned(),"" );
            // Bridge: Nscript scale Quee handler.
            // in nscript class blueengine.position_q is used by func blueengine.setposition()

            let qbuffer = nscript_checkvar("blueengine.position_q", &mut vmap);
            for i in split(&qbuffer,NC_ARRAY_DELIM){
                if i != ""{ // if queed items in pool

                    let data = split(&i,",");
                    if data.len() > 3 && data[0] != ""{// check obj,x,y,z
                        // let thisobj = objects
                        //     .get_mut(data[0])
                        //     .unwrap()
                        //     .set_position(data[1].parse().unwrap_or(0.0), data[2].parse().unwrap_or(0.0), data[3].parse().unwrap_or(0.0));
                        objects.update_object(data[0], |object| {

                            object.set_position(data[1].parse().unwrap_or(0.0), data[2].parse().unwrap_or(0.0), data[3].parse().unwrap_or(0.0))

                        });;
                    }
                    else{
                        cwrite("Split error on the position quee","red")
                    }
                    // remove the entree from the pool ( in nscript blueengine.position_quee )
                    //vmap.setvar("blueengine.position_q".to_owned(),&poolremove(&qbuffer,&i) );

                }
            }
            vmap.setvar("blueengine.position_q".to_owned(),"");

            // Bridge: Nscript rotation Quee handler.
            // in nscript class blueengine.roation_q is used by func blueengine.setrotatation()
            let qbuffer = nscript_checkvar("blueengine.rotation_q", &mut vmap);
            for i in split(&qbuffer,NC_ARRAY_DELIM){
                if i != ""{ // if queed items in pool
                    //println!("debug:{}",i);
                    let data = split(&i,",");
                    if data.len() > 2 {// check obj,x,y,z
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
                            _ => RotateAxis::X
                        };
                        objects
                            .get_mut(data[0])
                            .unwrap()
                            .set_rotatation(data[1].parse().unwrap_or(0.0), axis);

                    }
                    else{
                        cwrite("Split error on the rotation quee ","red")
                    }
                    // remove the entree from the pool ( in nscript blueengine.position_quee )
                    //vmap.setvar("blueengine.rotation_q".to_owned(),&poolremove(&qbuffer,&i) );//clearpool

                }
            }
            vmap.setvar("blueengine.rotation_q".to_owned(),"");

            let qbuffer = nscript_checkvar("blueengine.camera_q", &mut vmap);
            for i in split(&qbuffer,NC_ARRAY_DELIM){
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
            vmap.setvar("blueengine.camera_q".to_owned(),"");//clearpool


            //
            let qbuffer = nscript_checkvar("blueengine.deletion_q", &mut vmap);
            for i in split(&qbuffer,NC_ARRAY_DELIM){
                if i != ""{ // if queed items in pool

                    objects.remove(i);

                    // remove the entree from the pool
                    //vmap.setvar("blueengine.camera_q".to_owned(),&poolremove(&qbuffer,&i) );

                }
            }
            vmap.setvar("blueengine.deletion_q".to_owned(),"");//clearpool

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

            let qbuffer = nscript_checkvar("blueengine.anim_q", &mut vmap);
            //println!("animq:{}",qbuffer);
            for sprite in split(&qbuffer,NC_ARRAY_DELIM) {
                if sprite != "" {
                    //println!("full: {}",sprite);
                    let spritesplit:Vec<String> = sprite.split("|").map(String::from).collect();
                    //println!("array: {:?}",spritesplit);
                    if spritesplit.len() > 1{
                        animationsystem.set(&spritesplit[0],&spritesplit);
                    }
                }

            }
            vmap.setvar("blueengine.anim_q".to_owned(),"");//clearpool

            if Ntimer::diff(animationtimer) > 100 {
                                for sprite in split(&vmap.getvar("animationhandler.allsprites"),NC_ARRAY_DELIM){
                    let thisanim = animationsystem.frame(&sprite);
                    if thisanim != ""{
                        objects
                            .get_mut(sprite)
                            .expect("Error during copying texture of the main square")
                            .reference_texture(thisanim);

                    }
                }
                animationtimer = Ntimer::init();
            }

            //reset nscript property
        })
        .expect("Error during update loop");
}






