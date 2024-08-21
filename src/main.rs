use tree_sitter::{Node, Parser};
use walkdir::WalkDir;

fn print_deps(node: Node, source: &str, mut val: i32) {
    let kind = node.kind();
    let start_byte = node.start_byte();
    let end_byte = node.end_byte();
    let text = &source[start_byte..end_byte];

    // Print the identifier ie the things which are used in import
    if val > 0 && kind == "identifier" {
        print!("\n - {}", text);
    }

    for child in node.children(&mut node.walk()) {
        if child.kind() == "from" || child.kind() == "import" {
            /*  
            Since the control flow reached here means its printable so 
            change val to 1 and when function returns it resets back 
            to 0 as local scope ends and resets to global scope.
            */
            val = 1;
            print!(" \n\n {} ", child.kind());
            print_deps(child, source, val);
        } else {
            print_deps(child, source, val);
        }
    }
}

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_python::language())
        .expect("Error loading Python grammar");
    /*
    Enter the directory specified here its test
    Read each file and then use the print_deps function
    to find the import dependencies */
    for entry in WalkDir::new("test") {
        let path = entry.as_ref().unwrap().path();
        let source_code = std::fs::read_to_string(path).unwrap_or_default();
        let tree = parser.parse(&source_code, None).unwrap();
        let root_node = tree.root_node();
        print_deps(root_node, &source_code, 0);
    }
}
