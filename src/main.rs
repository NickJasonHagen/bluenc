#![allow(unused)]

use blue_engine::{
    primitive_shapes::{cube, square, triangle, uv_sphere},
    uniform_type::Matrix,
    utils::default_resources::DEFAULT_MATRIX_4,
    Engine, Instance, ObjectSettings, PolygonMode, PowerPreference, RotateAxis, ShaderSettings,
    TextureData, Vertex, WindowDescriptor,
};

//use blue_engine_utilities::FlyCamera;
//use nscript_v2::*;
//extern crate nscript_v2;
use std::collections::{HashMap};
//use std::{env, array, string};
extern crate nscript;
use nscript::*;
fn main() {
  cwrite(" started!!","g");
    let mut vmap = Varmap::new(); // global
    let mut engine = Engine::new().expect("win");
    cwrite("engine started!!","g");
    //let mut engine = ENGINE.lock().unwrap();
    //let test_instance = Instance::default();
    //println!("{:?}", test_instance.to_raw());
let mut texture_map: HashMap<String, blue_engine::wgpu::BindGroup> = HashMap::new();

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
            println!("bmpfont {}",i);
        }
    }
vmap.setvar("blueengine.bmpfont_q".to_owned(),"" );

    // spanwning quee
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

                object.set_scale(0.5, 0.5, 1f32);
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
                cwrite("A split error on the position quee accuired","red")

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
                cwrite(&i,"m");
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

    engine
        .update_loop(move |renderer, window, objects, input, camera, plugins| {

            // run all nscript loops , (quees be filled here and bridged to the render engine)
            nscript_loops(&mut vmap);
            vmap.setvar("key.event".to_owned(),"false");
            for i in split(&nscript_checkvar("blueengine.bmpfont_q", &mut vmap),NC_ARRAY_DELIM){//NC_ARRAY_DELIM
                //let ckey = "blueengine_textures.".to_owned() + &Nstring::stringtoeval(&i);
                if  i != ""{

                        objects.new_object(
                            i.clone(),
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
                        ObjectSettings {
                            camera_effect: false,
                            ..Default::default()
                        },
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
                        i.clone(),
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
                            .expect("Error getting ref")
                            ;
                    thisobj.set_position(0.0,0.0,-5.0);
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

                            object.set_translation(data[1].parse().unwrap_or(0.0), data[2].parse().unwrap_or(0.0), data[3].parse().unwrap_or(0.0))

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
                        objects.update_object(data[0], |object| {
                            object.set_render_order(1).unwrap();

                            object.set_scale(5.5, 0.5, 1f32);
                            object.set_position(0.0,0.0,-10.0);

                        });;
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
                        camera
                            .set_target(data[0].parse().unwrap_or(0.0), data[1].parse().unwrap_or(0.0), data[2].parse().unwrap_or(0.0) -  30.0)
                            .expect("Couldn't update the camera eye");
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
                    cwrite(&keyname[idx],"y")
                }
                else{
                    vmap.setvar(keyname[idx].to_owned(),"up");
                }
                idx +=1;
            }


            //reset nscript property
        })
        .expect("Error during update loop");
}






