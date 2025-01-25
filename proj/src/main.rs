
const SECONDS_IN_MINUTE: u32 = 60;

fn main() {
    const MINUTES_IN_HOUR: u32 = 60;
    const SECONDS_IN_HOUR: u32 = MINUTES_IN_HOUR * SECONDS_IN_MINUTE;

    // inner function, função interna
    // {
    //     let total_hours = 60;
    //     println!("Trabalhou {} horas", total_hours);
    // }

    let total_hours = 30;
    let total_em_segundos = total_hours * SECONDS_IN_HOUR;

    println!("Trabalhou {} segundos", total_em_segundos);
}
