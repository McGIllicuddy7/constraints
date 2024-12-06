
pub use raylib::prelude::Color;
pub use raylib::prelude::Image;
use raylib::shaders::RaylibShader;
use raylib::texture::RaylibTexture2D;
use std::ops::Index;
use std::ops::IndexMut;
use std::sync::Arc;
use std::thread;
#[derive(Clone)]
pub struct ByteImage{
    colors:Box<[Color]>,
    height: usize, 
    width:usize
}

impl Index<usize> for ByteImage{
    type Output = [Color];
    fn index(&self, index: usize) -> &Self::Output {
        &self.colors[index*self.width..(index+1)*self.width]
    }
}

impl IndexMut<usize> for ByteImage{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.colors[index*self.width..(index+1)*self.width]
    }
}

impl ByteImage{
    pub fn new(image:&mut Image)->Self{
        let mut vcolors = Vec::new();
        vcolors.reserve(image.height as usize *image.width as usize);
        for y in 0..image.height{
            for x in 0..image.width{
                vcolors.push(image.get_color(x, y));
            }
        }
        Self{colors:vcolors.into_boxed_slice(), height:image.height as usize, width:image.width as usize}
    }
    pub fn new_from_color(colors:&[Color], height:usize, width:usize)->Self{
        let mut vcolors = Vec::new();
        vcolors.reserve_exact(height*width);
        for y in 0..height{
            for x in 0..width{
                vcolors.push(colors[y*width+x]);
            }
        }
        Self{colors:vcolors.into_boxed_slice(), height, width}
    }
    
    pub fn new_from_file(path:&str)->Result<Self,String>{
        let mut img = Image::load_image(path)?;
        Ok(Self::new(&mut img))
    }

    pub fn to_image(&self)->Image{
        let mut img = Image::gen_image_color(self.width as i32, self.height as i32, Color::BLACK);
        for y in 0..self.height{
            for x in 0..self.width{
                img.draw_pixel(x as i32, y as i32, self[y][x]);
            }
        }
        img
    }

    pub fn get_height(&self)->usize{
        self.height
    }

    pub fn get_width(&self)->usize{
        self.width
    }

    pub fn get_data(&self)->&[Color]{
        &self.colors
    }

    pub fn get_data_mut(&mut self)->&mut [Color]{
        &mut self.colors
    }

    pub fn sub_image(&self, x_start:usize, y_start:usize, x_end:usize, y_end:usize)->Self{
        let mut out_buff = Vec::new();
        out_buff.reserve_exact((x_end-x_start)*(y_end-y_start));
        for y in y_start..y_end{
            for x in x_end..x_end{
                out_buff.push(self[y][x]);
            }
        }
        Self { colors: out_buff.into_boxed_slice(), height: y_end-y_start, width: x_end-x_start }
    }

    pub fn export(&self, file_name:&str){
        let img = self.to_image();
        img.export_image(file_name);
    }

    fn kernel_blur(&self,x:usize, y:usize,kernel_size:usize, exp_divisor:f64)->Color{
        let ks = kernel_size as isize;
        let mut total_mlt = 0.0;
        let mut col = [0.0, 0.0, 0.0, 0.0];
        for dy in -ks..ks{
            for dx in -ks..ks{
                let ix = x as isize + dx;
                let iy = y as isize+dy;
                if ix<0 || iy<0 || ix>= self.width as isize || iy >= self.height as isize{
                    continue;
                }
                let vx = ix as usize;
                let vy = iy as usize;
                let scaler = fast_math::exp((-((dx*dx+dy*dy) as f64)/exp_divisor)as f32) as f64;
                total_mlt += scaler;
                let c = self[vy][vx];
                col[0] += (c.r as f64)*scaler;
                col[1] += (c.g as f64)*scaler;
                col[2] += (c.b as f64)*scaler;
                col[3] += (c.a as f64)*scaler;
            }
        }
        col[0] /= total_mlt;
        col[1] /= total_mlt;
        col[2] /= total_mlt;
        col[3] /= total_mlt;
        Color { r: col[0] as u8, g: col[1] as u8, b: col[2] as u8, a: col[3] as u8 }
    }

    pub fn blur_single_threaded(&self, kernel_size:usize, exp_divisor:f64)->Self{
        let mut out = self.clone();
        for y in 0..self.height{
            for x in 0..self.width{
                out[y][x] = self.kernel_blur(x, y, kernel_size, exp_divisor);
            }
        }
        out
    }



