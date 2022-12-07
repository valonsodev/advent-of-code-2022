use log::LevelFilter;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
// mod day_7;
mod utils;
fn main() {
    env_logger::Builder::new()
        .format_timestamp(None)
        .filter(None, LevelFilter::max())
        .init();

    // day_1::part1::main();
    // day_1::part2::main();
    // day_2::part1::main();
    // day_2::part2::main();
    // day_3::part1::main();
    // day_3::part2::main();
    // day_4::part1::main();
    // day_4::part2::main();
    // day_5::part1::main();
    // day_5::part2::main();
    day_6::part1::main();
    // day_6::part2::main();
    // day_7::part1::main();
    // day_7::part2::main();
    // day_8::part1::main();
    // day_8::part2::main();
    // day_9::part1::main();
    // day_9::part2::main();
}
