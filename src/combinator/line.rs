use parser_combinators:: { many1, digit, string, satisfy, optional, token, value, try, between, newline, parser, Parser, ParserExt, ParseResult };
use parser_combinators::primitives:: { State, Stream };
use record:: { LCOVRecord, Token };
use combinator::value:: { integer_value, string_value };

pub fn lines_record<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    try(parser(lines_hit::<I>))
        .or(parser(lines_found::<I>))
        .parse_state(input)
}

fn lines_hit<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let line_count = parser(integer_value::<I>)
        .map( | lines_hit | LCOVRecord::LinesHit(lines_hit) );

    between(string("LH:"), newline(), line_count).parse_state(input)
}

fn lines_found<I>(input: State<I>) -> ParseResult<LCOVRecord, I> where I: Stream<Item=char> {
    let line_found = parser(integer_value::<I>)
        .map( | line_found | LCOVRecord::LinesFound(line_found) );

    between(string("LF:"), newline(), line_found).parse_state(input)
}
