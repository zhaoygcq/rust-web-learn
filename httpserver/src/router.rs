use http::{httprequest::{HttpRequest, self}, httpresponse::HttpResponse};
use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use std::io::prelude::*;
pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        // 根据请求方法作出响应
        match req.method {
            httprequest::Method::Get => match &req.resource {
                // 处理请求路径
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        },
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            }
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}