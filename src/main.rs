
/**
# Используемые ресурсы 

- [[DOC] Для работы с изображениями](http://www.piston.rs/image/image/struct.ImageBuffer.html)
- [[LIB] Для работы с изображениями](https://github.com/PistonDevelopers/image)
- [Работа с путями](https://doc.rust-lang.org/beta/std/path/struct.Path.html)
- [Документация по структурам](https://rurust.github.io/rust_book_ru/src/structs.html)
*/

extern crate image;
extern crate rand;

/// Для работы с файлами
use std::fs;

use std::fs::{ File, read_dir } ;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

/// Для генерации изображения
use rand::Rng;

/// Для получения списка аргументов
use std::env;

/// Для работы с изображениями 
use image::{ GenericImage, ImageBuffer, Rgba, imageops };

/// Опциональность
use std::option::Option;

/// Debug descriptions
use std::fmt;

struct ImgFileMeta { 
		
	file_path: String, 
	for_save_file_path: String, 

	origin_file_name: String,
	for_save_file_name: String,

	directory_path: String
}


impl fmt::Debug for ImgFileMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImgFileMeta {{ x: {}, y: {} }}", self.file_path, self.for_save_file_path)
    }
}


fn main() {
	
	// Получаем и выводим список аргументов запуска    
	let args: Vec<String> = env::args().collect();

    println!("[Crop Images] {:?}", args);

    for arg in &args {
    	println!("	- {}", arg);
    }

	if args.len() <= 1 {
		println!("Use CropImages (f or d) /path-to-file-ordirectory/");
		return;
	}

	if args.len() <= 2 {
		println!("Use CropImages (f or d) /path-to-file-ordirectory/");
		return;
	}

	// Срезаем первый элемент (Текущая дирректория)
	let args = &args[1..];

	// Тип (файл или дирректория)
	let str_arg = &args[0];
    
	// путь
    let path = &args[1];
	
	if str_arg == "f" {
		
		processing_file(path);
		
		return;
	}

	if str_arg == "d" {

		processing_directory(path);

		return;
	}

	println!("Use CropImages (f or d) /path-to-file-ordirectory/");
}

fn processing_directory(path: &String) {
	
	let is_exist = self::path_exists(&path);
	let is_dir = self::is_dir(&path);

	if !is_exist {
		println!("Directory not found {:?}", path);
		return;
	}

	if !is_dir {
		println!("it is not directory {:?}", path);
		return;
	}
   
   	let paths = fs::read_dir(path).unwrap();

    for path in paths {

	    if let Some(file_path) = path.unwrap().path().to_str() {
	    	
	    	let file_path = file_path.to_string();
			
			println!("File: {}", &file_path);

	    	processing_file(&file_path);
		} 
    }

}

fn processing_file(path: &String) {
	let is_exist = path_exists(&path);
	let is_file = is_file(&path);

	if !is_exist {
		println!("File not found {:?}", path);
		return;
	}

	if !is_file {
		println!("it is not file {:?}", path);
		return;
	}

	processing(&path);
}

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn is_dir(path: &str) -> bool {
	let meta: fs::Metadata = match fs::metadata(path) {
            Ok(num) => num,
            Err(_) => return false,
     };

	return meta.is_dir();
}

pub fn is_file(path: &str) -> bool {
	
	let meta: fs::Metadata = match fs::metadata(path) {
            Ok(num) => num,
            Err(_) => return false,
    };

	return meta.is_file();
}


fn processing(path: &String) {
	
	if let Some(meta) = file_meta(path) {



		if let Ok(_) = image::open(path) {
			
			let ref mut img = image::open(path).unwrap();

			let height = img.height();
			let width = img.width();

			println!("dimensions {:?} {:?}", img.dimensions(), img.color());
			
			let subimg = imageops::crop(img, 20, 20, width - 40, height - 124);

			let imgBuff = subimg.to_image();

			imgBuff.save(meta.for_save_file_path).expect("[Error from save]");
		} else {
			println!("[NOT IMG]: {:?} ", path);
		}
	}
}

fn file_meta(path: &String) -> Option<ImgFileMeta> {

	let strPath = path.to_string();

	let path = Path::new(path);
	
	let mut origin_file_name = String::new();
	if let Some(name) = path.file_name()?.to_str() {
		origin_file_name = name.to_string();
	} else {
		println!("File name not found, ABORT");
		return None;		
	}

	let mut directory_path = String::new();
	if let Some(_parent) = path.parent()?.to_str() {
		directory_path = _parent.to_string();
	} else {
		println!("File name not found, ABORT");
		return None;
	}

	let for_save_file_name: String = format!("PROCESSING_{}", origin_file_name);
	let mut for_save_file_path: String = String::new();

	if let Some(_for_save_file_path) = path.with_file_name(&
		for_save_file_name).to_str() {
		for_save_file_path = _for_save_file_path.to_string()
	} else {
		println!("File name not found, ABORT");
		return None;
	}

	let imgFileMeta = ImgFileMeta {
		file_path: strPath, 
		for_save_file_path: for_save_file_path,
		origin_file_name: origin_file_name,
		for_save_file_name: for_save_file_name,
		directory_path: directory_path
	};

	return Some(imgFileMeta);
}

/*
fn demo() {
    let mut image = ImageBuffer::<Rgba<u8>, _>::new(800, 800);

	for (x, y, pixel) in image.enumerate_pixels_mut() { 

		let r = rand::thread_rng().gen_range(1, 255);
		let g = rand::thread_rng().gen_range(1, 255);
		let b = rand::thread_rng().gen_range(1, 255);

		pixel.data = [r, g, b, 255];
	}

	image.save("output.png").unwrap();
}
*/