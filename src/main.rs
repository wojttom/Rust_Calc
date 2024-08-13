use std::io;

fn parser() -> i32{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn ensure_non_0(mut b:i32) -> i32 {
    while b==0 {
            println!("Wartość a jest dzielona przez 0, prosze wybrać inną liczbę: ");
            b = parser();
    }   
    b
}

fn dzialania(a:i32, b:i32 ,x:i32) -> f32 {

    let result = match x{
        1=> (a+b) as f32,
        2=> (a-b) as f32,
        3=> (a*b) as f32,
        4=> (a/b) as f32,
        5=> (a as f32).powf(b as f32),
        6=> (a as f32).sqrt(),
        _ => {
            println!("Nieznana operacja");
            0.0
        }
    };
    return result;
}

fn main() {
    println!("Prosty kalkulator. Prosze wyybrać liczbe nr 1: ");
    let a = parser();
    println!("Prosze wybrać działanie: \n (1=dodawanie, 2=odejmowanie, 3=mnożenie, 4=dzielenie, 5=potęgowanie,6=pierwiastkowanie)");
    let x= parser();
    let b = if  x!=6{
        println!("Prosze wybrać liczbe nr 2: ");
        let initial_b = parser();
        if x==4 {
            ensure_non_0(initial_b)
        }else {
            initial_b
        }
    }else{
        0
    };
    let wynik = dzialania(a,b,x);
    println!("Wynik: {wynik}");
    
}
