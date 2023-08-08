use std::{io::{self, Read}, fs::File};



fn main() {
    // throw error using panic macro 
   let  result =  read_file();
   let s =  match result {
    // return value 
     Ok(value)=> value,
     Err(err)=> {
        panic!("file not found {:?}", err)
     }
   };
   println!("{}",s);
    //   a();
}

// Result Enum 

// fn read_text_file (  ){
//   let f =   File::open("random.txt");
//   let f = match f {
//         Ok(file)=>file,
//         Err(error)=>panic!("error reading the file {:?}", error)
//     };
//     println!("{:?}",f);
// }


fn read_file ()-> Result<String, io::Error>{
    let mut s = String::new();
    File::open("random.txt")?.read_to_string(&mut s )?;
    Ok(s)
}





