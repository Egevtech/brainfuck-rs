use super::{OptToken, OptTokens};

pub trait Codegen {
    fn codegen(self) -> String;
}

impl Codegen for OptTokens {
    fn codegen(self) -> String {
        let mut loop_counter = 0u32;
        let mut code: String = String::from(
            r#"
#include "bfstdlib.h"

void bf_entry(vec_t *vec) {"#,
        );

        for token in self {
            code += "\n";
            code += &match token {
                OptToken::ParAdd(c) => format!("\tvector_add(vec, {});", c),
                OptToken::ParSub(c) => format!("\tvector_add(vec, -{});", c),

                OptToken::ParMoveFor(c) => format!("\tvector_move(vec, {});", c),
                OptToken::ParMoveBack(c) => format!("\tvector_move(vec, -{});", c),

                OptToken::ParDebug => String::from("\tvector_debug(vec);"),
                OptToken::ParPrint => String::from("\tvector_print(vec);"),

                OptToken::ParLoopStart => {
                    loop_counter += 1;
                    format!("\tloop{loop_counter}:")
                }
                OptToken::ParLoopEnd => {
                    let str = format!("\tif (vector_get(vec) != 0) goto loop{loop_counter};");
                    loop_counter -= 1;
                    str
                }
                OptToken::ParComma(_) => "".to_string(),
            };
            code += "\n";
        }

        code += "}";

        code
    }
}
