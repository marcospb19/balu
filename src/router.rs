use std::{collections::HashMap, fmt};

use crate::{response::IntoResponse, Method, Request, Response};

#[derive(Debug, Default)]
pub struct Router {
    routes: Vec<Route>,
    lookup: HashMap<(Method, &'static str), usize>,
}

impl Router {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn get<F, R>(self, path: &'static str, mut handler: F) -> Self
    where
        F: FnMut(Request) -> R + 'static,
        R: IntoResponse,
    {
        self._add(Method::Get, path, Box::new(move |req| handler(req).into_response()))
    }

    #[inline]
    pub fn add<F, R>(self, method: Method, path: &'static str, mut handler: F) -> Self
    where
        F: FnMut(Request) -> R + 'static,
        R: IntoResponse,
    {
        self._add(method, path, Box::new(move |req| handler(req).into_response()))
    }

    fn _add(mut self, method: Method, path: &'static str, handler: Box<dyn FnMut(Request) -> Response>) -> Self {
        let index = self.routes.len();
        let route = Route {
            handler: Box::new(handler),
            path,
        };
        self.routes.push(route);

        let lookup_element = (method, path);

        assert!(
            self.lookup.insert(lookup_element, index).is_none(),
            "Duplicated routes with {path:?}"
        );

        self
    }

    pub fn lookup_handler(&mut self, method: Method, path: &str) -> Option<impl FnMut(Request) -> Response + '_> {
        let &index = self.lookup.get(&(method, path))?;

        let route = self.routes.get_mut(index)?;

        Some(&mut route.handler)
    }
}

pub struct Route {
    handler: Box<dyn FnMut(Request) -> Response>,
    path: &'static str,
}

impl fmt::Debug for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}
