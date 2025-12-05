test:
    cargo test

all:
    cargo run --release -- -1

run day:
    cargo run {{day}}

fast day:
    cargo run --release {{day}}

new_day_line := shell('grep -Fn "_ =>" ./src/main.rs | cut -f1 -d:')
next_day_line := shell('grep -Fn "// NEXT DAY" ./src/main.rs | cut -f1 -d:')

init day:
    mkdir src/day{{day}}
    cp -r src/dayN/* src/day{{day}}/
    sed -i '{{new_day_line}}i {{day}} => day{{day}}::run,' src/main.rs
    sed -i '{{next_day_line}}i mod day{{day}};' src/main.rs
    sed -iE 's/const MAX_DAY: i32 = [0-9]\+;/const MAX_DAY: i32 = {{day}};/g' src/main.rs

fix:
    cargo clippy --fix --allow-dirty