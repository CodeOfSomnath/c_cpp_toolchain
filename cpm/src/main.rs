/// This is main package manager binarary for distribution

// modules


fn make_parser() -> argparser::Parser {
    let mut parser = argparser::Parser::new(Some("[options] [values]"));

    parser.set_helpmessage("Shows help message");
    parser.add_string("new", "Create new project", "");
    parser.add_bool("lib", "Create new library project", false);
    parser.add_string("path", "Path for project", "");
    parser.parse();
    return parser;
}

fn main() {
    let mut parser = make_parser();
    
}
