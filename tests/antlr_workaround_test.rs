use antlr4rs::atn::ATN;
use antlr4rs::atn_type::ATNType;
use antlr4rs::common_token_stream::CommonTokenStream;
use antlr4rs::error_listener::ErrorListener;
use antlr4rs::errors::ANTLRError;
use antlr4rs::input_stream::StringStream;
use antlr4rs::lexer::BaseLexer;
use antlr4rs::lexer_atn_simulator::BaseLexerATNSimulator;
use antlr4rs::prediction_context::PredictionContextCache;
use antlr4rs::recognizer::{BaseRecognizer, Recognizer};
use antlr4rs::token_factory::CommonTokenFactory;

const RULE_NAMES: &'static [&'static str] = &["rule1", "rule2"];
const LITERAL_NAMES: &'static [&'static str] = &["literal1", "literal2"];
const SYMBOLIC_NAMES: &'static [&'static str] = &["symbolic1", "symbolic2"];

const GRAMMAR_FILE_NAME: &'static str = "grammar.g4";

struct MyANTLRErrorListener {}

impl ErrorListener for MyANTLRErrorListener {
    fn syntax_error(&mut self, _recognizer: &dyn Recognizer, _line: isize, _column: isize, _msg: &str, _err: Option<&ANTLRError>) {
        todo!()
    }
}

#[test]
fn test_antlr_workaround() {
    let recognizer = BaseRecognizer::new(
        RULE_NAMES,
        LITERAL_NAMES,
        SYMBOLIC_NAMES,
        GRAMMAR_FILE_NAME,
    );
    let lexer = BaseLexer::new(
        recognizer,
        BaseLexerATNSimulator::new(
            ATN::new(ATNType::Lexer, 0),
            PredictionContextCache::new(),
        ),
        CommonTokenFactory::new(),
        StringStream::from("this is char stream"),
    );
    let cts = CommonTokenStream::new(lexer, 0);
}