    pub fn blur(&self, kernel_size:usize, exp_divisor:f64)->Self{
        fn blur_thread(img:Arc<ByteImage>, kernel_size:usize, exp_divisor:f64, x_start:usize, y_start:usize, x_end:usize, y_end:usize)->Vec<Color>{
            let mut out = Vec::new();
            for y in y_start..y_end{
                for x in x_start..x_end{
                    out.push(img.kernel_blur(x, y, kernel_size, exp_divisor));
                }
            }
            out
        }
        let nt:usize = std::thread::available_parallelism().expect("should work?").into();
        if self.height<nt{
            return self.blur_single_threaded(kernel_size, exp_divisor);
        }
        let img = Arc::new(self.clone());
        let w = self.width;
        let y = self.height;
        let mut results = Vec::new();
        for i in 0..nt{
            let by = y/nt*i;
            let ey = if i != nt-1{y/nt*(i+1)+1} else{y};
            let img0 = img.clone();
            let ft = thread::spawn(move ||{blur_thread( img0, kernel_size, exp_divisor,0, by, w,ey)});
            results.push(ft);
        }
        let mut outbuffer = Vec::new();
        outbuffer.reserve_exact(self.height*self.width);
        for i in results{
            let s = i.join().expect("please help");
            for j in s{
                outbuffer.push(j);
            }
        }
        Self::new_from_color(&outbuffer, self.height, self.width)
       
    }

    pub fn blur_shader(&self, thread:&raylib::prelude::RaylibThread, handle:&mut raylib::prelude::RaylibHandle, kernel_size:usize, divisor:f64)->Result<Self, String>{
        static VS_CODE:&str = core::include_str!("shaders/blur.vs");
        static FS_CODE:&str = core::include_str!("shaders/blur.fs");
        let img = self.to_image();
        let tex = handle.load_texture_from_image(thread, &img)?;
        let render_tex = handle.load_render_texture(thread, self.width as u32, self.height as u32)?;
        
        let mut shader = handle.load_shader_from_memory(thread, Some(VS_CODE), Some(FS_CODE)); 
        shader.set_shader_value_v(shader.get_shader_location("height"), &[self.height as i32]);
        shader.set_shader_value_v(shader.get_shader_location("width"), &[self.width as i32]);
        shader.set_shader_value_v(shader.get_shader_location("kernel_size"), &[kernel_size as i32]);
        shader.set_shader_value_v(shader.get_shader_location("divisor"), &[divisor as i32]);
        {
            unsafe{
                use raylib::ffi::*;
                BeginTextureMode(*render_tex);
                BeginShaderMode(*shader);
                rlSetTexture(tex.id);
                rlBegin(raylib::ffi::RL_QUADS as i32);
                rlColor4f(1.0, 1.0, 1.0, 1.0);
                rlTexCoord2f(0.0, 1.0);
                rlVertex2f(0.0, 0.0);
                rlTexCoord2f(0.0, 0.0);
                rlVertex2f(0.0, self.height as f32);
                rlTexCoord2f(1.0, 0.0);
                rlVertex2f(self.width as f32, self.height as f32);
                rlTexCoord2f(1.0, 1.0);
                rlVertex2f(self.width as f32,0.0);
    
                raylib::ffi::rlEnd();
                EndTextureMode();
            }
        }
        let mut img = render_tex.load_image()?;
        Ok(Self::new(&mut img))
    }

    pub fn guass_diff(&self, kernel_size1:usize, exp_divisor1:f64, kernel_size2:usize, exp_divisor2:f64)->Self{
        let mut img1 = self.blur(kernel_size1, exp_divisor1);
        let img2 = self.blur(kernel_size2, exp_divisor2);
        for y in 0..img1.height{
            for x in 0..img1.width{
                img1[y][x].r  = (img1[y][x].r as f64- img2[y][x].r as f64).abs() as u8;
                img1[y][x].g  = (img1[y][x].g as f64- img2[y][x].g as f64).abs() as u8;
                img1[y][x].b = (img1[y][x].b as f64- img2[y][x].b as f64).abs() as u8;
            }
        }
        img1
    }
    
