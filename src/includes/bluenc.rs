use crate::*;
use nalgebra::{Matrix3, Point3, Rotation3,Vector3,Unit};
use nalgebra as na;

pub fn castraytoarray(fromx: &f64,fromy:&f64,fromz:&f64,tox:&f64,toy:&f64,speed:&f64){
//let mut Vec3
}

#[allow(clippy::ptr_arg)] // false positive
pub fn password_ui(ui: &mut gui::Ui, password: &mut String) -> gui::Response {
    // This widget has its own state — show or hide password characters (`show_plaintext`).
    // In this case we use a simple `bool`, but you can also declare your own type.
    // It must implement at least `Clone` and be `'static`.
    // If you use the `persistence` feature, it also must implement `serde::{Deserialize, Serialize}`.

    // Generate an id for the state
    let state_id = ui.id().with("show_plaintext");

    // Get state for this widget.
    // You should get state by value, not by reference to avoid borrowing of [`Memory`].
    let mut show_plaintext = ui.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

    // Process ui, change a local copy of the state
    // We want TextEdit to fill entire space, and have button after that, so in that case we can
    // change direction to right_to_left.
    let result = ui.with_layout(gui::Layout::right_to_left(gui::Align::Center), |ui| {
        // Toggle the `show_plaintext` bool with a button:
        let response = ui
            .add(gui::SelectableLabel::new(show_plaintext, "👁"))
            .on_hover_text("Show/hide password");

        if response.clicked() {
            show_plaintext = !show_plaintext;
        }

        // Show the password field:
        ui.add_sized(
            ui.available_size(),
            gui::TextEdit::singleline(password).password(!show_plaintext),
        );
    });

    // Store the (possibly changed) state:
    ui.data_mut(|d| d.insert_temp(state_id, show_plaintext));

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, …) and maybe show a tooltip:
    result.response
}

// A wrapper that allows the more idiomatic usage pattern: `ui.add(…)`
/// Password entry field with ability to toggle character hiding.
///
/// ## Example:
/// ``` ignore
/// ui.add(password(&mut my_password));
/// ```
pub fn password(password: &mut String) -> impl gui::Widget + '_ {
    move |ui: &mut gui::Ui| password_ui(ui, password)
}

fn adjust_rotation(current_rotations: Vector3<f64>, adjustment: f64, axis_of_rotation: Vector3<f64>) -> Vector3<f64> {
    // Apply the adjustment to the corresponding axis of rotation
    let mut adjusted_rotations = current_rotations;
    if axis_of_rotation.x != 0.0 {
        adjusted_rotations.x += adjustment;
    }
    if axis_of_rotation.y != 0.0 {
        adjusted_rotations.y += adjustment;
    }
    if axis_of_rotation.z != 0.0 {
        adjusted_rotations.z += adjustment;
    }
    adjusted_rotations
}
fn rotate_around_point(
    rotvec_deg: na::Vector3<f64>,
    center_point: na::Vector3<f64>,
    position: na::Vector3<f64>,
    axis: na::Vector3<f64>,
    roll_adjustment: f64,
) -> (na::Vector3<f64>, na::Vector3<f64>) {
    // Convert degrees to radians
    let rotvec_rad = rotvec_deg.map(|deg| deg.to_radians());
    let roll_adjustment_rad = roll_adjustment.to_radians();

    // Calculate position relative to the center point
    let relative_position = position - center_point;

    // Normalize axis
    let axis_unit = na::Unit::new_normalize(axis);

    // Apply roll adjustment
    let roll_rotation = na::Rotation3::from_axis_angle(&axis_unit, roll_adjustment_rad);
    let relative_position = roll_rotation * relative_position;

    // Apply rotations
    let mut current_relative_position = relative_position;
    for (i, rotation_rad) in rotvec_rad.iter().enumerate() {
        let rotation_axis = match i {
            0 => na::Vector3::x_axis(),
            1 => na::Vector3::y_axis(),
            2 => na::Vector3::z_axis(),
            _ => panic!("Invalid axis index"),
        };
        let rotation = na::Rotation3::from_axis_angle(&rotation_axis, *rotation_rad);
        current_relative_position = rotation * current_relative_position;
    }

    // Transform back to absolute position
    let new_position = current_relative_position + center_point;

    (new_position, current_relative_position)
}

