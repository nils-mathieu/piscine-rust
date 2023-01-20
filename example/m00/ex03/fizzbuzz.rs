#[allow(dead_code)]
fn using_if(i: u32) {
    let multiple_of_three = i % 3 == 0;
    let multiple_of_five = i % 5 == 0;

    if multiple_of_three && multiple_of_five {
        println!("\tft_putstr(\"fizzbuzz\\n\");");
    } else if multiple_of_three {
        println!("\tft_putstr(\"fizz\\n\");");
    } else if multiple_of_five {
        println!("\tft_putstr(\"buzz\\n\");");
    } else {
        println!("\tft_putstr(\"{i}\\n\");");
    }
}

fn using_match(i: u32) {
    match (i % 3, i % 5) {
        (0, 0) => println!("\tft_putstr(\"fizzbuzz\\n\");"),
        (0, _) => println!("\tft_putstr(\"fizz\\n\");"),
        (_, 0) => println!("\tft_putstr(\"buzz\\n\");"),
        (_, _) => println!("\tft_putstr(\"{i}\\n\");"),
    }
}

fn main() {
    println!(
        "\
#include <unistd.h>

static void\tft_putstr(char const *s)
{{
\tsize_t\ti;

\ti = 0;
\twhile (s[i])
\t\t++i;
\twrite(STDOUT_FILENO, s, i);
}}

int\tmain(void)
{{
"
    );

    for i in 1..=100 {
        using_match(i);
        // using_if(i);
    }

    println!("}}");
}