    pub fn guass_diff_single_threaded(&self, kernel_size1:usize, exp_divisor1:f64, kernel_size2:usize, exp_divisor2:f64)->Self{
        let mut img1 = self.blur_single_threaded(kernel_size1, exp_divisor1);
        let img2 = self.blur_single_threaded(kernel_size2, exp_divisor2);
        for y in 0..img1.height{
            for x in 0..img1.width{
                img1[y][x].r  = (img1[y][x].r as f64- img2[y][x].r as f64).abs() as u8;
                img1[y][x].g  = (img1[y][x].g as f64- img2[y][x].g as f64).abs() as u8;
                img1[y][x].b = (img1[y][x].b as f64- img2[y][x].b as f64).abs() as u8;
            }
        }
        img1
    }


    pub fn guass_diff_shader(&self, thread:&raylib::prelude::RaylibThread, handle:&mut raylib::prelude::RaylibHandle, kernel_size0:usize, divisor0:f64, kernel_size1:usize, divisor1:f64, b_and_w:bool)->Result<Self, String>{
        static VS_CODE:&str = core::include_str!("shaders/blur.vs");
        static FS_CODE:&str = core::include_str!("shaders/diff.fs");
        let img = self.to_image();
        let tex = handle.load_texture_from_image(thread, &img)?;
        let render_tex = handle.load_render_texture(thread, self.width as u32, self.height as u32)?;
        
        let mut shader = handle.load_shader_from_memory(thread, Some(VS_CODE), Some(FS_CODE)); 
        shader.set_shader_value_v(shader.get_shader_location("height"), &[self.height as i32]);
        shader.set_shader_value_v(shader.get_shader_location("width"), &[self.width as i32]);
        shader.set_shader_value_v(shader.get_shader_location("kernel_size0"), &[kernel_size0 as i32]);
        shader.set_shader_value_v(shader.get_shader_location("divisor0"), &[divisor0 as i32]);
        shader.set_shader_value_v(shader.get_shader_location("kernel_size1"), &[kernel_size1 as i32]);
        shader.set_shader_value_v(shader.get_shader_location("divisor1"), &[divisor1 as i32]);
        shader.set_shader_value_v(shader.get_shader_location("b_and_w"), &[b_and_w as i32]);
        {
            unsafe{
                use raylib::ffi::*;
                BeginTextureMode(*render_tex);
                BeginShaderMode(*shader);
                rlSetTexture(tex.id);
                rlBegin(raylib::ffi::RL_QUADS as i32);
                rlColor4f(1.0, 1.0, 1.0, 1.0);
                rlTexCoord2f(0.0, 1.0);
                rlVertex2f(0.0, 0.0);
                rlTexCoord2f(0.0, 0.0);
                rlVertex2f(0.0, self.height as f32);
                rlTexCoord2f(1.0, 0.0);
                rlVertex2f(self.width as f32, self.height as f32);
                rlTexCoord2f(1.0, 1.0);
                rlVertex2f(self.width as f32,0.0);
    
                raylib::ffi::rlEnd();
                EndTextureMode();
            }
        }
        let mut img = render_tex.load_image()?;
        Ok(Self::new(&mut img))
    }
    pub fn cell_shader(&self, thread:&raylib::prelude::RaylibThread, handle:&mut raylib::prelude::RaylibHandle, kernel_size:usize, divisor:f64)->Result<Self, String>{
        static VS_CODE:&str = core::include_str!("shaders/blur.vs");
        static FS_CODE:&str = core::include_str!("shaders/cell.fs");
        let img = self.to_image();
        let tex = handle.load_texture_from_image(thread, &img)?;
        let render_tex = handle.load_render_texture(thread, self.width as u32, self.height as u32)?;
        
        let mut shader = handle.load_shader_from_memory(thread, Some(VS_CODE), Some(FS_CODE)); 
        shader.set_shader_value_v(shader.get_shader_location("height"), &[self.height as i32]);
        shader.set_shader_value_v(shader.get_shader_location("width"), &[self.width as i32]);
        shader.set_shader_value_v(shader.get_shader_location("kernel_size"), &[kernel_size as i32]);
        shader.set_shader_value_v(shader.get_shader_location("divisor"), &[divisor as i32]);
        {
            unsafe{
                use raylib::ffi::*;
                BeginTextureMode(*render_tex);
                BeginShaderMode(*shader);
                rlSetTexture(tex.id);
                rlBegin(raylib::ffi::RL_QUADS as i32);
                rlColor4f(1.0, 1.0, 1.0, 1.0);
                rlTexCoord2f(0.0, 1.0);
                rlVertex2f(0.0, 0.0);
                rlTexCoord2f(0.0, 0.0);
                rlVertex2f(0.0, self.height as f32);
                rlTexCoord2f(1.0, 0.0);
                rlVertex2f(self.width as f32, self.height as f32);
                rlTexCoord2f(1.0, 1.0);
                rlVertex2f(self.width as f32,0.0);
    
                raylib::ffi::rlEnd();
                EndTextureMode();
            }
        }
        let mut img = render_tex.load_image()?;
        Ok(Self::new(&mut img))
    }

