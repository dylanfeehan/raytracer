use image::{ImageBuffer, Rgb};

fn main() {
    const VH: u32 = 800;
    const VW: u32 = 800;
    println!("Hello, world!");

    // impl <P: Pixel> ImageBuffer<P, Vec<P::Subpixel>>
    //  - this says that "the following function definitions take place on the ImageBuffer<P, Vec<P::Subpixel>> type where P is a pixel variant"
    //  - So p must be some struct that implements Pixel. One such struct is Rgb. Rgb struct from the image crate implements the Pixel trait!
    //  -   Rgb 'Pixel' impl: 
    //  -       impl <T: Primitive + Enlargeable> Pixel for Rgb<T>
    //  -       so we're seeing the implementation of Pixel for Rgb! Where T is some Primitive, Enlargeable type... so what exactly is.. Primitive Enlargeable?
    //  -           Primitive: basically a sized int (u8, u32, i32)
    //  -           Enlargeable: primitives implement these, it basically means that they can increase in capacity. They can grow, enlarge. here's an impl on u32:

    //  -               impl Enlargeable for u32 {
    //  -                   type Larger = u64;
    //  -               }

    //  - Lastly, what is a Subpixel?

    // pub fn new(width: u32, height: u32) -> ImageBuffer<P, Vec<P::Subpixel>>
    // 
    let mut img_buf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(VH, VW);
    let red_pixel: Rgb<u8> = Rgb([255, 0, 0]);
    for x in 0..VW {
        for y in 0..VH {
            img_buf.put_pixel(x, y, red_pixel);
        }
    }
    img_buf.save("color.png");

}
