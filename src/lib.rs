// Single GET route which:
// - Returns a JSON response containing a random npm expansion if content-type header is set to JSON
// - Returns HTML if the content-type is set to HTML
// - Serves appropriate static files for SEO, robots and favicons
// - All other routes return an appropraite error response

// System should:
// - Handle incoming requests in an optimised way e.g. threads or asynchronous events

// Implementation plan
// - Implement Accept header pasrsing
// - Figure out error behaviour (rendering 500 page)
// - Render static files at static routes
// - Figure out how to structure tests in a more readable manner
// - Refactor all code and tests into a more organised structure
//   following best practices
// - Implement NPM expansions home page in figma
// - Implement 404 and 500 pages
// - Implement CICD with github actions
// - Implement husky with testing and formatting
// - 500 internal server error for any errors
// - Perhaps move each route code into a struct called "controller"?
// - Remember some issues could be client caused. If they parse incorrect headers thats 400 not 500

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
