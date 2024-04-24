
use tree_sitter::{Parser, Node};


fn print_tree(node: Node, source: &str, mut val:i32) {
    
    let kind = node.kind();
    let start_byte = node.start_byte();
    let end_byte = node.end_byte();
    let text = &source[start_byte..end_byte];
    
    
    if val>0 && kind == "identifier" {
        println!("{:?} ({:?}) '{}'", kind, start_byte, text);
    }
    for child in node.children(&mut node.walk()) {
        if child.kind() == "from" || child.kind() == "import"{
            val=1;
            print_tree(child, source,val);
        }
        else{
            print_tree(child, source,val);
        }
    } 

}

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_python::language())
        .expect("Error loading Python grammar");
 
    let source_code = std::fs::read_to_string("test.py").unwrap_or_default();
    let tree = parser.parse(&source_code, None).unwrap();
    let root_node = tree.root_node();

    print_tree(root_node, &source_code,0);

}

