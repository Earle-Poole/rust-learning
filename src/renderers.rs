use increment_count;
use nickel::{MiddlewareResult, Request, Response};
use say_hello;
use std::collections::HashMap;

pub fn home_render<'mw, 'conn>(
    _req: &mut Request<'mw, 'conn>,
    res: Response<'mw>,
) -> MiddlewareResult<'mw> {
    let count_result = increment_count::func();
    let mut data = HashMap::new();

    let header = say_hello::func(Some(
        [
            "visitor #: ",
            count_result.unwrap().count.to_string().as_mut_str(),
        ]
        .concat(),
    ));

    data.insert("header", header);
    return res.render("./assets/home.tpl", &data);
}

// pub fn favicon_render<'mw, 'conn>(
//     _req: &mut Request<'mw, 'conn>,
//     res: Response<'mw>,
// ) -> MiddlewareResult<'mw> {
//     return res.send_file("./assets/favicon.ico");
// }
