use std::{collections::HashMap, fmt};

use crate::{response::IntoResponse, Method, Request, Response};

#[derive(Debug, Default)]
pub struct Router<'a> {
    routes: Vec<Route<'a>>,
    lookup: HashMap<(Method, &'static str), usize>,
}

impl<'a> Router<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn get<F, R>(self, path: &'static str, handler: F) -> Self
    where
        F: Fn(Request) -> R + 'a,
        R: IntoResponse,
    {
        self._add(Method::Get, path, Box::new(move |req| handler(req).into_response()))
    }

    #[inline]
    pub fn add<F, R>(self, method: Method, path: &'static str, handler: F) -> Self
    where
        F: Fn(Request) -> R + 'a,
        R: IntoResponse,
    {
        self._add(method, path, Box::new(move |req| handler(req).into_response()))
    }

    fn _add(mut self, method: Method, path: &'static str, handler: Box<dyn Fn(Request) -> Response + 'a>) -> Self {
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

    pub fn lookup_handler(&mut self, method: Method, path: &str) -> Option<&dyn Fn(Request) -> Response> {
        let &index = self.lookup.get(&(method, path))?;

        let route = self.routes.get(index)?;

        Some(&route.handler)
    }
}

pub struct Route<'a> {
    handler: Box<dyn Fn(Request) -> Response + 'a>,
    path: &'static str,
}

impl fmt::Debug for Route<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}
