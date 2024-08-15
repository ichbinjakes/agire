use crate::http::types::HttpMethod;
use crate::server::traits::{Request,Response};
use crate::server::context::RequestContext;
use crate::server::error::{ServerError,StdServerError};


pub struct Route<T: Request, R: Response> {
    pub path: String,
    pub func: Box<dyn Fn(RequestContext<T, R>) -> Result<RequestContext<T, R>, ServerError>>,
    pub methods: Vec<HttpMethod>,
}


pub struct Router<T: Request, R: Response> {
    routes: Vec<Route<T, R>>,
}

impl<T: Request, R: Response> Router<T, R> {
    pub fn new(routes: Vec<Route<T, R>>) -> Self {
        Self {
            routes: routes,
        }
    }
}


impl<T: Request, R: Response> Router<T, R> {
    
    // Find the route requested via path matching
    fn match_path_to_route(&self, request: &impl Request) -> Result<&Route<T, R>, ServerError> {

        // simple matching:
        for route in self.routes.iter() {
            if route.path == request.get_path() {
                for method in route.methods.iter() {
                    if *method == request.get_method() {
                        return Ok(&route)
                    }
                }
            }
        }

        // TODO: regex matching

        Err(StdServerError::NotFound.to_error())
    }

    // Execute route handler
    pub fn dispatch(&self, ctx: RequestContext<T, R>) -> Result<RequestContext<T, R>, ServerError> {
        let request = ctx.get_request();

        let route = match self.match_path_to_route(request) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
                
        let f = &route.func;
        f(ctx)
    }

}
