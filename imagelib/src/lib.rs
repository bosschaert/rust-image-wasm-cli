extern crate base64;
extern crate image;

pub enum Operation {
    Blur,
    Greyscale,
    HFlip,
    Invert,
    VFlip,
    Rotate,
}

pub fn transform_image_b64(base64: &str, op: Operation) -> String {
    let s = String::from(base64);

    let idx = s.find("base64,");
    match idx {
        Some(i) => {
            let b64 = &s[i+7..];
            let transformed = do_transform_img_b64(b64, op);

            String::from("data:image/png;base64,") + transformed.as_str()
        },
        None => String::from("Something went wrong")
    }
}

fn do_transform_img_b64(base64: &str, op: Operation) -> String {
    let dec = base64::decode(base64);

    match dec {
        Ok(img) => {
            let res = do_transform_img(&img, op);
            base64::encode(&res)
        },
        Err(_) => String::from(base64)
    }
}

pub fn do_transform_img(bytes: &[u8],op: Operation) -> Vec<u8> {
    let imgres = image::load_from_memory(bytes);
    match imgres {
        Ok(mut img) => {
            let res = match op {
                Operation::Blur => img.blur(1.276),
                Operation::Greyscale => img.grayscale(),
                Operation::HFlip => img.fliph(),
                Operation::Invert => {img.invert(); img},
                Operation::Rotate => img.rotate90(),
                Operation::VFlip => img.flipv()
            };

            let mut outbuf = Vec::new();
            res.write_to(&mut outbuf, image::ImageOutputFormat::PNG).unwrap();
            outbuf
        },
        Err(e) => {
            panic!("Error {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertical_flip() {
        let s = String::from("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI12P4//8/w38GIAXDIBKE0DHxgljNBAAO9TXL0Y4OHwAAAABJRU5ErkJggg==");

        let transformed = transform_image_b64(&s, Operation::VFlip);
        assert!(transformed.starts_with("data:image/png;base64,iVBOR"));
        assert_ne!(s, transformed);
    }
}
