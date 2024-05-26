use crate::*;

pub fn loadfontdir(dir: &str,vmap:&mut Varmap){
    //let filearray = .to_owned();
    let mut getq = vmap.getvar("blueengine.textureload_q");
    for xfile in split(&Nfile::dirtolist(dir,true),NC_ARRAY_DELIM){

                //let texturestring = partnode.to_owned() + "," + quadparts[0];
                getq = pooladd(&getq,&xfile);

    }
    vmap.setvar("blueengine.textureload_q".to_owned(),&getq);
}
