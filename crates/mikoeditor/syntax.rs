use tree_sitter::{Parser, Tree};

pub use tree_sitter::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Keyword,
    Function,
    Type,
    String,
    Number,
    Comment,
    Operator,
    Punctuation,
    Variable,
    Property,
    Parameter,
    Constant,
    Text,
}

pub struct SyntaxHighlighter {
    parser: Parser,
    tree: Option<Tree>,
    language: Option<Language>,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            tree: None,
            language: None,
        }
    }
    
    pub fn set_language(&mut self, lang_name: &str) -> Result<(), String> {
        let language = match lang_name {
            "rust" => tree_sitter_rust::language(),
            "javascript" => tree_sitter_javascript::language(),
            "typescript" => tree_sitter_typescript::language_typescript(),
            "tsx" => tree_sitter_typescript::language_tsx(),
            "python" => tree_sitter_python::language(),
            "json" => tree_sitter_json::language(),
            _ => return Err(format!("Unsupported language: {}", lang_name)),
        };
        
        self.parser
            .set_language(language)
            .map_err(|e| format!("Failed to set language: {:?}", e))?;
        self.language = Some(language);
        Ok(())
    }
    
    pub fn parse(&mut self, source_code: &str) {
        self.tree = self.parser.parse(source_code, None);
    }
    
    pub fn get_highlights(&self, source_code: &str) -> Vec<(usize, usize, TokenType)> {
        let mut highlights = Vec::new();
        
        if let Some(ref tree) = self.tree {
            let root_node = tree.root_node();
            self.traverse_node(root_node, source_code, &mut highlights);
        }
        
        highlights
    }
    
    fn traverse_node(
        &self,
        node: tree_sitter::Node,
        source_code: &str,
        highlights: &mut Vec<(usize, usize, TokenType)>,
    ) {
        let kind = node.kind();
        let start = node.start_byte();
        let end = node.end_byte();
        
        let token_type = self.classify_node(kind);
        
        if token_type != TokenType::Text && !node.is_named() {
            highlights.push((start, end, token_type));
        }
        
        // Traverse children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.traverse_node(child, source_code, highlights);
        }
    }
    
    fn classify_node(&self, kind: &str) -> TokenType {
        match kind {
            // Keywords - Rust
            "fn" | "let" | "mut" | "const" | "if" | "else" | "for" | "while" | "loop" |
            "match" | "return" | "break" | "continue" | "pub" | "use" | "mod" | "struct" |
            "enum" | "trait" | "impl" | "type" | "where" | "async" | "await" | "move" |
            "static" | "ref" | "self" | "super" | "crate" | "unsafe" | "extern" | "in" |
            
            // Keywords - JavaScript/TypeScript
            "function" | "var" | "class" | "import" | "export" | "from" | "as" |
            "new" | "this" | "typeof" | "instanceof" | "void" | "delete" |
            "interface" | "namespace" | "declare" | "abstract" | "extends" | "implements" |
            
            // Keywords - Python
            "def" | "lambda" | "pass" | "raise" | "try" | "except" | "finally" |
            "with" | "yield" | "assert" | "global" | "nonlocal" | "is" | "not" | "and" | "or" |
            "elif" | "print" |
            
            // Keywords - C/C++
            "sizeof" | "typedef" | "union" | "volatile" | "register" | "goto" |
            "switch" | "case" | "default" |
            
            // Keywords - Java
            "package" | "throws" | "throw" | "catch" | "synchronized" | "native" |
            "transient" | "volatile" | "strictfp" |
            
            // Keywords - Go
            "func" | "package" | "defer" | "go" | "chan" | "select" | "fallthrough" |
            
            // Common keywords
            "do" | "then" | "end" | "begin" => {
                TokenType::Keyword
            }
            
            // Types
            "type_identifier" | "primitive_type" | "type" | "type_annotation" |
            "predefined_type" | "class_name" | "interface_name" => TokenType::Type,
            
            // Functions
            "function_item" | "function_declaration" | "function_definition" |
            "call_expression" | "method_declaration" | "method_definition" |
            "function_name" | "method_name" => TokenType::Function,
            
            // Strings
            "string_literal" | "string" | "raw_string_literal" | "char_literal" |
            "string_content" | "template_string" | "template_literal" => {
                TokenType::String
            }
            
            // Numbers
            "integer_literal" | "float_literal" | "number" | "numeric_literal" |
            "decimal_integer_literal" | "hex_integer_literal" | "binary_integer_literal" => {
                TokenType::Number
            }
            
            // Comments
            "line_comment" | "block_comment" | "comment" | "documentation_comment" |
            "doc_comment" => TokenType::Comment,
            
            // Operators
            "+" | "-" | "*" | "/" | "%" | "=" | "==" | "!=" | "<" | ">" | "<=" | ">=" |
            "&&" | "||" | "!" | "&" | "|" | "^" | "<<" | ">>" | "+=" | "-=" | "*=" | "/=" |
            "**" | "===" | "!==" | "??" | "?." | "..." | "=>" | "->" | "::" |
            "binary_operator" | "unary_operator" | "assignment_operator" => {
                TokenType::Operator
            }
            
            // Punctuation
            ";" | "," | "." | ":" | "{" | "}" | "[" | "]" | "(" | ")" |
            "punctuation" | "delimiter" => {
                TokenType::Punctuation
            }
            
            // Variables and identifiers
            "identifier" | "variable_name" => TokenType::Variable,
            "field_identifier" | "property_identifier" | "member_expression" => TokenType::Property,
            "parameter" | "parameter_declaration" => TokenType::Parameter,
            
            // Constants
            "boolean_literal" | "true" | "false" | "null" | "None" | "True" | "False" |
            "nil" | "undefined" | "NULL" | "constant" | "const_identifier" => {
                TokenType::Constant
            }
            
            _ => TokenType::Text,
        }
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}
