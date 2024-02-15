use crate::*;

pub struct Bn3models{
    pub tilemap: HashMap<String,Vec<String>>,

}
impl Bn3models{
    pub fn new() -> Bn3models{
        Bn3models{
            tilemap: HashMap::new(),


        }
    }
    pub fn loadmodel(objectname:&str,filename:&str,vmap:&mut Varmap){
        let filedata = Nfile::read(filename);
        let mut part_i = 0;

        //pool to hold the quad parts on as a object
        let objectparts = objectname.to_owned() + ".parts";
        let mut objectpartpool = vmap.getvar(&objectparts);

        for parts in split(&filedata,"\n"){
            let quadparts = split(parts,",");
            if quadparts.len() > 9{
                part_i +=1;
                let partnode = objectname.to_owned() + "__" + &part_i.to_string();
                let texturestring = partnode.to_owned() + "," + quadparts[0] +",";
                let positionstring = partnode.to_owned() + "," + quadparts[1] + "," + quadparts[2] + "," + quadparts[3]+",";
                let rotx = partnode.to_owned() + "," + quadparts[4] + ",x,";
                let roty = partnode.to_owned() + "," + quadparts[5] + ",y,";
                let rotz = partnode.to_owned() + "," + quadparts[6] + ",z," ;
                let scale = partnode.to_owned() + "," + quadparts[7] + "," + quadparts[8] + "," + quadparts[9];

                objectpartpool = pooladd(&objectpartpool,&partnode);

                let properties = [".texture",".x",".y",".z",".rx",".ry","rz",".sx",".sy",".sz"];
                let mut index = 0;
                // assign all properties to subquad
                for prop in properties{
                    let setx = partnode.to_owned() + &prop;
                    vmap.setvar(setx.to_owned(),&quadparts[index]);
                    index +=1;
                }

                let properties = [".offx",".offy",".offz"];
                let mut index = 1;
                // assign all properties to subquad
                for prop in properties{
                    let setx = partnode.to_owned() + &prop;
                    vmap.setvar(setx.to_owned(),&quadparts[index]);
                    index +=1;
                }
                // update all the quees in main.rs so the wgpu renders all.
                let mut getq = vmap.getvar("blueengine.square_q");
                getq = pooladd(&getq,&partnode);
                vmap.setvar("blueengine.square_q".to_owned(),&getq);

                let mut getq = vmap.getvar("blueengine.textureset_q");
                //let texturestring = partnode.to_owned() + "," + quadparts[0];
                getq = pooladd(&getq,&texturestring);
                vmap.setvar("blueengine.textureset_q".to_owned(),&getq);

                let mut getq = vmap.getvar("blueengine.position_q");
                getq = pooladd(&getq,&positionstring);
                vmap.setvar("blueengine.position_q".to_owned(),&getq);

                let mut getq = vmap.getvar("blueengine.rotation_q");
                getq = pooladd(&getq,&rotx);
                getq = pooladd(&getq,&roty);
                getq = pooladd(&getq,&rotz);
                vmap.setvar("blueengine.rotation_q".to_owned(),&getq);

                let mut getq = vmap.getvar("blueengine.scale_q");
                getq = pooladd(&getq,&scale);
                vmap.setvar("blueengine.scale_q".to_owned(),&getq);

            }
        }
        vmap.setvar(objectparts.to_owned(),&objectpartpool);
        let properties = [".x",".y",".z"];
        let mut index = 0;
        for prop in properties{
            let setx = objectname.to_owned() + &prop;
            vmap.setvar(setx.to_owned(),"0.0");
        }

    }
    pub fn setposition(objectname:&str,x:&str,y:&str,z:&str,vmap:&mut Varmap){
        let xparsed = match x.parse::<f32>(){
            Ok(res) => res,
            Err(_) =>{
                println!("Error setting bn3 model positionx {}",x);
                0.0
            }
        };
        let yparsed = match y.parse::<f32>(){
            Ok(res) => res,
            Err(_) =>0.0
        };
        let zparsed = match z.parse::<f32>(){
            Ok(res) => res,
            Err(_) =>0.0
        };


        let objectparts = objectname.to_owned() + ".parts";
        let  objectpartpool = vmap.getvar(&objectparts);
        if objectpartpool == ""{
            // well theres nothing in the object!
            return
        }

        let center_xref = objectname.to_owned() + ".x";
        let  center_x = match vmap.getvar(&center_xref).parse::<f32>(){
            Ok(res) => res ,
            Err(_) =>0.0
        };
        //let newcenterx = center_x + xparsed;
        vmap.setvar(center_xref,&xparsed.to_string());

        let center_yref = objectname.to_owned() + ".y";
        let  center_y = match vmap.getvar(&center_yref).parse::<f32>(){
            Ok(res) => res,
            Err(_) =>0.0
        };
        //let newcentery = center_y + yparsed;
        vmap.setvar(center_yref,&yparsed.to_string());

        let center_zref = objectname.to_owned() + ".z";
        let  center_z = match vmap.getvar(&center_zref).parse::<f32>(){
            Ok(res) => res,
            Err(_) =>0.0
        };
        //let newcenterz = center_z + zparsed;
        vmap.setvar(center_zref,&zparsed.to_string());

        //let mut updatepool = vmap.getvar("blueengine.update_q");

        let mut pospool = vmap.getvar("blueengine.position_q");
        for thispart in split(&objectpartpool,NC_ARRAY_DELIM){
            if thispart != "" {
                let thispartxref = thispart.to_owned() + ".offx";
                let thispartx = match vmap.getvar(&thispartxref).parse::<f32>(){
                    Ok(res) => res + xparsed,
                    Err(_) => xparsed
                };
                let thispartyref = thispart.to_owned() + ".offy";
                let thisparty = match vmap.getvar(&thispartyref).parse::<f32>(){
                    Ok(res) => res + yparsed,
                    Err(_) => yparsed
                };
                let thispartzref = thispart.to_owned() + ".offz";
                let thispartz = match vmap.getvar(&thispartzref).parse::<f32>(){
                    Ok(res) => res + zparsed,
                    Err(_) => zparsed
                };

                let thispartxref = thispart.to_owned() + ".x";
                let thispartzref = thispart.to_owned() + ".z";
                let thispartyref = thispart.to_owned() + ".y";
                vmap.setvar(thispartxref,&thispartx.to_string());
                vmap.setvar(thispartyref,&thisparty.to_string());
                vmap.setvar(thispartzref,&thispartz.to_string());
                let posqstring = thispart.to_owned() + "," + &thispartx.to_string() + "," + &thisparty.to_string() + "," + &thispartz.to_string() + ",";
                pospool = pooladd(&pospool,&posqstring);
                //updatepool = pooladd(&updatepool,&thispart);
            }
        }
        //vmap.setvar("blueengine.update_q".to_owned(),&updatepool);
        vmap.setvar("blueengine.position_q".to_owned(),&pospool);

    }
    pub fn delete(object:&str,vmap:&mut Varmap){
        let objectparts = object.to_owned() + ".parts";
        if objectparts != ""{
            let  objectpartpool = vmap.getvar(&objectparts);
            let mut deletionq = vmap.getvar("blueengine.deletion_q");
            for part in split(&objectpartpool,NC_ARRAY_DELIM){
                deletionq = pooladd(&deletionq,&part);
                vmap.delobj(part);
            }
            vmap.setvar("blueengine.deletion_q".to_owned(),&deletionq);
            vmap.delobj(object);
        }
    }

}
