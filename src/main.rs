use std::fs;
use std::io::Write;
use substring::Substring;



fn main()->Result<(),std::io::Error>{
    let mut header_names: Vec<String>= Vec::new();
    let mut makefile = fs::File::create("Makefile").expect("Unable to create Makefile");
    for entry in fs::read_dir("./includes")?{
        let entry = entry?;
        let path= entry.path();
        let mut filename=path.into_os_string().into_string().unwrap();
        filename = filename.substring(11,filename.len()).to_string();
        header_names.push(filename);
    }


    makefile.write_all(b"bin/build: bin/app.o ").expect("Unable to write data");
    for i in 0..header_names.len(){
        let tmp:String= format!("bin/{}.o ",header_names[i]);
        makefile.write_all(tmp.as_bytes()).expect("Unable to write data");
    }
    makefile.write_all(b"\n\tg++ -std=c++17 bin/app.o ")?;
    for i in 0..header_names.len(){
        let tmp:String= format!("bin/{}.o ",header_names[i]);
        makefile.write_all(tmp.as_bytes()).expect("Unable to write data");
    }
    makefile.write(b"-o bin/build")?;
    makefile.write_all(b"\n\nbin/app.o: source/app.cpp\n\tg++ --std=c++17 -c source/app.cpp -o bin/app.o")?;
    for i in 0..header_names.len(){
        let tmp = format!("\n\nbin/{a}.o: includes/{a}/{a}.cpp includes/{a}/{a}.h\n\tg++ --std=c++17 -c includes/{a}/{a}.cpp -o bin/{a}.o",a=header_names[i]);
        makefile.write_all(tmp.as_bytes()).expect("Unable to write data");
    }
    makefile.write_all(b"\n\nclean:\n\trm -rf bin/*.o\n")?;
    Ok(())

}
