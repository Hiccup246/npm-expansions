// Single GET route which:
// - Returns a JSON response containing a random npm expansion if content-type header is set to JSON
// - Returns HTML if the content-type is set to HTML
// - Serves appropriate static files for SEO, robots and favicons
// - All other routes return an appropraite error response

// System should:
// - Handle incoming requests in an optimised way e.g. threads or asynchronous events

// Implementation plan
// - Tests first for functionality of JSON GET route
// - Start in main and split to library files when needed
// - When JSON route is completed move to HTML
// - Start single threaded then move to multi threaded (tracer rounds approach)
// - 500 internal server error for any errors
// - Perhaps move each route code into a struct called "controller"?

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
