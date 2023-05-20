use mime::Mime;


pub fn find_mime_type (filename : &String) -> Mime{

    let parts : Vec<&str> = filename.split('.').collect();

    let res = match parts.last() {
            Some(v) =>
                match *v {
                    "png" => mime::IMAGE_PNG,
                    "jpg" => mime::IMAGE_JPEG,
                    "json" => mime::APPLICATION_JSON,
                    "js" => mime::APPLICATION_JAVASCRIPT_UTF_8,
                    "mjs" => mime::APPLICATION_JAVASCRIPT_UTF_8,
                    "css" => mime::TEXT_CSS_UTF_8,
                    &_ => mime::TEXT_PLAIN,
                },
            None => mime::TEXT_PLAIN,
        };

    return res;
}