pub fn testrotate(objectname:&str,units:&f64,axis:&str,vmap: &mut Varmap){
    let mut points: Vec<Point3<f64>> = Vec::new();
    let objectparts = objectname.to_owned() + ".parts";
    let partdata = nscript_checkvar(&objectparts, vmap);//Nfile::read(modelfile);
    let prop = objectname.to_owned() + ".x";
    let cposx = match vmap.getvar(&prop).parse::<f64>(){
        Ok(res) =>{
            res
        }
        Err(err) => {
            0.0
        }
    };
    let prop = objectname.to_owned() + ".y";
    let cposy = match vmap.getvar(&prop).parse::<f64>(){
        Ok(res) =>{
            res
        }
        Err(err) => {
            0.0
        }
    };

    let prop = objectname.to_owned() + ".z";
    let cposz = match vmap.getvar(&prop).parse::<f64>(){
        Ok(res) =>{
            res
        }
        Err(err) => {
            0.0
        }
    };
    let meshhasrotatedprop = objectname.to_owned() + ".hr" + axis;
    let meshhasrotated = vmap.getvar(&meshhasrotatedprop).parse::<f64>().unwrap_or(0.0);

    let meshhasrotatedprop = objectname.to_owned() + ".rx" + axis;
    let meshhasrotatedx = vmap.getvar(&meshhasrotatedprop).parse::<f64>().unwrap_or(0.0);
    let meshhasrotatedprop = objectname.to_owned() + ".ry" + axis;
    let meshhasrotatedy = vmap.getvar(&meshhasrotatedprop).parse::<f64>().unwrap_or(0.0);
    let meshhasrotatedprop = objectname.to_owned() + ".rz" + axis;
    let meshhasrotatedz = vmap.getvar(&meshhasrotatedprop).parse::<f64>().unwrap_or(0.0);

    let centerpoint = na::Vector3::new(cposx,cposy,cposz );
    let splitdata: Vec<&str> = partdata.split(NC_ARRAY_DELIM).collect();
    for x in splitdata{
        if x  != ""{
            // retract the map position back to 0,0,0 for proper transform.
            let prop = x.to_owned() + ".x";
            let posx = match vmap.getvar(&prop).parse::<f64>(){
                Ok(res) =>{
                    res - cposx
                }
                Err(err) => {
                    0.0
                }
            };
            let prop = x.to_owned() + ".y";
            let posy = match vmap.getvar(&prop).parse::<f64>(){
                Ok(res) =>{
                    res - cposy
                }
                Err(err) => {
                    0.0
                }
            };

            let prop = x.to_owned() + ".z";
            let posz = match vmap.getvar(&prop).parse::<f64>(){
                Ok(res) =>{
                    res - cposz
                }
                Err(err) => {
                    0.0
                }
            };

            let mut quad = na::Vector3::new(posx,posy,posz);

            let axisv = match axis{
                "x" => {
                    na::Vector3::x_axis()

                }
                "y" => {
                    na::Vector3::y_axis()

                }
                "z" | _ =>{
                    na::Vector3::z_axis()

                }

            };
            let setunits = 0.0 - meshhasrotated + units.to_radians();

            //let mut updateq = vmap.getvar("blueengine.rotation_q");

            //rotate it back to 0

            let prop = x.to_owned()+".rx";
            let tosetx = vmap.getvar(&prop);

            let prop = x.to_owned()+".ry";
            let tosety = vmap.getvar(&prop);

            let prop = x.to_owned()+".rz";
            let tosetz = vmap.getvar(&prop);


            let rotvec: na::Unit<na::Vector3<f64>> = match axis {
                "x" => na::Vector3::x_axis(),
                "y" => na::Vector3::y_axis(),
                "z" | _ => na::Vector3::z_axis(),
            };

            let (new_position,newrotations) =       rotate_around_point(rotvec.into_inner(), centerpoint, quad, axisv.into_inner(), units.to_owned());

            vmap.setvar(meshhasrotatedprop.clone(),&setunits.to_string());
            //points.push(Point3::new(posx,posy,posz));
            let prop = x.to_owned() + ".x";
            vmap.setvar(prop,&new_position[0].to_string());
            let prop = x.to_owned() + ".y";
            vmap.setvar(prop,&new_position[1].to_string());
            let prop = x.to_owned() + ".z";
            vmap.setvar(prop,&new_position[2].to_string());


            //let mut updateq = vmap.getvar("blueengine.update_q");
            //updateq = pooladd(&updateq, x);
            vmap.pushstringarray("blueengine.update_q",&x);
            let mut updateq = vmap.getvar("blueengine.rotation_q");

            let setunits = newrotations[0].to_degrees();
            //let addunits = units.to_radians()+ tosetx.parse::<f64>().unwrap_or(0.0);
            let set = x.to_owned() + "," + &setunits.to_string() + ",x," ;
            updateq = pooladd(&updateq, &set);
            let prop = x.to_owned() + ".rx";
            vmap.setvar(prop,&setunits.to_string());

            let setunits = newrotations[1].to_degrees();

            let set = x.to_owned() + "," + &setunits.to_string() + ",y," ;
            updateq = pooladd(&updateq, &set);
            let prop = x.to_owned() + ".ry";
            vmap.setvar(prop,&setunits.to_string());

            let setunits = newrotations[2].to_degrees();
            //let addunits = units.to_radians() + tosetz.parse::<f64>().unwrap_or(0.0);
            let set = x.to_owned() + "," + &setunits.to_string() + ",z," ;
            //updateq = pooladd(&updateq, &set);
            let prop = x.to_owned() + ".rz";
            vmap.setvar(prop,&setunits.to_string());


            vmap.pushstringarray("blueengine.rotation_q",&set);

        }
    }


    }
    fn rotate_quad(points: &mut Point3<f64>, axis: Vector3<f64>,angle: &f64) {
     //angle %= 2.0 * std::f64::consts::PI;
        let rotation_axis = Unit::new_normalize(axis);
        let rotation_matrix = Rotation3::from_axis_angle(&rotation_axis, angle.to_radians());

            *points = rotation_matrix.transform_point(points);

    }

    fn rotate_quads(points: &mut [Point3<f64>], axis: Vector3<f64>,mut angle: f64) {
     angle %= 2.0 * std::f64::consts::PI;
        let rotation_axis = Unit::new_normalize(axis);
        let rotation_matrix = Rotation3::from_axis_angle(&rotation_axis, angle);
        for point in points {
            *point = rotation_matrix.transform_point(point);
        }
    }
    fn createpoints(objectname: &str,vmap: &mut Varmap){
        let mut quads = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ];
    }


    pub fn nscript_bluenc_functionbindings(vmap: &mut Varmap)-> String{

    let funcname = vmap.funcname.as_str();
    let param1 = vmap.param1.clone();
    let param2 = vmap.param2.clone();
    let param3 = vmap.param3.clone();
    let param4 = vmap.param4.clone();
    let param5 = vmap.param5.clone();
    let param6 = vmap.param6.clone();
    let param7 = vmap.param7.clone();
    let param8 = vmap.param8.clone();
    let param9 = vmap.param9.clone();

    match funcname {
        "spriteload" => {
            return Bluenc::spriteload(&param1, &param2, vmap);
        }
        "spritedelete" => {
            Bluenc::spritedelete(&param1, vmap);
}
        "spritesetanimation" =>{
             Bluenc::spritesetanimation(&param1, &param2, vmap);
        }
        "spritegetanimations" => {
            return Bluenc::spritegetanimations(&param1,vmap);
        }
        "nodespawnsquare" => {
            return  Bluenc::nodespawnsquare(&param1, &nscript_f64(&param2), &nscript_f64(&param3), &nscript_f64(&param4), vmap);

        }
        "nodesetposition" => {
            Bluenc::nodesetposition(&param1, &nscript_f64(&param2),&nscript_f64(&param3),&nscript_f64(&param4),vmap);
        }
        "nodesetrotation" => {
            Bluenc::nodesetrotation(&param1, &nscript_f64(&param2),&nscript_f64(&param3),&nscript_f64(&param4),vmap);
        }
        "nodesetscale" => {
            Bluenc::nodesetscale(&param1, &nscript_f64(&param2),&nscript_f64(&param3),&nscript_f64(&param4),vmap);
        }
        "camerasetposition" => {
            Bluenc::camerasetposition(&nscript_f64(&param1), &nscript_f64(&param2), &nscript_f64(&param3), &nscript_f64(&param4), &nscript_f64(&param5), &nscript_f64(&param6), vmap);
        }
        "nodedelete" =>{
            Bluenc::nodedelete(&param1, vmap);

        }
        "nodegetposition" =>{
            return Bluenc::nodegetposition(&param1, vmap);
        }
        "nodegetrotation" =>{
            return Bluenc::nodegetrotation(&param1, vmap);
        }
        "nodegetscale" =>{
            return Bluenc::nodegetscale(&param1, vmap);
        }

        "bn3load" => {
            Bn3models::loadmodel(&param1,&param2, vmap);
            return "ok".to_owned();
        }
        "bn3setposition" => {
            Bn3models::setposition(&param1,&param2,&param3,&param4, vmap);
            return "ok".to_owned();
        }
        "bn3delete" => {
            Bn3models::delete(&param1, vmap);
            return "ok".to_owned();
        }
        "rotatequad" => { // maps this scope as function testing() in nscript.
            let fl64 = vmap.param2.parse::<f64>().unwrap_or(0.0);
            let forobj = vmap.param1.clone();
            testrotate(&forobj,&fl64, &vmap.param3.clone(),vmap);
            return vmap.param1.to_owned();
        }
        "textureload" =>{
            return Bluenc::textureload(&param1, vmap);
        }
        "textureset" => {
            Bluenc::textureset(&param1,&param2,vmap);
        }
        "nodesetcolor" => {
            //let getq = vmap.getvar("blueengine.color_q");
            if param1 != "" && param2 != ""{
                let colorstring = "".to_owned() + &param1 + "," + &param2;
                vmap.pushstringarray("blueengine.color_q", &colorstring);
            }
        }
        "image2d" => {
            return Bluenc::image2d(&param1, &nscript_f64(&param2), &nscript_f64(&param3), &nscript_f64(&param4), vmap);
        }
        "eguiwindow" =>{
            return Bluenc::eguiwindow(&param1, &param2, vmap);
        }
        "eguiclosewindow" =>{
            Bluenc::eguiclose(&param1, vmap);
        }
        "eguiopenwindow" => {
            Bluenc::eguiopen(&param1, vmap);
        }
        "eguibutton" => {
            Bluenc::eguibutton(&param1, &param2, &param3, vmap);
        }
        "eguilabel" => {
            Bluenc::eguilabel(&param1, &param2, &param3, vmap);
        }
        "eguihyperlink" => {
            Bluenc::eguihyperlink(&param1, &param2, &param3, vmap);
        }
        "eguiinput" => {
            Bluenc::eguiinput(&param1, &param2, &param3, vmap)
        }
        "eguipassword" => {
            Bluenc::eguipassword(&param1, &param2, &param3, vmap)
        }
        "eguicheckbox" => {
            Bluenc::eguicheckbox(&param1, &param2, &param3, vmap)
        }
        "eguicolorpicker" => {
            Bluenc::eguicolorpicker(&param1, &param2, &param3, vmap)
        }
        "eguisavefilebutton" => {
            Bluenc::eguisavefilebutton(&param1, &param2, &param3, &param4, vmap)
        }
        "eguiopenfilebutton" => {
            Bluenc::eguiopenfilebutton(&param1, &param2, &param3, &param4, vmap)
        }
        "eguislider" => {
            Bluenc::eguislider(&param1, &param2, &param3, &param4, &param5, vmap)
        }
        "eguiradio" => {
            Bluenc::eguiradio(&param1, &param2, &param3, &param4, vmap)
        }
        "eguicombo" => {
            Bluenc::eguicombo(&param1, &param2, &param3, &param4, vmap)
        }
        "pngfontload" =>{
            return Bluenc::loadpngfont(&param1, vmap);
        }
        "textnode" => {
            return Bluenc::textnode(&param1, &param2, &param3, &param4, &param5, &param6, vmap);
        }
        "textnodeupdate" => {
             Bluenc::textnodeupdate(&param1, &param2, &param3, &param4, &param5, &param6, vmap);
        }
        "textnodecolor" => {
            Bluenc::textnodecolor(&param1, &param2, vmap);
        }
        "textnodedelete" => {
            //let getq = vmap.getvar("blueengine.textdelete_q");
            vmap.pushstringarray("blueengine.textdelete_q", &param1);
        }


        _ =>{
            return "".to_owned();
        }
    }
    "ok".to_owned() // if no match continue core mappings.

}