    pub fn does_it_rotate(&self)->Self{
        let mut out = self.clone();
        for y in 0..self.height{
            for x in 0..self.width{
                out[y][x] = self[y][x];
            }
        }
        out
    }
    pub fn length(&self)->f64{
        let mut out = 0.0;
        for y in 0..self.height{
            for x in 0..self.width{
                out += self[y][x].r as f64;
                out += self[y][x].g as f64;
                out += self[y][x].b as f64;
            }
        }
        out
    }

    #[allow(unused)]
    pub fn shift_horizontal(&self, shift:isize)->Self{
        let shift = shift %(self.width as isize);
        let mut bytes = Vec::new();
        bytes.reserve(self.height*self.width);
        for _ in 0..self.height*self.width{
            bytes.push(Color::BLACK);
        }
        for y in 0..self.height{
            for x in 0..self.width{
                let xdelt = {
                    let t = x as isize + shift;
                    if t<0{
                        (self.width as isize-1 +t ) as usize
                    } else if t>=self.width as isize{
                        t as usize-self.width
                    }
                    else{
                        t as usize
                    }
                };
                bytes[y*self.width+xdelt] = self[y][x];
            }
        }
        Self::new_from_color(&bytes, self.height, self.width)
    }
    #[allow(unused)]
    pub fn shift_vertical(&self, shift:isize)->Self{
        let shift = shift %(self.height as isize);
        let mut bytes = Vec::new();
        bytes.reserve(self.height*self.width);
        for _ in 0..self.height*self.width{
            bytes.push(Color::BLACK);
        }
        for y in 0..self.height{
            for x in 0..self.width{
                let ydelt = {
                    let t = y as isize + shift;
                    if t<0{
                        (self.height as isize -1+t ) as usize
                    } else if t>=self.height as isize{
                        t as usize-self.height
                    }
                    else{
                        t as usize
                    }
                };
                bytes[ydelt*self.width+x] = self[y][x];
            }
        }
        Self::new_from_color(&bytes, self.height, self.width)
    }
}

#[allow(unused)]
pub fn byte_image_dot_product_no_normalization(a:&ByteImage, b:&ByteImage)->f64{
    let mut out = 0.0;
    let al = 1.0;
    let bl = 1.0;
    println!("al:{},bl:{}", al,bl);
    for y in 0..a.height{
        for x in 0..a.width{
            out += (a[y][x].r as f64/al)*(b[y][x].r as f64)/bl;
            out += (a[y][x].g as f64)/al*(b[y][x].g as f64)/bl;
            out += (a[y][x].b as f64)/al*(b[y][x].b as f64)/bl;
        }
    }
    out 
}
#[allow(unused)]
pub fn byte_image_dot_product(a:&ByteImage, b:&ByteImage)->f64{
    let mut out = 0.0;
    let al = byte_image_dot_product_no_normalization(&a, &a).sqrt();
    let bl = byte_image_dot_product_no_normalization(&b, &b).sqrt();
    println!("al:{},bl:{}", al,bl);
    for y in 0..a.height{
        for x in 0..a.width{
            out += (a[y][x].r as f64/al)*(b[y][x].r as f64)/bl;
            out += (a[y][x].g as f64)/al*(b[y][x].g as f64)/bl;
            out += (a[y][x].b as f64)/al*(b[y][x].b as f64)/bl;
        }
    }
    out
}

#[allow(unused)]
pub fn byte_image_comparision(a:&ByteImage, b:&ByteImage)->f64{
    let kernel_size1 = 10;
    let exp_divisor1 = 2.0;
    let kernel_size2 = 20;
    let exp_divisor2 = 4.0;
    let a_edges = a.guass_diff( kernel_size1, exp_divisor1, kernel_size2, exp_divisor2);
    let b_edges = b.guass_diff( kernel_size1, exp_divisor1, kernel_size2, exp_divisor2);
    let a_blur = a_edges.blur(10, 40.0);
    let b_blur = b_edges.blur(10, 40.0); 
    a_blur.export("a_blur.png");
    b_blur.export("b_blur.png");
    byte_image_dot_product(&a_blur, &b_blur)
}